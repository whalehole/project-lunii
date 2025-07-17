#!/bin/bash
set -e

set -a
source ../.env || { echo ".env file is missing"; exit 1; }
set +a

if [ -z "$SSH_KEY_PATH" ] || \
   [ -z "$SSH_USER" ] || \
   [ -z "$SSH_HOST" ] || \
   [ -z "$HUGGINGFACE_TOKEN" ] || \
   [ -z "$AWS_ACCESS_KEY_ID" ] || \
   [ -z "$AWS_SECRET_ACCESS_KEY" ] || \
   [ -z "$AWS_DEFAULT_REGION" ] || \
   [ -z "$GITHUB_SSH_KEY_PATH" ]; then
  echo "1 or more environment variables are missing"
  exit 1
fi

chmod 600 "$SSH_KEY_PATH"
chmod 600 "$GITHUB_SSH_KEY_PATH"

# start ssh agent
echo "starting ssh agent"
eval "$(ssh-agent -s)"
touch ~/.ssh/config

# save ssh private keys to ssh agent
echo "adding ssh private keys to ssh agent"
ssh-add "$GITHUB_SSH_KEY_PATH"
ssh-add -l || { echo "No identities loaded in ssh-agent!"; exit 1; }

# allow ssh agent forwarding on target server
echo "copying allow_ssh_agent_forwarding.sh onto target server"
scp -i "$SSH_KEY_PATH" ../../scripts/allow_ssh_agent_forwarding.sh \
    "$SSH_USER@$SSH_HOST":~/ \
    || { echo "failed to upload script to allow ssh agent forwarding"; exit 1; }

echo "running allow_ssh_agent_forwarding.sh script on target server"
ssh -i "$SSH_KEY_PATH" "$SSH_USER@$SSH_HOST" 'bash ~/allow_ssh_agent_forwarding.sh' \
    || { echo "failed to run allow_ssh_agent_forwarding.sh"; exit 1; }

# copy the training .sh script to target server
scp -i "$SSH_KEY_PATH" \
    ../../scripts/install_docker.sh \
    ../../scripts/install_git.sh \
    ../../scripts/install_aws_cli.sh \
    setup_and_train.sh \
    "$SSH_USER@$SSH_HOST":~/ \
    || { echo "failed to upload necessary files"; exit 1; }

# run the training .sh script inside the target server
ssh -A -i "$SSH_KEY_PATH" "$SSH_USER@$SSH_HOST" \
    SSH_USER="$SSH_USER" \
    SSH_HOST="$SSH_HOST" \
    HUGGINGFACE_TOKEN="$HUGGINGFACE_TOKEN" \
    AWS_ACCESS_KEY_ID="$AWS_ACCESS_KEY_ID" \
    AWS_SECRET_ACCESS_KEY="$AWS_SECRET_ACCESS_KEY" \
    AWS_DEFAULT_REGION="$AWS_DEFAULT_REGION" \
    'bash ~/setup_and_train.sh' \
    || { echo "failed to run setup_and_train.sh"; exit 1; }