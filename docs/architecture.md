# Architecture (providers)

`providers/` owns the thin auth proxy for OpenAI/Codex Responses. The current runtime service name remains `openai-codex-server`.

## Source Layout

- `crates/provider-codex-auth/src/services/auth.rs`: OAuth auth loading/refresh
- `crates/provider-codex-auth/src/models/auth.rs`: auth schemas
- `crates/provider-server/src/handlers`: thin HTTP route handlers
- `crates/provider-server/src/services`: config, upstream forwarding
- `crates/provider-server/src/models`: minimal schemas for health and `/openai/v1/responses`

## Current Status

- `GET /openai/v1/health` is implemented
- `POST /openai/v1/responses` is implemented as the main proxy path
- `auth.json` loading and OAuth refresh are implemented in the auth crate
- `auth.example.json` is the public template; real `auth.json` stays local
- upstream auth injection is implemented through `Authorization` and `chatgpt-account-id`
- OpenAPI is generated with `utoipa`
- Swagger UI is mounted at `/swagger-ui`
- container delivery runs the provider-server binary on port `8080`

## Boundary

The server keeps only upstream account-facing concerns:

- OAuth credential loading and refresh (provider-codex-auth)
- upstream auth/header injection
- basic forwarding and header hygiene

The server does not own:

- `previous_response_id` rewriting
- response cache for continuation semantics
- chat/responses conversion
- tool-call extraction
- SSE semantic parsing
- prompt cache key generation

Those protocol responsibilities belong in the caller-side provider adapter.
