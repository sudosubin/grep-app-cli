<div align="center">

# grep-app-cli

[![version](https://badgen.net/github/release/sudosubin/grep-app-cli?label=version)](https://github.com/sudosubin/grep-app-cli/releases)
[![license](https://badgen.net/github/license/sudosubin/grep-app-cli?color=green)](LICENSE)
[![downloads](https://badgen.net/github/assets-dl/sudosubin/grep-app-cli?color=green)](https://github.com/sudosubin/grep-app-cli/releases)

CLI for [grep.app](https://grep.app) — search code across 1M+ public GitHub repos with syntax highlighting. Powered by [mcp.grep.app](https://mcp.grep.app).

</div>

## Quick Start

```sh
cargo install grep-app-cli
grep-app-cli 'useState('
```

## Installation

```sh
cargo install grep-app-cli
```

Or download a binary from [GitHub Releases](https://github.com/sudosubin/grep-app-cli/releases).

## Usage

```sh
grep-app-cli [OPTIONS] <QUERY>
```

```sh
grep-app-cli 'useState('
grep-app-cli --language TypeScript --language TSX 'getServerSession'
grep-app-cli --use-regexp --match-case '(?s)useEffect\(.*cleanup'
grep-app-cli --repo facebook/react 'createContext'
grep-app-cli --json 'async function'
```

## Options

| Flag | Description |
| --- | --- |
| `--match-case` | Case sensitive search |
| `--match-whole-words` | Match whole words only |
| `--use-regexp` | Interpret query as a regular expression |
| `--repo <REPO>` | Filter by repository |
| `--path <PATH>` | Filter by file path |
| `--language <LANG>` | Filter by language (repeatable) |
| `--json` | Output raw JSON |

## Shell Completions

Generate static completions with `completion <shell>`.

```sh
grep-app-cli completion bash
grep-app-cli completion elvish
grep-app-cli completion fish
grep-app-cli completion powershell
grep-app-cli completion zsh
```

## Use with AI Agents

You can add and use the [`grep-app-cli`](./skills/grep-app-cli) skill.

```sh
npx skills add sudosubin/grep-app-cli
```

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --all-features
```

## License

MIT, see [LICENSE](./LICENSE).
