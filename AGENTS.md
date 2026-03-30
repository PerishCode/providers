# providers

## Provider Workspace

`providers/` is the provider workspace. Its current compose service and Docker image are still named `openai-codex-server`.

Do not publish real `auth.json`; keep local secrets in `auth.json` and use `auth.example.json` as the public template.

## Start Here

- `docs/architecture.md`: current thin-proxy shape and boundaries

## Directory Rules

- `crates/provider-codex-auth/src/{models,services}`: auth models + AuthService
- `crates/provider-server/src/{handlers,models,services}`: HTTP handlers, config, upstream
- `docs/`: architecture and cleanup notes
- `Dockerfile`: builds provider-server binary

## Common Commands (from `providers/`)

Run local Rust server:

```bash
cargo run -p provider-codex-server
```

Build check:

```bash
cargo check -p provider-codex-server
```

Format:

```bash
cargo fmt -p provider-codex-server
```

Run Docker image for the current `openai-codex-server` service:

```bash
docker build -t openai-codex-server .
docker run --rm -p 8080:8080 -v "$PWD/auth.json:/app/auth.json" openai-codex-server
```

Run Docker Compose (from root):

```bash
docker compose up --build
```

Smoke test current routes:

```bash
curl http://127.0.0.1:8080/openai/v1/health
curl -sN -X POST http://127.0.0.1:8080/openai/v1/responses \
  -H 'Content-Type: application/json' \
  --data '{"model":"gpt-5.4","instructions":"Reply with one short word.","input":[{"role":"user","content":[{"type":"input_text","text":"hello"}]}],"stream":true,"store":false}'
```

Current upstream contract requires `stream=true` and `store=false` for this smoke call.

## Runtime Defaults

- Swagger UI: `http://127.0.0.1:8080/swagger-ui`

Required local auth file shape: copy `auth.example.json` to `auth.json` and fill real credentials locally.

Useful env vars:

- `AUTH_FILE`: auth file path
- `PORT`: bind port
- `CODEX_API_ENDPOINT`: upstream Codex responses endpoint
- `OPENAI_CLIENT_ID`: OAuth client id for token refresh
- `OPENAI_ISSUER`: OAuth issuer base URL

## FAQ

Legacy `openai-codex-server/` source code moved into `providers/`; the compose service name remains `openai-codex-server`.
