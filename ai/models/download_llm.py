import os
from huggingface_hub import snapshot_download
from pathlib import Path

mistral_model_path_str = os.getenv("LLM_MODEL_PATH")
if not mistral_model_path_str:
    print("ERROR: The environment variable LLM_MODEL_PATH must be set.")
    exit(1)

huggingface_token = os.getenv("HUGGINGFACE_TOKEN")
if not huggingface_token:
    print("ERROR: The environment variable HUGGINGFACE_TOKEN must be set.")
    exit(1)

mistral_model_path = Path(mistral_model_path_str)
mistral_model_path.mkdir(parents=True, exist_ok=True)

snapshot_download(
    repo_id="mistralai/Mistral-7B-Instruct-v0.3",
    allow_patterns=["params.json", "consolidated.safetensors", "tokenizer.model.v3"],
    local_dir=mistral_model_path,
    token=huggingface_token
)