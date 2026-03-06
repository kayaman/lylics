# ── Outputs ───────────────────────────────────────────────────

output "app_url" {
  description = "Application URL"
  value       = "https://${var.app_name}.${var.domain_zone}"
}

output "alb_dns_name" {
  description = "ALB DNS name"
  value       = aws_lb.main.dns_name
}

output "ghcr_image" {
  description = "GHCR image URI"
  value       = "ghcr.io/${var.github_repo}"
}

output "ecs_cluster_name" {
  description = "ECS cluster name"
  value       = aws_ecs_cluster.main.name
}

output "ecs_service_name" {
  description = "ECS service name"
  value       = aws_ecs_service.main.name
}

output "github_actions_role_arn" {
  description = "IAM role ARN for GitHub Actions (set as repo secret AWS_ROLE_ARN)"
  value       = aws_iam_role.github_actions.arn
}
