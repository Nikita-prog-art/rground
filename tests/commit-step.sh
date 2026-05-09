#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: ./tests/commit-step.sh [-m "commit message"] [--all] [--no-push] [-- path...]

Runs project checks, stages changes, commits them, and pushes the current branch.

Default staged paths:
  .cargo Cargo.lock Cargo.toml README.md assets docs src tests

Options:
  -m, --message  Commit message. Default: "Update game prototype"
  --all          Stage all changes with git add -A.
  --no-push      Commit only.

Environment:
  WINDOWS_CHECK=0  Skip cargo check --target x86_64-pc-windows-gnu.
USAGE
}

message="Update game prototype"
stage_all=0
push_after_commit=1
paths=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    -m|--message)
      shift
      if [[ $# -eq 0 ]]; then
        echo "Missing value for --message" >&2
        exit 2
      fi
      message="$1"
      ;;
    --all)
      stage_all=1
      ;;
    --no-push)
      push_after_commit=0
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      while [[ $# -gt 0 ]]; do
        paths+=("$1")
        shift
      done
      break
      ;;
    *)
      paths+=("$1")
      ;;
  esac
  shift
done

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

echo "Running format check..."
cargo fmt --check

echo "Running cargo check..."
cargo check

echo "Running cargo test..."
cargo test

if [[ "${WINDOWS_CHECK:-1}" != "0" ]]; then
  if rustup target list --installed | grep -qx "x86_64-pc-windows-gnu"; then
    echo "Running Windows target check..."
    cargo check --target x86_64-pc-windows-gnu
  else
    echo "Skipping Windows target check: x86_64-pc-windows-gnu is not installed."
  fi
fi

if [[ "$stage_all" -eq 1 ]]; then
  git add -A
elif [[ "${#paths[@]}" -gt 0 ]]; then
  git add -- "${paths[@]}"
else
  default_paths=(.cargo .gitignore Cargo.lock Cargo.toml README.md assets docs src tests)
  existing_paths=()
  for path in "${default_paths[@]}"; do
    if [[ -e "$path" ]]; then
      existing_paths+=("$path")
    fi
  done
  git add -- "${existing_paths[@]}"
fi

if git diff --cached --quiet; then
  echo "No staged changes to commit."
  git status --short
  exit 0
fi

git commit -m "$message"

if [[ "$push_after_commit" -eq 1 ]]; then
  branch="$(git branch --show-current)"
  if [[ -z "$branch" ]]; then
    echo "Cannot push from detached HEAD." >&2
    exit 1
  fi

  if git rev-parse --abbrev-ref --symbolic-full-name '@{u}' >/dev/null 2>&1; then
    git push
  else
    git push -u origin "$branch"
  fi
fi

git status --short
