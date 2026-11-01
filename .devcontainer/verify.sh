#!/usr/bin/env bash
# Default: cheap environment checks. --smoke is for explicitly requested CI checks.
set -euo pipefail
if [[ $# -gt 1 || ( $# -eq 1 && "$1" != "--smoke" ) ]]; then
    echo "Usage: bash .devcontainer/verify.sh [--smoke]" >&2
    exit 2
fi
python3 - <<'PY'
import json, os, platform, subprocess, tomllib
from pathlib import Path
def output(*args):
    return subprocess.check_output(args, text=True).strip()
assert os.geteuid() != 0, "Development must run as a non-root user"
assert platform.system() == "Linux"
hosts = {"x86_64": "x86_64-unknown-linux-gnu", "aarch64": "aarch64-unknown-linux-gnu"}
assert platform.machine() in hosts
assert platform.python_version() == "3.12.15"
rust = tomllib.loads(Path("rust-toolchain.toml").read_text())["toolchain"]
engines = json.loads(Path("package.json").read_text())["engines"]
rust_info = output("rustc", "-vV")
assert output("rustc", "--version").split()[1] == rust["channel"]
assert "host: " + hosts[platform.machine()] in rust_info.splitlines()
assert output("node", "--version") == "v" + engines["node"]
assert output("npm", "--version") == engines["npm"]
assert "wasm32-unknown-unknown" in output("rustup", "target", "list", "--installed").splitlines()
output("cargo", "fmt", "--version")
output("cargo", "clippy", "--version")
print("Native platform:", platform.system(), platform.machine())
print(rust_info)
for command in (("python3", "--version"), ("gcc", "--version"), ("g++", "--version"),
                ("git", "--version"), ("node", "--version"), ("npm", "--version")):
    print(output(*command).splitlines()[0])
print("Environment verification: PASS")
PY

if [[ $# -eq 1 ]]; then
    cargo build -p quantos-cli --locked
    python3 - <<'PY'
import json, os, subprocess
from pathlib import Path
target = Path(os.environ.get("CARGO_TARGET_DIR", "target")).resolve()
binary = target / "debug" / "quantos-cli"
config = "fixtures/synthetic/import-config.json"
def run(csv, exit_code):
    p = subprocess.run([str(binary), "import", "--csv", csv, "--config", config],
                       capture_output=True, text=True, check=False)
    assert p.returncode == exit_code, (csv, p.returncode)
    assert p.stderr == ""
    assert p.stdout.endswith("\n")
    return json.loads(p.stdout)
valid = run("fixtures/synthetic/valid-candles.csv", 0)
assert valid["status"] == "success" and valid["stage"] == "normalization"
result = valid["result"]
assert result["schema_version"] == "1" and result["status"] == "success"
assert result["issues"] == []
assert result["metadata"] == {
    "source": "synthetic", "synthetic": True,
    "adjustment_status": "unknown", "redistribution_status": "unknown"
}
values = [(0,10000,10300,9900,10200,10),
          (300000,10200,10400,10100,10300,12),
          (600000,10300,10500,10000,10100,8)]
expected = [dict(zip(("ts_utc_ms","open","high","low","close","volume"), row),
                 symbol="SYNTH", exchange="TEST", interval="5m", source="synthetic")
            for row in values]
assert result["candles"] == expected
bad = run("fixtures/synthetic/invalid-ohlc.csv", 1)
assert bad["status"] == "failure" and bad["stage"] == "normalization"
assert bad["result"]["candles"] == []
issues = bad["result"]["issues"]
assert any(x["code"] == "INVALID_OHLC" and x["severity"] == "error"
           and x["row"] == 2 and x["field"] is None for x in issues)
print("Synthetic native CLI success/rejection smoke: PASS")
PY
fi
