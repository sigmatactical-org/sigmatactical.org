#!/usr/bin/env bash
# Back-compat wrapper — use sync-github-metadata.sh.
exec "$(dirname "$0")/sync-github-metadata.sh" "$@"
