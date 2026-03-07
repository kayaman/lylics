# ── Production Environment ────────────────────────────────────

aws_region  = "eu-north-1"
app_name    = "lylics"
environment = "production"

# DNS / TLS (Route53 zone must already exist)
domain_zone = "magj.dev"

# Container
container_port = 3000
image_tag      = "latest"

# ECS — smallest Fargate config, Spot pricing
cpu            = 256   # 0.25 vCPU
memory         = 512   # 0.5 GB
desired_count  = 1

# Observability
log_retention_days = 7

# GitHub
github_repo = "kayaman/lylics"
