#!/usr/bin/env bash
# Commit everything and push to the GitHub repo's main branch.
# Self-contained: the author identity and the remote URL are passed on every run, because the
# sandbox snapshot does not keep .git/config. Credentials come ONLY from GITHUB_TOKEN and are
# sent as an HTTP header per command; they are never written to disk.
# Usage: scripts/sync-github.sh "commit message"
# Exit codes: 0 pushed · 2 no token (committed locally only) · other = git failure.
set -euo pipefail
cd "$(dirname "$0")/.."
REMOTE_URL="https://github.com/shoaibumer38063-art/Gipulo-arena.git"
AUTHOR_NAME="Gipulo Agent"
AUTHOR_EMAIL="agent@gipulo.invalid"
msg="${1:-Update $(date -u +%Y-%m-%dT%H:%MZ)}"

git add -A
if ! git diff --cached --quiet; then
  git -c user.name="$AUTHOR_NAME" -c user.email="$AUTHOR_EMAIL" commit -q -m "$msg"
  echo "committed: $msg"
else
  echo "nothing new to commit"
fi

if [ -z "${GITHUB_TOKEN:-}" ]; then
  echo "GITHUB_TOKEN is not set: changes are committed locally; push skipped." >&2
  exit 2
fi
auth=$(printf 'x-access-token:%s' "$GITHUB_TOKEN" | base64 | tr -d '\n')
git -c "http.extraheader=Authorization: Basic $auth" push -q "$REMOTE_URL" HEAD:refs/heads/main
echo "pushed to $REMOTE_URL (main)"
