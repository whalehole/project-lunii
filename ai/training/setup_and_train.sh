#!/bin/bash
set -e

# install git, docker, aws cli
echo "installing git"
bash ~/install_git.sh || { echo "failed to install git"; exit 1; }
echo "installing docker"
bash ~/install_docker.sh "$SSH_USER" || { echo "failed to install docker"; exit 1; }
echo "installing aws cli"
bash ~/install_aws_cli.sh || { echo "failed to install aws cli"; exit 1; }

# register github account to remote server ssh known hosts
ssh-keyscan github.com >> ~/.ssh/known_hosts

# git repo url and repo
REPO_URL=${REPO_URL:-git@github.com:whalehole/project-lunii.git}
REPO=project-lunii

# create the folder for residing codebases
mkdir -p ~/projects
cd ~/projects

# git clone the repo's metadata and history first
rm -rf "$REPO" && git clone --filter=blob:none --no-checkout --depth 1 "$REPO_URL"
cd project-lunii

# then git clone the desired subfolder
git sparse-checkout init --cone
git sparse-checkout set ai # 'ai' is the subfolder
git checkout

# build docker image
docker build -t lunii_ai:dev ai/

# download model
#docker run lunii_ai python ai/models/download_llm.py || { echo "Error encountered when downloading LLM model"; exit 1; }