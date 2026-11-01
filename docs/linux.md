# QuantOS Linux development and execution

Linux is an owner-authorized architecture extension, recorded in
[ADR 0003](decisions/0003-linux-development-and-execution.md).
It shares the existing Rust core; public browser backtesting still uses WASM
and requires no backend. [QOS-LINUX-001](tasks/QOS-LINUX-001.md) implements the
first infrastructure milestone, subject to PR review and owner approval.

## Assessed baseline — 2026-11-01

Accepted develop: fcd55862fac170c3326da5850b2b0e9c031c1274 (PR #10).
[Its post-merge CI](https://github.com/kushalshah7/QuantOS/actions/runs/41197152381)
passed the existing Linux Rust/web jobs. The implementation baseline has
107 Rust tests, including native CLI subprocess tests and 39 execution tests.

| Component | Observed state | Remaining Linux work |
| --- | --- | --- |
| CSV parsing, normalization and import CLI | Existing native Linux execution/tests | ARM64 and reproducible container evidence |
| Provided-signal execution kernel | Existing native Linux financial assertions | Same tests on ARM64; no algorithm rewrite |
| SMA generator | PR #11 open at e7a7c542115ad1878de50c4962c52ed9476a9c6a; 132 Rust tests including 25 SMA passed on Ubuntu | Separate owner acceptance; not included in this branch's base |
| React/TypeScript | Placeholder with existing Linux lint/typecheck/test/build | Reproducible Node/npm development tools |
| WASM | Compilation checked; financial browser bindings/parity unimplemented | Preserve compile gate; runtime parity later |
| DuckDB/Parquet, analytics, native backtest command, broker bridge, worker | Planned or unimplemented | Separate contracts/tasks, not Linux-001 capabilities |

PR #11 metadata and CI output were verified; its source review and owner
acceptance remain separate from this infrastructure task. Recorded earlier Mac
checks are not evidence that SMA or this container ran on macOS.

## Bounded milestones

| Milestone | Dependency | Deliverable and acceptance |
| --- | --- | --- |
| L1: QOS-LINUX-001 | Accepted develop; public free runners | Versioned non-root dev container, existing x86 checks, native ARM64 tests, container CLI smoke, reviewed PR |
| L1 interactive acceptance | Green L1 PR plus owner-confirmed free Codespaces quota | Fresh Codespace on reviewed branch; version check and synthetic smoke succeed; stop the Codespace; record SHA/architecture/exits |
| L2: native workflow and portability | Separate accepted P2/CLI work, including SMA if used | Small synthetic native workflow; compare fixed expected decisions/ledger on x86-64 and ARM64; targeted Mac check; separately approved task |
| L3: optional private research worker | Accepted native job interface and measured need | Design bounded queue/state/error/shutdown/resource contracts first; systemd/cgroups/profiling only where measurements justify them |

DuckDB/Parquet and real-data ingestion are not prerequisites for L1. No new
phase is automatically authorized by this plan. No local VM is required.

## Reproducible environment

The Dockerfile uses official Python 3.12.15/bookworm at an immutable OCI index
digest supporting amd64 and arm64. That digest also fixes its included C/C++
compiler/build tools and OS libraries; no floating apt installation is performed.
Exact Rust 1.99.0, Node 24.19.0 and npm 11.17.0 are read from the existing manifests.
Rustup 1.29.2 and Node archives have committed SHA-256 checks; npm has a pinned
SHA-512 integrity check. Toolchain changes must update the checksums deliberately.

Rustfmt, Clippy and wasm32-unknown-unknown are installed. Python is for optional
standard-library verification; no scientific packages or extra language stack is
installed. The non-root quantos user has two Cargo build jobs. Post-create checks
versions only; it does not install application dependencies, run suites, start a
server or launch an agent. Ports are not made public.

The Docker context allows only `.devcontainer/Dockerfile`,
`.devcontainer/install-toolchains.sh`, `.devcontainer/downloads.sha256`,
`rust-toolchain.toml` and `package.json` as COPY inputs. The broad directory
exception identified in PR #12 review is removed; no unlisted descendant of
`.devcontainer/` is intended to be included. Docker separately transmits its
control inputs, which are not additional COPY-visible files. Source, credentials,
local edits, Market-Data/, results and synthetic CSV files remain excluded.
CI mounts the fresh public checkout read-only for the smoke check and
writes compilation output to a disposable container directory. No image/artifact
is published.

## Codespaces: manual, within included usage only

GitHub personal accounts have a finite included Codespaces allowance: GitHub
Free currently includes 120 core-hours and 15 GB-month storage (two-core use
consumes two core-hours per running hour). Storage continues to count while
stopped. Account plan, available quota and billing controls are not verified here.

Before creation, check your Codespaces usage and billing budget. Do not enable
paid overage or add payment details to bypass exhausted quota. If a payment method
already exists, verify Codespaces has a stop-usage budget preventing paid overage.
If that cannot be confirmed, use free public Actions and defer Codespaces.

1. In GitHub, select the reviewed feature branch, then Code -> Codespaces ->
   create with options. Select this dev container and the smallest two-core type.
   Do not enable prebuilds or add repository/account secrets.
2. Confirm the expected SHA with git rev-parse HEAD and run:
   `bash .devcontainer/verify.sh`.
3. For the explicit synthetic native smoke:
   `bash .devcontainer/verify.sh --smoke`.
   Expect non-root Linux identity, pinned tools and both PASS messages.
4. For actual development, run only relevant checks. Frontend dependencies use
   `npm ci --ignore-scripts`; existing npm commands remain unchanged. No
   automatic npm audit fix or dependency upgrade is authorized.
5. Set a short idle timeout (for example 15 minutes), stop the Codespace when
   finished, and delete it only after all intended work has been safely pushed
   or exported. Stopping does not remove storage usage.
6. Use the existing feature-branch/ff-only workflow across laptops. Never upload
   private Market-Data/ to a Codespace in this task.

No Codespace is automatically created by this PR. Interactive provider startup,
editor behavior and browser preview remain NOT RUN until the owner records them.
Building the same Dockerfile in Actions validates the image, not those provider
features. Linux container execution is not macOS-native execution.

## CI and evidence contract

Existing required names rust-quality and web-quality remain unchanged.
Both use ubuntu-24.04; the former runs formatting, Clippy, full Rust tests and
the WASM compile check, while the latter retains the frontend suite.
The explicit x86 host assertion prevents mislabeling a cross build.

New linux-arm64 runs on ubuntu-24.04-arm, asserts Linux/aarch64 and the native
rustc host, and executes the full Rust suite. No QEMU, frontend duplication or
second formatting job. New linux-devcontainer builds the Codespaces Dockerfile,
checks the non-root environment and runs exact valid/rejected CLI imports.
The new jobs run on PRs and pushes to main/develop, not duplicate feature pushes.
All jobs retain contents: read and timeouts; no secrets or paid/larger runners.

The owner-authorized context correction adds an actual BuildKit regression step
inside `linux-devcontainer`. A disposable invented context contains five positive
stubs plus six private-file canaries; a stdin `FROM scratch`/`COPY . /` probe
exports to temporary directories outside that context. Both builds must succeed.
The same exact-set/payload assertion must reject the original rules for including
the three unlisted `.devcontainer/` canaries and pass the checked-in corrected
rules with all six canaries absent. No private checkout contents are used; cleanup
is limited to the probe's temporary root. This is Docker execution evidence,
not a Git-ignore or glob simulation.

At correction checkpoint `c4906a6a98071e57c6479d257877d8d6e77f0181`,
[PR CI 41297639261](https://github.com/kushalshah7/QuantOS/actions/runs/41297639261)
passed all four jobs. Inspected BuildKit logs show both successful exports:
original rules failed the same isolation assertion for the three unlisted
`.devcontainer/` canaries; corrected rules passed with exactly five byte-preserved
files and all six canaries absent. Existing image/environment/CLI smoke also
passed. The task card retains this checkpoint receipt; PR #12 records the fresh
final-head evidence after the documentation handoff. Local Docker/runtime checks
remain NOT RUN; independent re-review is required.

Review requires all four jobs green at the final PR head, even though existing
branch protection names are not changed automatically. Future task checks
must include Linux failures rather than silently skipping an unavailable runner.
GitHub runner labels pin the OS series, not every hosted-runner image patch.

Implementation/final-head results are recorded in this branch's PR description.
Do not infer PASS from the commands above or from older CI. Local Docker/builds,
Mac runtime, interactive Codespaces and browser financial parity are NOT RUN for
this task. Public Actions/container output is the native runtime evidence.

## Risks and operating limits

- Codespaces allowance exhaustion: stop-usage controls and explicit shutdown;
  no promise of unlimited free interactive development.
- Container/tool archive changes: immutable pins and integrity failure, reviewed
  upgrades; the build still needs official registries/toolchain servers.
- ARM runner or registry availability: fail/report, never silently substitute
  cross-compilation or a paid runner.
- Parallel review with PR #11: keep runtime source untouched; recheck final base
  and CI after either merge rather than accepting stale combined evidence.
- Private data: Market-Data/ is ignored and excluded from Docker context; ignore
  rules are not authorization to transfer it to cloud systems.

## Official references checked 2026-11-01

- [Standard runner availability and public-repo pricing](https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/choose-the-runner-for-a-job)
- [Native ARM64 public runners](https://github.blog/changelog/2025-08-07-arm64-hosted-runners-for-public-repositories-are-now-generally-available/)
- [Codespaces billing and quotas](https://docs.github.com/en/billing/concepts/product-billing/github-codespaces)
- [Included-usage guidance](https://docs.github.com/en/codespaces/troubleshooting/troubleshooting-included-usage)
- [Dev-container configuration](https://docs.github.com/en/codespaces/setting-up-your-project-for-codespaces/adding-a-dev-container-configuration/introduction-to-dev-containers)
