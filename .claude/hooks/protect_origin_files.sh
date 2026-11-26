#!/usr/bin/env bash
# PreToolUse: Origin's files are not the Generator's to change. Refuses any
# edit to tests/guard/ or .claude/, by file tool or by shell command.
in=$(cat)
t=$(jq -r '.tool_name' <<<"$in")
if [ "$t" = "Bash" ]; then
  target=$(jq -r '.tool_input.command' <<<"$in")
else
  target=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$in")
fi
if grep -qE '(^|[/ "'"'"'=])(tests/guard/|\.claude/(settings|hooks))' <<<"$target"; then
  if [ "$t" = "Bash" ] && ! grep -qE '(>|\btee\b|\bsed\b.*-i|\bmv\b|\bcp\b|\brm\b|\bchmod\b|\btruncate\b|\bln\b|git (checkout|restore|reset|rm|mv|apply|stash)|python|perl|\bdd\b)' <<<"$target"; then
    exit 0   # reading Origin's files is allowed
  fi
  jq -n '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"deny",permissionDecisionReason:"tests/guard/ and .claude/ are Origin'"'"'s: what is admitted and the hooks that enforce it. Ask Origin to change them."}}'
  exit 0
fi
exit 0
