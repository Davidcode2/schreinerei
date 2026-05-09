#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/ci-selective.sh [--staged | --branch | --all] [--fast] [--dry-run]

Modes:
  --staged   Inspect staged files (for pre-commit)
  --branch   Inspect changes from merge-base with main to HEAD (default, for pre-push)
  --all      Run all backend and frontend checks unconditionally

Options:
  --fast     Run lightweight checks only
  --dry-run  Print selected scopes without running checks
EOF
}

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

mode="branch"
fast=false
dry_run=false

while (($# > 0)); do
  case "$1" in
    --staged)
      mode="staged"
      ;;
    --branch)
      mode="branch"
      ;;
    --all)
      mode="all"
      ;;
    --fast)
      fast=true
      ;;
    --dry-run)
      dry_run=true
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf 'Unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
  esac
  shift
done

resolve_base_ref() {
  if git show-ref --verify --quiet refs/remotes/origin/main; then
    printf 'origin/main\n'
    return
  fi

  if git show-ref --verify --quiet refs/heads/main; then
    printf 'main\n'
    return
  fi

  git rev-list --max-parents=0 HEAD | tail -n1
}

collect_changed_files() {
  case "$mode" in
    staged)
      git diff --cached --name-only --diff-filter=ACMR
      ;;
    branch)
      base_ref=$(resolve_base_ref)
      merge_base=$(git merge-base "$base_ref" HEAD)
      git diff --name-only "$merge_base...HEAD"
      ;;
    all)
      printf '__ALL__\n'
      ;;
  esac
}

source_backend_env() {
  if [[ -f .env ]]; then
    set -a
    # shellcheck disable=SC1091
    source .env
    set +a
  fi

  if [[ -z "${DATABASE_URL:-}" ]]; then
    printf 'DATABASE_URL is not set. Create a workspace-local .env before running backend checks.\n' >&2
    exit 1
  fi
}

run_backend_fast() {
  printf 'Running backend fast checks...\n'
  source_backend_env
  cargo fmt -- --check
}

run_backend_full() {
  printf 'Running backend CI-equivalent checks...\n'
  source_backend_env
  cargo fmt -- --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all
  cargo export-types
  if ! git diff --quiet -- frontend/src/types/generated.ts; then
    printf 'Generated TypeScript types are out of date. Run cargo export-types and commit frontend/src/types/generated.ts.\n' >&2
    exit 1
  fi
}

run_frontend_fast() {
  printf 'Running frontend fast checks...\n'
  (
    cd frontend
    npm run lint
  )
}

run_frontend_full() {
  printf 'Running frontend CI-equivalent checks...\n'
  (
    cd frontend
    npm run test:run
    npm run build
  )
}

run_backend=false
run_frontend=false
run_contract=false
run_shared=false
validate_backend_image=false
validate_frontend_image=false

mapfile -t changed_files < <(collect_changed_files)

if [[ "$mode" == "all" ]]; then
  run_backend=true
  run_frontend=true
  run_contract=true
  validate_backend_image=true
  validate_frontend_image=true
else
  for file in "${changed_files[@]}"; do
    [[ -z "$file" ]] && continue

    case "$file" in
      frontend/*|dockerfile.frontend|nginx.conf)
        run_frontend=true
        ;;
      src/*|migrations/*|Cargo.toml|Cargo.lock|dockerfile.backend)
        run_backend=true
        ;;
      .github/workflows/*|.dockerignore)
        run_shared=true
        run_backend=true
        run_frontend=true
        ;;
    esac

    case "$file" in
      dockerfile.frontend|nginx.conf)
        validate_frontend_image=true
        ;;
      dockerfile.backend)
        validate_backend_image=true
        ;;
      .dockerignore)
        validate_backend_image=true
        validate_frontend_image=true
        ;;
    esac

    case "$file" in
      frontend/src/types/generated.ts|src/common/*|src/modules/*/api/*|src/modules/iam/domain/user_preferences.rs|src/modules/inventory/domain/stock_entry.rs)
        run_contract=true
        run_frontend=true
        ;;
    esac

    if [[ "$file" == *.rs ]] && [[ -f "$file" ]] && grep -q '#\[ts(export' "$file"; then
      run_contract=true
      run_frontend=true
    fi
  done
fi

printf 'Selective check summary\n'
printf '  mode: %s\n' "$mode"
printf '  backend: %s\n' "$run_backend"
printf '  frontend: %s\n' "$run_frontend"
printf '  contract: %s\n' "$run_contract"
printf '  shared: %s\n' "$run_shared"
printf '  backend image: %s\n' "$validate_backend_image"
printf '  frontend image: %s\n' "$validate_frontend_image"

if ((${#changed_files[@]} > 0)) && [[ "$mode" != "all" ]]; then
  printf '  files:\n'
  for file in "${changed_files[@]}"; do
    [[ -z "$file" ]] && continue
    printf '    - %s\n' "$file"
  done
fi

if [[ "$dry_run" == true ]]; then
  exit 0
fi

if [[ "$run_backend" == false && "$run_frontend" == false ]]; then
  printf 'No backend or frontend checks required for the selected changes.\n'
  exit 0
fi

if [[ "$run_backend" == true ]]; then
  if [[ "$fast" == true ]]; then
    run_backend_fast
  else
    run_backend_full
  fi
fi

if [[ "$run_frontend" == true ]]; then
  if [[ "$fast" == true ]]; then
    run_frontend_fast
  else
    run_frontend_full
  fi
fi

if [[ "$fast" == false ]] && [[ "$validate_backend_image" == true || "$validate_frontend_image" == true ]]; then
  if ! command -v docker >/dev/null 2>&1; then
    printf 'Docker is required to validate image builds for docker-related changes.\n' >&2
    exit 1
  fi
fi

if [[ "$fast" == false ]] && [[ "$validate_backend_image" == true ]]; then
  printf 'Validating backend Docker image build...\n'
  docker build -f dockerfile.backend -t schreinerei-backend-check .
fi

if [[ "$fast" == false ]] && [[ "$validate_frontend_image" == true ]]; then
  printf 'Validating frontend Docker image build...\n'
  docker build \
    -f dockerfile.frontend \
    --build-arg VITE_KEYCLOAK_URL=https://auth.jakob-lingel.dev \
    --build-arg VITE_KEYCLOAK_REALM=schreinerei \
    --build-arg VITE_KEYCLOAK_CLIENT_ID=schreinerei_pwa_prod \
    --build-arg VITE_API_URL=https://schreinerei.jakob-lingel.dev \
    -t schreinerei-frontend-check .
fi
