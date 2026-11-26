#!/usr/bin/env bash
# PostToolUse (Edit|Write|MultiEdit|NotebookEdit): after any change to Rust
# code or the manifest, run the guard and the derivation register. A failure
# is fed back to Claude at once (exit 2).
f=$(jq -r '.tool_input.file_path // .tool_response.filePath // empty')
case "$f" in *.rs|*/Cargo.toml) ;; *) exit 0 ;; esac
cd "${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel)}" || exit 0
out=$(cargo test -q --test guard_operators --test v10_derivation 2>&1)
if [ $? -ne 0 ]; then
  echo "GUARD FAILED after editing $f. Fix the code; do not change tests/guard/ (Origin's)." >&2
  echo "$out" | grep -vE '^\s+(Compiling|Running|[0-9]+:|at )' | tail -40 >&2
  exit 2
fi
exit 0
