[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [string] $Source,
    [string] $Connection = 'arkhe-runtime',
    [string] $CargoRegistry = (Join-Path $env:USERPROFILE '.cargo/registry'),
    [ValidatePattern('^[0-9a-f]{64}$')]
    [string] $ImageId = 'b6af8c19e4ac9303d54a74f965b4f19c834a24c9c56fd815dd553470c9a1fc57',
    [switch] $BuildWasm,
    [string] $WasmOutput
)

$ErrorActionPreference = 'Stop'

function ConvertTo-WslMountPath([string] $Path) {
    $absolute = (Resolve-Path -LiteralPath $Path).ProviderPath
    if ($absolute -notmatch '^([A-Za-z]):[\\/](.+)$' -or $absolute -match '[,\x00-\x1f]') {
        throw 'Mount paths must be on a local Windows drive and contain no commas or control characters.'
    }
    $drive = $Matches[1].ToLowerInvariant()
    $relative = $Matches[2] -replace '\\', '/'
    return "/mnt/$drive/$relative"
}

$root = git -C $Source rev-parse --show-toplevel
if ($LASTEXITCODE -ne 0) { throw 'Source must be an Arkhe Git checkout.' }
$changes = git -C $root status --porcelain --untracked-files=no
if ($LASTEXITCODE -ne 0 -or $changes) { throw 'Use a checkout with clean tracked files.' }
$commit = git -C $root rev-parse HEAD
if ($LASTEXITCODE -ne 0) { throw 'Could not resolve source commit.' }

$toolchain = Get-Content -Raw -LiteralPath (Join-Path $root 'rust-toolchain.toml')
$channel = [regex]::Match($toolchain, '(?m)^\s*channel\s*=\s*"([0-9]+\.[0-9]+\.[0-9]+)"')
if (!$channel.Success) { throw 'Expected a pinned numeric Rust toolchain in rust-toolchain.toml.' }
$rust = $channel.Groups[1].Value
$sourceMount = ConvertTo-WslMountPath $root
$registryMount = ConvertTo-WslMountPath $CargoRegistry
$wasm = [int]$BuildWasm.IsPresent

Write-Output "Arkhe commit: $commit"
Write-Output "Podman image: $ImageId"
$script = @'
set -eu
mkdir -p /scratch/cargo/registry
ln -s /registry/cache /registry/index /scratch/cargo/registry/
export CARGO_HOME=/scratch/cargo CARGO_TARGET_DIR=/scratch/target CARGO_BUILD_JOBS=2
rustc --version
cargo --version
if [ "$ARKHE_BUILD_WASM" = 1 ]; then
    rustup target list --installed | grep -Fx wasm32-unknown-unknown || {
        echo 'BuildWasm requires wasm32-unknown-unknown preinstalled in the image.' >&2
        exit 1
    }
fi
cargo test --locked --offline -p arkhe-verify-core -p arkhe-verify-cli -p arkhe-verify-wasm
cargo build --locked --offline -p arkhe-verify-cli --bin arkhe-verify
/scratch/target/debug/arkhe-verify --help
cargo run --locked --offline --quiet -p arkhe-verify-core --example afir_retrieval_vectors > /scratch/afir-retrieval-v1.json
cmp /scratch/afir-retrieval-v1.json verification/vectors/afir/afir-retrieval-v1.json
sha256sum /scratch/afir-retrieval-v1.json
if [ "$ARKHE_BUILD_WASM" = 1 ]; then
    cargo build --locked --offline -p arkhe-verify-wasm --target wasm32-unknown-unknown --release
    sha256sum /scratch/target/wasm32-unknown-unknown/release/arkhe_verify_wasm.wasm
    if [ -n "${ARKHE_WASM_OUTPUT:-}" ] && [ -d "/wasm-out" ]; then
        cp /scratch/target/wasm32-unknown-unknown/release/arkhe_verify_wasm.wasm /wasm-out/
    fi
fi
'@

$arguments = @(
    '--connection', $Connection, 'run', '--rm', '--pull=never',
    '--network=none', '--read-only', '--cap-drop=ALL',
    '--security-opt=no-new-privileges', '--timeout=600',
    '--env', "RUSTUP_TOOLCHAIN=$rust-x86_64-unknown-linux-gnu",
    '--env', "ARKHE_BUILD_WASM=$wasm",
    '--mount', "type=bind,source=$sourceMount,target=/src,readonly",
    '--mount', "type=bind,source=$registryMount,target=/registry,readonly",
    '--mount', 'type=volume,destination=/scratch'
)

if ($WasmOutput) {
    if (!(Test-Path -LiteralPath $WasmOutput)) {
        New-Item -ItemType Directory -LiteralPath $WasmOutput | Out-Null
    }
    $wasmOutMount = ConvertTo-WslMountPath $WasmOutput
    $arguments += @(
        '--env', 'ARKHE_WASM_OUTPUT=1',
        '--mount', "type=bind,source=$wasmOutMount,target=/wasm-out"
    )
}

$arguments += @(
    '--workdir', '/src', '--entrypoint', '/bin/sh', $ImageId,
    '-ec', ($script -replace "`r`n", "`n")
)

podman @arguments
if ($LASTEXITCODE -ne 0) { throw "Arkhe container verification failed (exit $LASTEXITCODE)." }
