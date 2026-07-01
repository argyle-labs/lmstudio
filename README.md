<p align="center">
  <img src="assets/icon-256.png" width="120" alt="lmstudio" />
</p>

# lmstudio

LM Studio is a desktop app that runs local LLMs behind an OpenAI-compatible server — install it on the host and connect.

A first-party [orca](https://github.com/argyle-labs/orca) plugin (model provider).

This is a **backend/adapter** — it has no service of its own; it wires an existing system into orca.

---

## Run it without orca

There's nothing to deploy: this plugin drives software you already run (upstream: <https://lmstudio.ai/>). Install/configure that directly, then register it with orca.


## With orca

orca drives this plugin through its generic surface — rich, lmstudio-specific data comes back in the typed `service.status` payload, never bespoke tools.

## Layout

- `src/` — the plugin (pure Rust): the `ServiceBackend` descriptor + `configure` / `status`.
- [CAPABILITIES.md](CAPABILITIES.md) — the service-backend contract checklist.
- `assets/` — plugin icon.
