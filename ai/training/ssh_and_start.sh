#!/bin/bash
set -e

set -a
source ../.env
set +a

if [ -z "$SSH_KEY_PATH" ] || \
   [ -z "$SSH_LOGIN" ] || \
   [ -z "$HUGGINGFACE_TOKEN" ] || \
   [ -z "$AWS_ACCESS_KEY_ID" ] || \
   [ -z "$AWS_SECRET_ACCESS_KEY" ]; then
  echo "1 or more environment variables are missing"
  exit 1
fi

chmod 600 "$SSH_KEY_PATH"

# copy the training .sh script to target server
scp -i "$SSH_KEY_PATH" setup_and_train.sh "$SSH_LOGIN":~/ || { echo "failed to upload setup_and_train.sh file"; exit 1; }

# run the training .sh script inside the target server
ssh -i "$SSH_KEY_PATH" "$SSH_LOGIN" \
HUGGINGFACE_TOKEN="$HUGGINGFACE_TOKEN" \
AWS_ACCESS_KEY_ID="$AWS_ACCESS_KEY_ID" \
AWS_SECRET_ACCESS_KEY="$AWS_SECRET_ACCESS_KEY" \
bash ~/setup_and_train.sh || \
{ echo "failed to run setup_and_train.sh"; exit 1; }