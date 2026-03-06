# lylics

A lightweight Rust microservice that streams random lyrics chunks via **Server-Sent Events (SSE)**.

Built with [Axum](https://github.com/tokio-rs/axum), compiled to a static musl binary, and shipped in a [distroless](https://github.com/GoogleContainerTools/distroless) container (~5 MB).

## API

| Endpoint | Method | Description |
|---|---|---|
| `/healthz` | GET | Liveness probe |
| `/readyz` | GET | Readiness probe (includes version + lyrics count) |
| `/api/v1/random` | GET | Returns a single random lyric chunk as JSON |
| `/api/v1/stream?interval=10` | GET | SSE stream of random lyrics (interval in seconds, default 10) |

### SSE Example

```bash
curl -N http://localhost:3000/api/v1/stream?interval=5
```

```
event: lyric
data: {"artist":"Your Artist","song":"Your Song","chunk":"Some lyric line"}

event: lyric
data: {"artist":"Another Artist","song":"Another Song","chunk":"Another line"}
```

### Browser Example

```javascript
const source = new EventSource('http://localhost:3000/api/v1/stream?interval=10');
source.addEventListener('lyric', (e) => {
  const { artist, song, chunk } = JSON.parse(e.data);
  console.log(`${artist} — ${song}: ${chunk}`);
});
```

## Configuration

| Env Variable | Default | Description |
|---|---|---|
| `LYLICS_HOST` | `0.0.0.0` | Bind address |
| `LYLICS_PORT` | `3000` | Bind port |
| `LYLICS_DATA_PATH` | (embedded) | Path to custom lyrics JSON file |
| `RUST_LOG` | `lylics=info` | Log level |

## Lyrics Data Format

```json
[
  {
    "artist": "Artist Name",
    "song": "Song Title",
    "chunks": [
      "First lyrics chunk",
      "Second lyrics chunk"
    ]
  }
]
```

## Development

```bash
cargo run
# or with live reload
cargo watch -x run
```

## Docker

```bash
docker build -t lylics .
docker run -p 3000:3000 lylics
```

## Helm

The chart lives in [kayaman/helm-charts](https://github.com/kayaman/helm-charts) and can also be found under `charts/lylics/` in this repo.

```bash
helm install lylics charts/lylics \
  --set lyricsData='[{"artist":"Me","song":"My Song","chunks":["hello world"]}]'
```

## Release

Push a semver tag to trigger the full pipeline:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This will: lint → test → build & push container to `ghcr.io/kayaman/lylics` → deploy to AWS ECS → create a GitHub Release with auto-generated changelog.

## AWS Infrastructure

The infrastructure is managed with Terraform in `terraform/` and deployed via GitHub Actions.

### Architecture

```
Internet → Route53 (lylics.magj.dev)
              ↓
         ALB (HTTPS :443, ACM cert)
              ↓ HTTP :80 redirects → HTTPS
         ECS Fargate Spot (port 3000) ← default VPC public subnets
              ↓
         GHCR (ghcr.io/kayaman/lylics)
```

**Resources provisioned:**
- **Default VPC** — no custom networking, no NAT Gateway
- **ALB** with HTTPS (ACM certificate, auto-validated via DNS) + HTTP→HTTPS redirect
- **Route53** A record: `lylics.magj.dev` → ALB
- **ECS Fargate Spot** (0.25 vCPU, 512 MB) in public subnets with public IP
- **GHCR** — container images stored in GitHub Container Registry (no ECR)
- **GitHub OIDC** IAM role (no long-lived AWS credentials)
- **CloudWatch** log group (7-day retention)

**Estimated cost:** ~$5–15/month (Fargate Spot + ALB, no NAT Gateway, no ECR)

### Prerequisites

- AWS account with admin access
- [Terraform](https://developer.hashicorp.com/terraform/install) >= 1.5
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) configured
- [GitHub CLI](https://cli.github.com/) authenticated (`gh auth login`)

### First-time Setup

Run the orchestrator script that bootstraps the state backend, provisions infrastructure, and configures GitHub Actions secrets/variables:

```bash
./scripts/setup.sh
```

Or run each step individually:

```bash
# 1. Bootstrap the remote state backend (S3 + DynamoDB)
./scripts/bootstrap.sh

# 2. Provision infrastructure
./scripts/infra-up.sh

# 3. Set GitHub Actions secrets and variables from Terraform outputs
./scripts/setup-github.sh
```

After setup, note the outputs:
- `app_url` — `https://lylics.magj.dev`
- `github_actions_role_arn` — automatically set as `AWS_ROLE_ARN` secret

### Deploying

Deployments happen automatically via GitHub Actions:
- **Push to `main`** → build, push to GHCR, deploy to ECS
- **Push a `v*` tag** → same as above + create GitHub Release

### Teardown

```bash
# Destroy main infrastructure
./scripts/infra-down.sh

# Destroy everything including state backend
./scripts/infra-down.sh --include-bootstrap
```

### Manual Operations

```bash
# Check service status
aws ecs describe-services --cluster lylics --services lylics --query 'services[0].{status:status,running:runningCount,desired:desiredCount}'

# View logs
aws logs tail /ecs/lylics --follow

# Force new deployment (same image)
aws ecs update-service --cluster lylics --service lylics --force-new-deployment
```

## License

MIT
