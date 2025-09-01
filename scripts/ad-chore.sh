#! /usr/bin/env bash

git filter-repo --force \
  --commit-callback '
import re
msg = commit.message.decode("utf-8","replace")

# Opcional: no tocar merges
if len(commit.parents) > 1:
    return

lines = msg.splitlines()
if not lines:
    return

subject = lines[0].strip()
types = r"(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)"
valid = re.compile(rf"^{types}(\([^)]*\))?:")  # ya correcto -> no tocar
if valid.match(subject):
    return

# Caso problema: "type desc" sin ":" ni "(scope)"
m = re.match(rf"^({types})\s+(?!\()(.+)$", subject)
if m:
    new_subject = f"{m.group(1)}: {m.group(2).strip()}"
else:
    # fallback: prefijar chore:
    new_subject = f"chore: {subject}" if subject else "chore:"

lines[0] = new_subject
new_msg = "\n".join(lines)
if msg.endswith("\n"):
    new_msg += "\n"
commit.message = new_msg.encode("utf-8")
'