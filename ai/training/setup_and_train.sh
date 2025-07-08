#!/bin/bash
set -e

# install git, docker, aws cli
bash ~/install_git.sh || bash ../../scripts/install_git.sh
bash ~/install_docker.sh || bash ../../scripts/install_docker.sh
bash ~/install_aws_cli.sh || bash ../../scripts/install_aws_cli.sh

# git repo url
REPO_URL=${REPO_URL:-git@github.com:whalehole/project-lunii.git}

# create the folder for residing codebases
mkdir -p ~/projects
cd ~/projects || exit
# git clone the repo's metadata and history first
git clone --filter=blob:none --no-checkout --depth 1 "$REPO_URL"
cd project-lunii || exit
# then git clone the desired subfolder
git sparse-checkout init --cone
git sparse-checkout set ai # 'ai' is the subfolder
git checkout

# build docker image
docker build -t lunii_ai ai/

# download model
docker run lunii_ai python ai/models/download_llm.py || { echo "Error encountered when downloading LLM model"; exit 1; }