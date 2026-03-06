# ── Variables ─────────────────────────────────────────────────

variable "aws_region" {
  description = "AWS region for all resources"
  type        = string
  default     = "eu-north-1"
}

variable "app_name" {
  description = "Application name (used for resource naming)"
  type        = string
  default     = "lylics"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

# ── DNS / TLS ─────────────────────────────────────────────────

variable "domain_zone" {
  description = "Route53 hosted zone name (must already exist)"
  type        = string
  default     = "magj.dev"
}

# ── Container ─────────────────────────────────────────────────

variable "container_port" {
  description = "Port the container listens on"
  type        = number
  default     = 3000
}

variable "image_tag" {
  description = "Container image tag to deploy"
  type        = string
  default     = "latest"
}

# ── ECS ───────────────────────────────────────────────────────

variable "cpu" {
  description = "Fargate task CPU units (256 = 0.25 vCPU)"
  type        = number
  default     = 256
}

variable "memory" {
  description = "Fargate task memory in MiB"
  type        = number
  default     = 512
}

variable "desired_count" {
  description = "Number of running ECS tasks"
  type        = number
  default     = 1
}

variable "log_retention_days" {
  description = "CloudWatch log retention in days"
  type        = number
  default     = 7
}

# ── GitHub ────────────────────────────────────────────────────

variable "github_repo" {
  description = "GitHub repository in owner/repo format"
  type        = string
  default     = "kayaman/lylics"
}
