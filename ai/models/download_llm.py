import os
from huggingface_hub import snapshot_download
from pathlib import Path

mistral_models_path_str = os.getenv("MISTRAL_MODELS_PATH")
if not mistral_models_path_str:
    print("ERROR: The environment variable MISTRAL_MODELS_PATH must be set.")
    exit(1)

huggingface_token_str = os.getenv("HUGGINGFACE_TOKEN")
if not huggingface_token_str:
    print("ERROR: The environment variable HUGGINGFACE_TOKEN must be set.")
    exit(1)

mistral_models_path = Path(mistral_models_path_str)
mistral_models_path.mkdir(parents=True, exist_ok=True)

snapshot_download(
    repo_id="mistralai/Mistral-7B-Instruct-v0.3",
    allow_patterns=["params.json", "consolidated.safetensors", "tokenizer.model.v3"],
    local_dir=mistral_models_path,
    token=huggingface_token_str
)