param([string]$Python='python', [switch]$InstallNodeDependencies)
$ErrorActionPreference='Stop'
$taskDirectory=$PSScriptRoot
$taskRepo=[IO.Path]::GetFullPath((Join-Path $taskDirectory '..'))
$reportDirectory=Join-Path $taskDirectory 'test-results'
New-Item -ItemType Directory -Path $reportDirectory -Force | Out-Null
$stages=[Collections.Generic.List[object]]::new()
function Invoke-Stage {
    param([string]$Name,[string]$Program,[string[]]$CommandArguments,[string]$Directory)
    Push-Location -LiteralPath $Directory
    try {
        Write-Host "Running $Name"
        $output=& $Program @CommandArguments 2>&1 | ForEach-Object { "$_" }
        $code=$LASTEXITCODE
        $output | Set-Content -LiteralPath (Join-Path $reportDirectory "$Name.log") -Encoding utf8
        $output | ForEach-Object { Write-Host $_ }
        $stages.Add([ordered]@{name=$Name;exit_code=$code;log="$Name.log"})
        if ($code -ne 0) { throw "$Name failed: $code" }
    } finally { Pop-Location }
}
$success=$false
try {
    Invoke-Stage 'node-version' 'node' @('--version') $taskDirectory
    Invoke-Stage 'npm-version' 'npm.cmd' @('--version') $taskDirectory
    if ($InstallNodeDependencies) {
        Invoke-Stage 'node-install' 'npm.cmd' @('ci','--ignore-scripts','--no-audit','--no-fund') $taskDirectory
    }
    Invoke-Stage 'python-reference' $Python @('arkhe/python/reference_vectors.py') $taskRepo
    Invoke-Stage 'rust-tests' 'cargo' @('+1.96.0','test','--manifest-path','arkhe/protocol/Cargo.toml','--features','rbb-pqc','--locked') $taskRepo
    Invoke-Stage 'fixture-build' 'cargo' @('+1.96.0','build','--manifest-path','arkhe/protocol/Cargo.toml','--example','rbb_sign_fixture','--features','rbb-pqc','--locked') $taskRepo
    Invoke-Stage 'evm-tests' 'npm.cmd' @('test') $taskDirectory
    Invoke-Stage 'clippy' 'cargo' @('+1.96.0','clippy','--manifest-path','arkhe/protocol/Cargo.toml','--features','rbb-pqc','--lib','--tests','--examples','--locked','--','-D','warnings') $taskRepo
    $success=$true
} finally {
    [ordered]@{completed_utc=[DateTimeOffset]::UtcNow.ToString('o');success=$success;scope='Standalone fork: local protocol and EVM tests only';stages=$stages.ToArray()} |
        ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $reportDirectory 'results.json') -Encoding utf8
}
