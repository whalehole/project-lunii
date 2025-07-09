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
   [ -z "$AWS_DEFAULT_REGION" ]; then
  echo "1 or more environment variables are missing"
  exit 1
fi

chmod 600 "$SSH_KEY_PATH"

# copy the training .sh script to target server
scp -i "$SSH_KEY_PATH" \
    ../../scripts/install_docker.sh \
    ../../scripts/install_git.sh \
    ../../scripts/install_aws_cli.sh \
    setup_and_train.sh \
    "$SSH_USER@$SSH_HOST":~/ \
    || { echo "failed to upload necessary files"; exit 1; }

# run the training .sh script inside the target server
ssh -i "$SSH_KEY_PATH" "$SSH_USER@$SSH_HOST" \
    SSH_USER="$SSH_USER" \
    SSH_HOST="$SSH_HOST" \
    HUGGINGFACE_TOKEN="$HUGGINGFACE_TOKEN" \
    AWS_ACCESS_KEY_ID="$AWS_ACCESS_KEY_ID" \
    AWS_SECRET_ACCESS_KEY="$AWS_SECRET_ACCESS_KEY" \
    AWS_DEFAULT_REGION="$AWS_DEFAULT_REGION" \
    'bash ~/setup_and_train.sh' \
    || { echo "failed to run setup_and_train.sh"; exit 1; }