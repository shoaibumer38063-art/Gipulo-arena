#!/usr/bin/env bash
# Commit everything and push to origin/main.
# Credentials: GITHUB_TOKEN environment variable ONLY. The token is passed per command as an HTTP
# header and is never written to .git/config or any file.
# Usage: scripts/sync-github.sh "commit message"
set -euo pipefail
cd "$(dirname "$0")/.."
msg="${1:-Update $(date -u +%Y-%m-%dT%H:%MZ)}"
git add -A
if ! git diff --cached --quiet; then
  git commit -q -m "$msg"
  echo "committed: $msg"
else
  echo "nothing new to commit"
fi
if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo "GITHUB_TOKEN is not set: changes are committed locally; push skipped." >&2
  exit 2
fi
auth=$(printf 'x-access-token:%s' "$GITHUB_TOKEN" | base64 | tr -d '\n')
git -c "http.extraheader=Authorization: Basic $auth" push -q origin main
echo "pushed to origin/main"
