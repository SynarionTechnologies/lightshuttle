# AGENT.md — Contribution rules and working framework

## Agent role
You are an OpenAI Codex agent contributing to the **LightShuttle** project.
Produce code, documentation and prompts that follow the standards and organisation
outlined below.

## Documentation structure
- Documentation lives in `/docs/fr` (French) and `/docs/en` (English).
- Keep both versions synchronised in content and structure.
- Each page must be clear, concise and directly useful to developers.

## Glossary
- A `glossary.md` file exists in FR and EN.
- Add every new technical term, concept or acronym with:
  - **Name**
  - **Definition**
  - **Context of use** in LightShuttle

## README
- The main `README.md` is in English with a link to the French version.
- Include: short description, key features, quick installation, links to docs and licence.

## Prompts
- Dynamic variables are wrapped in `{{curly_braces}}`.
- Each prompt has its own page under `/docs/fr/prompts/` and `/docs/en/prompts/`.

## Good practices
1. **FR/EN coherence** – always update both languages.
2. **Living glossary** – never forget to add new terms.
3. **Clean commits** – use Conventional Commits (`feat:`, `fix:`, `docs:` …).
4. **Clarity** – avoid unnecessary jargon; prefer simple sentences.
5. **Cross references** – link relevant pages and sections.
6. **Idiomatic Rust** – write readable, maintainable code; handle errors properly and avoid `unwrap`/`expect` outside tests.
7. **Comments and documentation** – use `///` doc comments for public items and inline comments for complex logic; include examples where helpful.
8. **Formatting and linting** – run `cargo fmt --all` and `cargo clippy --all-targets --all-features -- -D warnings` before committing.
9. **Testing** – provide unit, integration and doc tests; run `cargo test` and ensure all tests pass.

---
