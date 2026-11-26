#!/usr/bin/env bash
# Stop: before Claude stops, if the Rust code changed since the last pass,
# run every check (cargo test, cargo test --features v9). A failure blocks
# the stop and is fed back (exit 2).
cd "${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel)}" || exit 0
stamp=target/.guard_passed
now=$( { git rev-parse HEAD; git diff HEAD -- '*.rs' Cargo.toml Cargo.lock; git ls-files -o --exclude-standard -- '*.rs' | xargs -r cat; } | sha256sum | cut -d' ' -f1)
[ -f "$stamp" ] && [ "$(cat "$stamp")" = "$now" ] && exit 0
for args in "" "--features v9"; do
  out=$(cargo test -q $args 2>&1)
  if [ $? -ne 0 ]; then
    echo "CHECKS FAILED (cargo test $args). Not done until they pass." >&2
    echo "$out" | grep -E 'panicked|FAILED|failed|error(\[|:)|^---- ' | head -40 >&2
    exit 2
  fi
done
mkdir -p target && echo "$now" > "$stamp"
exit 0
