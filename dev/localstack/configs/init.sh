#!/bin/bash
set -ex
ls -l /assets
ls -l /assets/carlotta.v0.glb

echo "Initializing LocalStack resources..."

# Create an S3 bucket
awslocal s3api create-bucket --bucket elfera-assets --create-bucket-configuration LocationConstraint=us-west-2 || true
echo "Created bucket 'elfera-assets'"

# Configure CORS for bucket
awslocal s3api put-bucket-cors --bucket elfera-assets --cors-configuration file:///configs/cors.json
echo "Configured CORS for Bucket 'elfera-assets'..."

# Putting a 3D model object into the bucket
# awslocal s3api put-object --bucket elfera-assets --key 3d_models/carlotta.v0.glb --body /assets/carlotta.v0.glb --cache-control "public, max-age=31536000, immutable" --content-type "model/gltf-binary" --acl public-read
awslocal s3api put-object --bucket elfera-assets --key 3d_models/carlotta.v0.glb --body /assets/carlotta.v0.glb 
awslocal s3api put-object --bucket elfera-assets --key 3d_models/carlotta.v0.glb.manifest --body /assets/carlotta.v0.glb.manifest 
echo "Inserted 3D model object into 'elfera-assets' bucket..."