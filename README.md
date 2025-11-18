# Gut

```

     ________________/  \       ______             _
    /   ________________/      / _____\   _    _  | |___
    \________________   \     | |     _  | |  | | |  ___|
    /   ________________/     | |____| | | |__| | | |___
    \________________   \      \/____//   \_\___| |_|__/
     /  ________________/
     \  /

```

_What a 'git' needs is just its gut!_

A CLI tool that wraps the Git version controller, providing smart subcommand inference, commit message formatting, config-driven hooks, and convenient shortcuts.

## Getting Started

### Installation

First you must have Git and Rust toolchain installed. Then just run the following to build from source:

```bash
git clone https://github.com/elliot-zzh/gut
cd gut
cargo install --path .
```

### Basic Usage

Just replace all your `git` with `gut`! Gut will automatically handle your typos / abbr command.

`gut commit` is one of the modified command: you don't need to add `-m` since the last argument will be treated as the commit message. Also conventional commits such as `feat: xxx` will automatically have emojis added.

For other modified commands refer to below. But most of the git commands are just with typo / abbr inference.

## Full Features

### Smart Git Workflow
- **Automatically infer git subcommands** from short abbreviations or typos
- **Colorized output** throughout for better readability
- **Better error messages** with helpful suggestions

### Enhanced Commit Experience
- **Interactive commit mode**: Just run `gut commit` with no arguments to enter interactive mode
- **Direct commit**: `gut commit <message>` - last argument is the commit message (no `-m` needed)
- **Auto-format commit messages**: Write `feat:xxx` or `feat(scope):xxx` and gut converts it to `feat: ✨ xxx` or `feat(scope): ✨ xxx`
- **Supports many conventional commit types**: feat, fix, docs, refactor, test, chore, build, style, ci, perf, revert
- **Custom emoji mapping** for commit types via `gut.config.json`
- **Commit message formatting modes**: `upper_case`/`lower_case` (configurable)

### Smart Undo Operations
- **`gut undo commit`**: Undo last commit (keep changes in staging)
- **`gut undo commit-hard`**: Undo last commit and discard changes (with confirmation)
- **`gut undo stage`**: Unstage all files
- **`gut undo changes`**: Discard all working changes (with confirmation)

### Better Stash Management
- **`gut save [message]`**: Save changes with an optional message (better than `git stash`)
- **`gut pop`**: Restore saved changes (better than `git stash pop`)

### Secret Protection
- **`gut remove-committed <files...>`**: Remove accidentally committed sensitive files (API keys, .env, etc.)
  - Automatically gets the last commit message
  - Resets the commit (keeping changes)
  - Removes specified files from git tracking
  - Re-commits without the sensitive files
  - Provides helpful reminders about .gitignore and secret rotation

### Log & Branch Management
- **`gut branch <name>`**: Create and auto-switch to a new branch
- **`gut log`**: Dense, configurable log (default: latest 10, short id + message)
- **`gut rlog`**: Reversed log, following `log` config
- **`gut tlog`**: Tree log showing latest N commits from all branches (current branch first)

### Template Repositories
- **`gut template <url> [dest]`**: Clone a repo as template (remove .git, re-init)

### Configuration & Hooks
- **Configurable global git hooks** via `gut.config.json` (auto-generated in `.git/hooks`)
- **Configuration validation** with helpful error messages
- All other git commands pass through with typo/abbreviation inference

## Quick Examples

### Interactive Commit
```bash
gut commit
# Prompts: Enter commit message (or use conventional format like 'feat: description')
# > feat: add user authentication
# ℹ Commit message: feat: ✨ add user authentication
# ✓ Commit created successfully!
```

### Quick Conventional Commit
```bash
gut commit "feat: add login page"
# Automatically formatted to: feat: ✨ add login page
```

### Undo Last Commit
```bash
gut undo commit
# Undoes last commit but keeps all changes in staging area
```

### Save Work in Progress
```bash
gut save "working on authentication feature"
# Later...
gut pop
# Restores your saved changes
```

### Remove Accidentally Committed Secret
```bash
# Oops! Committed .env file with API keys
gut remove-committed .env
# Automatically:
# 1. Gets last commit message
# 2. Resets commit (keeps changes)
# 3. Removes .env from git
# 4. Re-commits without .env
# 5. Reminds you to add .env to .gitignore and rotate keys
```

### Smart Command Inference
```bash
gut comit "fix: typo"  # Infers 'commit'
gut sttus              # Infers 'status'
gut chckout main       # Infers 'checkout'
```

## Configuration (`gut.config.json`)

- `hooks`: List of git hooks to auto-generate
- `log`/`tlog`: Configure log count and info level (`less`/`more`) for the `log` & `tlog` commands.
- `commit`: Configure commit message formatting mode, custom emoji mapping for commit types (footer), enable/disable emoji, and require conventional commit style

Example:

```json
{
  "commit": {
    "format_mode": "upper_case", // other formating mode. also try `lowe_case`!
    "emoji_enabled": true, // only work when using conventional commit
    "require_conventional": false,
    "emoji_mapping": { // customize emojis with conventional commit types. any type supported
      "custom": "🌟"
    }
  },
  "log": {
    "count": 15, // print 10 latest records. 10 by default
    "info": "less" // print less
  },
  "tlog": { "count": 20, "info": "more" },
  "hooks": [
    { "name": "pre-commit", "commands": ["echo Pre-commit hook"] }
  ]
}
```

## License and Others

This project is licensed under MIT license. Code mainly generated by Copilot Agent and GPT. Any issues or contributions welcomed!
