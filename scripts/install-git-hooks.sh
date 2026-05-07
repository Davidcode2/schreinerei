#!/usr/bin/env bash

set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
hooks_dir=$(git rev-parse --git-path hooks)

mkdir -p "$hooks_dir"

for hook in pre-commit pre-push; do
  target="$hooks_dir/$hook"
  cat > "$target" <<EOF
#!/usr/bin/env bash
set -euo pipefail
exec "$repo_root/.githooks/$hook" "\$@"
EOF
  chmod +x "$target"
done

printf 'Installed git hooks into %s\n' "$hooks_dir"
