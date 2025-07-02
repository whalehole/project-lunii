#!/bin/bash
set -ex
ls -l /assets
ls -l /assets/carlotta.glb

echo "Initializing LocalStack resources..."

# Create an S3 bucket
awslocal s3api create-bucket --bucket elfera-assets || true
echo "Created bucket 'elfera-assets'"

# Configure CORS for bucket
awslocal s3api put-bucket-cors --bucket elfera-assets --cors-configuration file:///configs/cors.json
echo "Configured CORS for Bucket 'elfera-assets'..."

# Putting a 3D model object into the bucket
awslocal s3api put-object --bucket elfera-assets --key 3d_models/carlotta.glb --body /assets/carlotta.glb
echo "Inserted 3D model object into 'elfera-assets' bucket..."