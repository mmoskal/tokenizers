topmodels='
EleutherAI/gpt-j-6b
EleutherAI/gpt-neox-20b
EleutherAI/pythia-6.9b
Gustavosta/MagicPrompt-Stable-Diffusion
NousResearch/Llama-2-7b-chat-hf
NousResearch/Llama-2-7b-hf
Riiid/sheep-duck-llama-2
TheBloke/Llama-2-13B-chat-GPTQ
TheBloke/Llama-2-7b-Chat-GPTQ
TheBloke/MythoMax-L2-13B-GPTQ
TheBloke/Wizard-Vicuna-7B-Uncensored-GPTQ
TheBloke/vicuna-7B-v1.3-GPTQ
WizardLM/WizardCoder-Python-34B-V1.0
Xenova/gpt-3.5-turbo-16k
Xenova/gpt-3.5-turbo
Xenova/gpt-3
Xenova/gpt-4
bigscience/bloom-560m
bigscience/bloom
bigscience/bloomz-1b1
codellama/CodeLlama-34b-Instruct-hf
codellama/CodeLlama-7b-hf
databricks/dolly-v2-3b
distilgpt2
fxmarty/tiny-llama-fast-tokenizer
gpt2-large
gpt2-medium
gpt2-xl
gpt2
huggyllama/llama-7b
microsoft/phi-1_5
mosaicml/mpt-7b-instruct
mosaicml/mpt-7b
nferruz/ProtGPT2
petals-team/StableBeluga2
smallcloudai/Refact-1_6B-fim
stabilityai/StableBeluga-7B
tiiuae/falcon-40b-instruct
tiiuae/falcon-7b-instruct
tiiuae/falcon-7b
xlnet-base-cased
'

selmodels='
Xenova/gpt-4:gpt4
codellama/CodeLlama-34b-Instruct-hf:llama
gpt2:gpt2
microsoft/phi-1_5:phi
mosaicml/mpt-7b:mpt
tiiuae/falcon-7b:falcon
mistralai/Mistral-7B-Instruct-v0.2:mistral
  '

#rm -rf toks
mkdir toks
for m in $selmodels ; do
  ./target/debug/cli binary $(echo $m | sed -e 's/:.*//')
  mm=$(echo $m | sed -e 's@.*:@@g')
  mv toks.json toks/$mm.json
done

perl -p -i -e 's/\s*("[^"]*":) (\d+),$/$1$2,/' toks/*.json
