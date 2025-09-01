#! /usr/bin/env bash

git filter-repo --force \
  --commit-callback '
import re
msg = commit.message.decode("utf-8", "replace")
valid = re.compile(r"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\([^)]*\))?:")
# si es merge, no tocar
if len(commit.parents) > 1:
    return
# si ya es convencional, no tocar
if valid.match(msg.strip()):
    return

lines = msg.splitlines()
first = lines[0].strip() if lines else ""
rest = "\n".join(lines[1:]) if len(lines) > 1 else ""
new_first = f"chore: {first}" if first else "chore:"
new_msg = new_first + (("\n" + rest) if rest else "")
if msg.endswith("\n"):
    new_msg += "\n"
commit.message = new_msg.encode("utf-8")
'