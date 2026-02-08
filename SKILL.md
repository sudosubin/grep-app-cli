---
name: grep-app-cli
description: >
  Find real-world code examples from over a million public GitHub repositories using grep-app-cli.
  This tool searches for literal code patterns (like grep), not keywords or natural language.
  Use when implementing unfamiliar APIs, looking up real usage patterns, finding idiomatic code
  in a specific language, or understanding how libraries work together in production codebases.
---

# grep-app-cli

Search code across 1M+ public GitHub repos via [grep.app](https://grep.app).

```
grep-app-cli [OPTIONS] <QUERY>
```

## Options

| Flag | Description |
|---|---|
| `--match-case` | Case sensitive search |
| `--match-whole-words` | Match whole words only |
| `--use-regexp` | Interpret query as a regular expression |
| `--repo <REPO>` | Filter by repository (e.g., `facebook/react`) |
| `--path <PATH>` | Filter by file path (e.g., `src/components/`) |
| `--language <LANG>` | Filter by language (repeatable) |
| `--json` | Output structured JSON |

## Translating Questions to Queries

Think about what the code literally looks like, then search for that string.

| Question | Command |
|---|---|
| How do devs handle auth in Next.js? | `grep-app-cli --language TypeScript 'getServerSession'` |
| Common error boundary patterns? | `grep-app-cli --language TSX 'ErrorBoundary'` |
| Real useEffect cleanup examples? | `grep-app-cli --use-regexp '(?s)useEffect\(\(\) => {.*removeEventListener'` |
| How is CORS handled in Flask? | `grep-app-cli --match-case --language Python 'CORS('` |
| Go HTTP handler implementations? | `grep-app-cli --use-regexp --match-case --language Go 'func\s+\(.*\)\s+ServeHTTP'` |
| Usage of createContext in React repo? | `grep-app-cli --repo facebook/react 'createContext'` |

## Best Practices

- **Include surrounding syntax** — search `useState(` not `useState` to disambiguate calls from docs.
- **Start specific, then broaden** — begin with `--language`/`--repo` filters, remove if too few results.
- **Use `--use-regexp` with `(?s)` prefix** for multi-line matching (e.g., `'(?s)impl\s+Display\s+for'`).
- **Results are capped at 10** — use filters to ensure relevance.
- **Use `--json`** for programmatic consumption; default output has terminal syntax highlighting.
- **Language names**: `TypeScript`, `JavaScript`, `TSX`, `JSX`, `Go`, `Rust`, `Python`, `Java`, etc.
