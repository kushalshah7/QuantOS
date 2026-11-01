#!/usr/bin/env bash
# Runs only while building the isolated development image.
set -euo pipefail
cd /tmp/quantos-toolchains
versions="$(python3 - <<'PY'
import json, re, tomllib
from pathlib import Path
rust = tomllib.loads(Path("rust-toolchain.toml").read_text())["toolchain"]
engines = json.loads(Path("package.json").read_text())["engines"]
values = [rust["channel"], engines["node"], engines["npm"]]
assert all(re.fullmatch(r"\d+\.\d+\.\d+", value) for value in values)
assert rust["profile"] == "minimal"
assert set(rust["components"]) == {"rustfmt", "clippy"}
assert rust["targets"] == ["wasm32-unknown-unknown"]
print(*values)
PY
)"
read -r rust_version node_version npm_version <<< "$versions"
case "$(uname -m)" in
    x86_64) node_arch=x64; rust_host=x86_64-unknown-linux-gnu ;;
    aarch64) node_arch=arm64; rust_host=aarch64-unknown-linux-gnu ;;
    *) echo "Unsupported native development architecture." >&2; exit 1 ;;
esac
download_dir="$(mktemp -d)"
trap 'rm -rf "$download_dir"' EXIT
checksum_file="$PWD/downloads.sha256"
download() {
    local url="$1" filename="$2"
    curl --fail --silent --show-error --location --proto '=https' --tlsv1.2 \
        "$url" --output "$download_dir/$filename"
    (
        cd "$download_dir"
        grep -F "  $filename" "$checksum_file" | sha256sum --check --strict -
    )
}
node_file="node-v$node_version-linux-$node_arch.tar.xz"
download "https://nodejs.org/dist/v$node_version/$node_file" "$node_file"
tar -xJf "$download_dir/$node_file" --directory /usr/local --strip-components=1

# npm's tarball is pinned independently of the Node distribution's bundled npm.
test "$npm_version" = 11.17.0
curl --fail --silent --show-error --location --proto '=https' --tlsv1.2 \
    "https://registry.npmjs.org/npm/-/npm-$npm_version.tgz" \
    --output "$download_dir/npm.tgz"
python3 - "$download_dir/npm.tgz" <<'PY'
import base64, hashlib, sys
from pathlib import Path
expected = "PurxiZexEHDTE4SSaLI3ZrnbAGiZfeyUcQcxcP5D+hfytNAze/D1IzDuInTn9XVLIbAQUnQuSPXJx02LHjLvQw=="
actual = base64.b64encode(hashlib.sha512(Path(sys.argv[1]).read_bytes()).digest()).decode()
assert actual == expected, "npm archive integrity mismatch"
PY
npm install --global --ignore-scripts --no-audit --no-fund "$download_dir/npm.tgz"
test "$(node --version)" = "v$node_version"
test "$(npm --version)" = "$npm_version"

rustup_file="rustup-init-$rust_host"
download "https://static.rust-lang.org/rustup/archive/1.29.2/$rust_host/rustup-init" "$rustup_file"
cp "$download_dir/$rustup_file" "$download_dir/rustup-init"
chmod +x "$download_dir/rustup-init"
"$download_dir/rustup-init" -y --no-modify-path --profile minimal --default-toolchain none
rustup set auto-self-update disable
rustup toolchain install "$rust_version" --profile minimal \
    --component rustfmt --component clippy --target wasm32-unknown-unknown
rustup default "$rust_version"
test "$(rustc --version | cut -d' ' -f2)" = "$rust_version"
