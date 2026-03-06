#!/usr/bin/env bash
set -euo pipefail

# Bootstrap Terraform state backend (S3 + DynamoDB).
# Run once before provisioning infrastructure.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BOOTSTRAP_DIR="${PROJECT_ROOT}/terraform/bootstrap"

echo "==> Checking prerequisites..."
command -v terraform >/dev/null 2>&1 || { echo "ERROR: terraform is not installed. See https://developer.hashicorp.com/terraform/install"; exit 1; }
command -v aws >/dev/null 2>&1 || { echo "ERROR: aws CLI is not installed. See https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html"; exit 1; }
aws sts get-caller-identity >/dev/null 2>&1 || { echo "ERROR: AWS credentials not configured. Run: aws configure"; exit 1; }

CALLER=$(aws sts get-caller-identity --query 'Arn' --output text)
echo "   AWS identity: ${CALLER}"

echo "==> Bootstrapping Terraform state backend..."
cd "$BOOTSTRAP_DIR"
terraform init -input=false
terraform apply -auto-approve -input=false

echo ""
echo "==> State backend ready."
echo "   S3 bucket:      $(terraform output -raw state_bucket_name 2>/dev/null || echo 'lylics-terraform-state')"
echo "   DynamoDB table:  $(terraform output -raw lock_table_name 2>/dev/null || echo 'lylics-terraform-locks')"
