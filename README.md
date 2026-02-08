# grep-app-cli

CLI for [grep.app](https://grep.app) — search code across 1M+ public GitHub repos with syntax highlighting. Powered by [mcp.grep.app](https://mcp.grep.app).

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
|---|---|
| `--match-case` | Case sensitive search |
| `--match-whole-words` | Match whole words only |
| `--use-regexp` | Interpret query as a regular expression |
| `--repo <REPO>` | Filter by repository |
| `--path <PATH>` | Filter by file path |
| `--language <LANG>` | Filter by language (repeatable) |
| `--json` | Output raw JSON |

## License

[MIT](LICENSE)
