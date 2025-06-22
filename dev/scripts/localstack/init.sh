#!/bin/bash
set -e

echo "Initializing LocalStack resources..."

# Create an S3 bucket
awslocal s3api create-bucket --bucket elfera-assets