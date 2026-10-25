param([string]$App = (Join-Path $PSScriptRoot 'Tangent.exe'))
$ErrorActionPreference = 'Stop'
$App = (Resolve-Path -LiteralPath $App).Path
$folder = Split-Path -Parent $App
$report = Join-Path $folder ('Tangent-test-' + (Get-Date -Format 'yyyyMMdd-HHmmss') + '.txt')
@("Tangent P-45 test: $(Get-Date -Format o)", "OS: $([Environment]::OSVersion.VersionString)", "Binary SHA256: $((Get-FileHash -LiteralPath $App -Algorithm SHA256).Hash)") | Set-Content -LiteralPath $report -Encoding UTF8
& $App --self-test 2>&1 | Tee-Object -FilePath $report -Append
if ($LASTEXITCODE -ne 0) { throw "Program self-test failed. See $report" }
Write-Host 'Follow TEST-P45.md while Tangent is open (about 10 minutes).'
Start-Process -FilePath $App -WorkingDirectory $folder -Wait
foreach ($check in @('Startup', 'Library import/search/edit/sort', 'Restart/reopen', 'P-45 notes/chords/latency', 'MIDI rescan and reconnect', 'Audio/playback performance', 'MIDI output: piano sound/no echo/pause', 'Reading: clefs/levels/hints, solid vs glow', 'Delete/export')) {
    $answer = Read-Host "$check [OK / FAIL + comment]"
    "$check : $answer" | Add-Content -LiteralPath $report -Encoding UTF8
}
Write-Host "Report saved: $report"
