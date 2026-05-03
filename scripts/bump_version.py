#!/usr/bin/env python3

import json
import re
import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent


def bump_patch(version: str) -> str:
    match = re.fullmatch(r"(\d+)\.(\d+)\.(\d+)", version)
    if match is None:
        raise ValueError(f"unsupported version format: {version}")

    major, minor, patch = (int(part) for part in match.groups())
    return f"{major}.{minor}.{patch + 1}"


def update_cargo_toml(path: Path, new_version: str) -> None:
    content = path.read_text()
    pattern = re.compile(
        r'(?ms)(^\[package\]\n(?:.*?\n)*?^version\s*=\s*")([0-9]+\.[0-9]+\.[0-9]+)(")'
    )
    updated, count = pattern.subn(rf'\g<1>{new_version}\g<3>', content, count=1)
    if count != 1:
        raise ValueError("failed to update package.version in Cargo.toml")
    path.write_text(updated)


def update_frontend_package_json(path: Path, new_version: str) -> None:
    data = json.loads(path.read_text())
    data["version"] = new_version
    path.write_text(json.dumps(data, indent=2) + "\n")


def update_frontend_package_lock(path: Path, new_version: str) -> None:
    data = json.loads(path.read_text())
    data["version"] = new_version
    root_package = data.get("packages", {}).get("")
    if isinstance(root_package, dict):
        root_package["version"] = new_version
    path.write_text(json.dumps(data, indent=2) + "\n")


def main() -> int:
    if len(sys.argv) != 2 or sys.argv[1] != "patch":
        print("usage: bump_version.py patch", file=sys.stderr)
        return 1

    cargo_toml_path = ROOT / "Cargo.toml"
    with cargo_toml_path.open("rb") as file:
        cargo_data = tomllib.load(file)

    current_version = cargo_data["package"]["version"]
    new_version = bump_patch(current_version)

    update_cargo_toml(cargo_toml_path, new_version)
    update_frontend_package_json(ROOT / "frontend/package.json", new_version)
    update_frontend_package_lock(ROOT / "frontend/package-lock.json", new_version)

    print(f"Bumped version: {current_version} -> {new_version}")
    print(new_version)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
