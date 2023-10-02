//!
//! This is the CLI binary for the Tokenizers project
//!

use clap::{Parser, Subcommand};
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use tokenizers::models::bpe::BPE;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::{AddedToken, Result};
use tokenizers::Tokenizer;

/// Generate custom Tokenizers or use existing ones
#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Shell {
        /// Path to the vocab.json file
        vocab: String,
        /// Path to the merges.txt file
        merges: String,
    },
    Binary {
        /// HuggingFace model identifier
        hf_id: String,
    },
}

fn hex_encode(input: &[u8]) -> String {
    input.iter().map(|b| format!("{:02x}", b)).collect()
}

#[derive(Serialize)]
struct TokenInfo {
    eos_token: u32,
    special: BTreeMap<String, u32>,
    binary: BTreeMap<String, u32>,
    text: BTreeMap<String, u32>,
}

fn binary(hf_id: &str) -> Result<()> {
    let tokenizer = Tokenizer::from_pretrained(hf_id, None)?;
    let vsize = tokenizer.get_vocab_size(true);
    println!(
        "Loaded tokenizer size={} added={}",
        vsize,
        vsize - tokenizer.get_vocab_size(false)
    );
    let mut info = TokenInfo {
        eos_token: 0xffffffff,
        special: BTreeMap::new(),
        binary: BTreeMap::new(),
        text: BTreeMap::new(),
    };
    for tok in 0..(vsize as u32) {
        let b0 = tokenizer.decode_as_bytes(&vec![tok as u32], true)?;
        let s = String::from_utf8(b0.clone());
        if b0.len() == 0 {
            let special = tokenizer.decode(&vec![tok as u32], false)?;
            if special.len() > 0 {
                info.special.insert(special, tok);
            }
        } else if s.is_err() {
            info.binary.insert(hex_encode(&b0), tok);
        } else {
            info.text.insert(s.unwrap(), tok);
        }
    }
    for tok_id in vec!["</s>", "<|endoftext|>"] {
        if let Some(id) = info.special.get(tok_id) {
            info.eos_token = *id;
        }
    }
    std::fs::write("toks.json", serde_json::to_string_pretty(&info)?)?;
    Ok(())
}

fn shell(vocab: &str, merges: &str) -> Result<()> {
    let bpe = BPE::from_file(vocab, merges).build()?;
    let mut tokenizer = Tokenizer::new(bpe);
    tokenizer
        .with_pre_tokenizer(ByteLevel::default())
        .with_decoder(ByteLevel::default());

    tokenizer.add_tokens(&[AddedToken::from(String::from("ing"), false).single_word(false)]);
    tokenizer
        .add_special_tokens(&[AddedToken::from(String::from("[ENT]"), true).single_word(true)]);

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();

        print!("\nEnter some text to tokenize:\n>  ");
        io::stdout().flush()?;
        handle.read_line(&mut buffer)?;
        let buffer = buffer.trim_end();

        let timer = std::time::Instant::now();
        let encoded = tokenizer.encode(buffer.to_owned(), false)?;
        let elapsed = timer.elapsed();
        println!("\nInput:\t\t{}", buffer);
        println!("Tokens:\t\t{:?}", encoded.get_tokens());
        println!("IDs:\t\t{:?}", encoded.get_ids());
        println!("Offsets:\t{:?}", encoded.get_offsets());
        println!(
            "Decoded:\t{}",
            tokenizer.decode(encoded.get_ids(), true).unwrap()
        );
        println!("Tokenized in {:?}", elapsed);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Shell { vocab, merges } => shell(&vocab, &merges),
        Command::Binary { hf_id } => binary(&hf_id),
    }
}
