#!/usr/bin/env bash
set -euo pipefail

# Provision (or update) AWS infrastructure via Terraform.
# Usage:
#   ./scripts/infra-up.sh              # plan + apply
#   ./scripts/infra-up.sh --plan-only  # plan only

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TERRAFORM_DIR="${PROJECT_ROOT}/terraform"

PLAN_ONLY=false
for arg in "$@"; do
  case "$arg" in
    --plan-only) PLAN_ONLY=true ;;
    *) echo "Unknown argument: $arg"; exit 1 ;;
  esac
done

echo "==> Checking prerequisites..."
command -v terraform >/dev/null 2>&1 || { echo "ERROR: terraform is not installed"; exit 1; }
command -v aws >/dev/null 2>&1 || { echo "ERROR: aws CLI is not installed"; exit 1; }
aws sts get-caller-identity >/dev/null 2>&1 || { echo "ERROR: AWS credentials not configured"; exit 1; }

echo "==> Initializing Terraform..."
cd "$TERRAFORM_DIR"
terraform init -input=false

echo "==> Planning infrastructure..."
terraform plan -out=tfplan -input=false

if [ "$PLAN_ONLY" = true ]; then
  echo ""
  echo "==> Plan saved to terraform/tfplan. Run without --plan-only to apply."
  exit 0
fi

echo "==> Applying infrastructure..."
terraform apply -input=false tfplan
rm -f tfplan

echo ""
echo "==> Infrastructure provisioned. Outputs:"
echo ""
terraform output
