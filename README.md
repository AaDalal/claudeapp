# claudeapp

A tiny macOS command-line tool that opens a folder in [Claude Code](https://code.claude.com/docs/en/desktop), the coding tab of the Claude desktop app.

Under the hood, it builds a [deep link](https://support.claude.com/en/articles/14729294-open-claude-desktop-with-a-link) like `claude://code/new?folder=/your/folder` and hands it to the app.

## Install

You need [Rust](https://rustup.rs). Then, from a clone of this repo:

```sh
cargo install --path .
```

## Use

```sh
claudeapp ~/dev/my-project   # open a folder
claudeapp                    # open the current directory
claudeapp --print .          # print the URL instead of opening it
```

## Notes

- macOS only.
- You need the [Claude desktop app](https://claude.com/download).
- Claude asks you to confirm before it uses the folder. That is a safety feature of the app, not a bug.
