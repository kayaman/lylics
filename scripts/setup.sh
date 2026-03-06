#!/usr/bin/env bash
set -euo pipefail

# Full infrastructure setup: bootstrap → provision → configure GitHub Actions.
# This is the main entry point for first-time setup.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "╔══════════════════════════════════════════════════╗"
echo "║       lylics — Full Infrastructure Setup         ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""

# ── Check all prerequisites upfront ──────────────────────────

echo "==> Checking prerequisites..."

MISSING=()
command -v terraform >/dev/null 2>&1 || MISSING+=("terraform")
command -v aws >/dev/null 2>&1 || MISSING+=("aws")
command -v gh >/dev/null 2>&1 || MISSING+=("gh")

if [ ${#MISSING[@]} -gt 0 ]; then
  echo "ERROR: Missing required tools: ${MISSING[*]}"
  echo ""
  echo "Install them:"
  echo "  terraform  → https://developer.hashicorp.com/terraform/install"
  echo "  aws        → https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html"
  echo "  gh         → https://cli.github.com/"
  exit 1
fi

aws sts get-caller-identity >/dev/null 2>&1 || { echo "ERROR: AWS credentials not configured. Run: aws configure"; exit 1; }
gh auth status >/dev/null 2>&1 || { echo "ERROR: gh CLI not authenticated. Run: gh auth login"; exit 1; }

CALLER=$(aws sts get-caller-identity --query 'Arn' --output text)
echo "   AWS identity: ${CALLER}"
echo "   All prerequisites met."
echo ""

# ── Step 1: Bootstrap State Backend ──────────────────────────

echo "┌──────────────────────────────────────────────────┐"
echo "│  Step 1/3: Bootstrap State Backend                │"
echo "└──────────────────────────────────────────────────┘"
echo ""
"$SCRIPT_DIR/bootstrap.sh"
echo ""

# ── Step 2: Provision Infrastructure ─────────────────────────

echo "┌──────────────────────────────────────────────────┐"
echo "│  Step 2/3: Provision Infrastructure               │"
echo "└──────────────────────────────────────────────────┘"
echo ""
"$SCRIPT_DIR/infra-up.sh"
echo ""

# ── Step 3: Configure GitHub Actions ─────────────────────────

echo "┌──────────────────────────────────────────────────────┐"
echo "│  Step 3/3: Configure GitHub Actions Secrets & Vars    │"
echo "└──────────────────────────────────────────────────────┘"
echo ""
"$SCRIPT_DIR/setup-github.sh"
echo ""

# ── Done ─────────────────────────────────────────────────────

echo "╔══════════════════════════════════════════════════╗"
echo "║              Setup Complete!                      ║"
echo "╚══════════════════════════════════════════════════╝"
echo ""
echo "Your service will be available at: https://lylics.magj.dev"
echo ""
echo "Next steps:"
echo "  1. Push to main or create a v* tag to trigger a deploy"
echo "  2. Monitor: gh run list --workflow=CI"
echo "  3. Verify:  curl https://lylics.magj.dev/healthz"
