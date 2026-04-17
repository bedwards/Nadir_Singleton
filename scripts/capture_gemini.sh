#!/usr/bin/env bash
# capture_gemini.sh <pr-number>
#
# For each distinct Gemini (or human) review comment on the PR, open a new
# GitHub issue labelled `from-review` and reply in the PR thread with the link.
# Idempotent: skips comments already captured (tracked by marker).
set -euo pipefail

PR="${1:?usage: capture_gemini.sh <pr-number>}"
MARKER="<!-- nadir-capture:done -->"

comments_json="$(gh pr view "$PR" --json comments,reviewThreads,reviews)"

echo "$comments_json" | jq -c '
  [
    (.comments // [])[]                  | {id, body, author: (.author.login // ""), where: "conversation"},
    (.reviews  // [])[]                   | select(.body != null and .body != "")
                                          | {id, body, author: (.author.login // ""), where: "review"},
    (.reviewThreads // [])[]
      | .comments[]?                     | {id, body, author: (.author.login // ""), where: "review-thread", path, line}
  ]
  | .[]
' | while read -r item; do
    id="$(echo "$item" | jq -r '.id')"
    body="$(echo "$item" | jq -r '.body')"
    author="$(echo "$item" | jq -r '.author')"
    where="$(echo "$item" | jq -r '.where')"
    path="$(echo "$item" | jq -r '.path // empty')"
    line="$(echo "$item" | jq -r '.line // empty')"

    # Skip already-marked comments, bots we don't care about, and empty bodies.
    if echo "$body" | grep -q "$MARKER"; then continue; fi
    if [[ -z "$body" || "$body" == "null" ]]; then continue; fi

    title="review: $(echo "$body" | head -1 | cut -c1-72)"
    issue_body="From PR #$PR, $where by @$author"
    [[ -n "$path" ]] && issue_body+=$'\n\n'"**Location:** \`$path\`"
    [[ -n "$line" && "$line" != "null" ]] && issue_body+=":$line"
    issue_body+=$'\n\n'"**Finding:**"$'\n\n'"$body"

    url="$(gh issue create --label from-review --title "$title" --body "$issue_body" 2>/dev/null | tail -1)"
    echo "captured: $url"

    # Reply on PR so reviewer knows we've tracked it.
    gh pr comment "$PR" --body "Captured as $url $MARKER" >/dev/null || true
done

echo "done."
