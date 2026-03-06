#!/usr/bin/env bash
set -euo pipefail

# Configure GitHub Actions secrets and repository variables
# from Terraform outputs. Requires: gh CLI authenticated, terraform state.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TERRAFORM_DIR="${PROJECT_ROOT}/terraform"

echo "==> Checking prerequisites..."
command -v gh >/dev/null 2>&1 || { echo "ERROR: gh CLI is not installed. See https://cli.github.com/"; exit 1; }
command -v terraform >/dev/null 2>&1 || { echo "ERROR: terraform is not installed"; exit 1; }
gh auth status >/dev/null 2>&1 || { echo "ERROR: gh CLI not authenticated. Run: gh auth login"; exit 1; }

cd "$TERRAFORM_DIR"

# Ensure state is initialized
terraform init -input=false >/dev/null 2>&1

# Extract outputs
echo "==> Reading Terraform outputs..."
ROLE_ARN=$(terraform output -raw github_actions_role_arn)
ECS_CLUSTER=$(terraform output -raw ecs_cluster_name)
ECS_SERVICE=$(terraform output -raw ecs_service_name)

# Read region from tfvars
AWS_REGION=$(grep -oP 'aws_region\s*=\s*"\K[^"]+' terraform.tfvars)

echo ""
echo "==> Setting GitHub Actions secrets..."
cd "$PROJECT_ROOT"
gh secret set AWS_ROLE_ARN --body "$ROLE_ARN"
echo "   AWS_ROLE_ARN = ${ROLE_ARN:0:20}...${ROLE_ARN: -10}"

echo ""
echo "==> Setting GitHub Actions variables..."
gh variable set AWS_REGION --body "$AWS_REGION"
echo "   AWS_REGION   = $AWS_REGION"

gh variable set ECS_CLUSTER --body "$ECS_CLUSTER"
echo "   ECS_CLUSTER  = $ECS_CLUSTER"

gh variable set ECS_SERVICE --body "$ECS_SERVICE"
echo "   ECS_SERVICE  = $ECS_SERVICE"

echo ""
echo "==> GitHub Actions configured."
echo ""
echo "Verify:"
echo "  gh secret list"
echo "  gh variable list"
