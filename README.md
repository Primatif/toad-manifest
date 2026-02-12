# toad-manifest

Context generation and manifest engine for the
[Primatif Toad](https://github.com/Primatif/Primatif_Toad) ecosystem.

## What It Does

`toad-manifest` is the **context generation engine**. It transforms raw project
data into AI-optimized output formats — the tiered prompt system that makes Toad
a context oracle.

- **Markdown Manifest** — `generate_markdown()` produces the high-density
  `MANIFEST.md` table with project stack, activity, VCS status, essence, and
  taxonomy. Token-budget aware.
- **System Prompt** — `generate_system_prompt()` creates `SYSTEM_PROMPT.md`,
  the ecosystem-level briefing designed to be injected into an AI agent's
  system prompt.
- **llms.txt** — `generate_llms_txt()` produces the ecosystem sitemap adapted
  from the llmstxt.org web standard for local use. This is the file an AI
  agent reads first.
- **AGENTS.md** — `generate_agents_md()` creates per-project operational
  instructions in the AGENTS.md format supported by 20+ AI tools.
- **CONTEXT.md** — `generate_project_context_md()` produces deep-dive project
  briefings with technical DNA, semantic essence, structure, artifacts, and
  submodule tables.
- **Token Mitigation** — All generators respect configurable token budgets via
  character-count estimation and truncation.

## Role in the Ecosystem

`toad-manifest` is the output layer. It depends only on `toad-core` for data
models. It consumes the `ProjectDetail` data produced by `toad-discovery` and
generates all the files that live in `~/.toad/shadows/`.

```text
toad-discovery ── produces ProjectDetail[] ──┐
                                             ├── toad-manifest ── generates:
toad-core ── provides data models ───────────┘     ├── MANIFEST.md
                                                   ├── SYSTEM_PROMPT.md
                                                   ├── llms.txt
                                                   ├── {project}/AGENTS.md
                                                   └── {project}/CONTEXT.md
```

## License

BUSL-1.1
