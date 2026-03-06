#!/usr/bin/env bash
set -euo pipefail

# Destroy AWS infrastructure.
# Usage:
#   ./scripts/infra-down.sh                    # destroy main infra only
#   ./scripts/infra-down.sh --include-bootstrap # also destroy state backend

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TERRAFORM_DIR="${PROJECT_ROOT}/terraform"
BOOTSTRAP_DIR="${PROJECT_ROOT}/terraform/bootstrap"

INCLUDE_BOOTSTRAP=false
for arg in "$@"; do
  case "$arg" in
    --include-bootstrap) INCLUDE_BOOTSTRAP=true ;;
    *) echo "Unknown argument: $arg"; exit 1 ;;
  esac
done

echo "==> Checking prerequisites..."
command -v terraform >/dev/null 2>&1 || { echo "ERROR: terraform is not installed"; exit 1; }
command -v aws >/dev/null 2>&1 || { echo "ERROR: aws CLI is not installed"; exit 1; }
aws sts get-caller-identity >/dev/null 2>&1 || { echo "ERROR: AWS credentials not configured"; exit 1; }

echo ""
echo "WARNING: This will destroy all AWS infrastructure for lylics."
echo "         Resources: ALB, ECS cluster/service, Route53 record, ACM cert, IAM roles, etc."
echo ""
read -rp "Type 'destroy' to confirm: " confirm
[ "$confirm" = "destroy" ] || { echo "Aborted."; exit 1; }

echo ""
echo "==> Destroying main infrastructure..."
cd "$TERRAFORM_DIR"
terraform init -input=false
terraform destroy -auto-approve -input=false

if [ "$INCLUDE_BOOTSTRAP" = true ]; then
  echo ""
  echo "WARNING: This will also destroy the Terraform state backend (S3 bucket + DynamoDB table)."
  echo "         This is irreversible — all state history will be lost."
  echo ""
  read -rp "Type 'destroy-bootstrap' to confirm: " confirm2
  [ "$confirm2" = "destroy-bootstrap" ] || { echo "Skipping bootstrap teardown."; exit 0; }

  echo ""
  echo "==> Destroying state backend..."
  cd "$BOOTSTRAP_DIR"
  terraform init -input=false
  terraform destroy -auto-approve -input=false
fi

echo ""
echo "==> Teardown complete."
