<#
.SYNOPSIS
Sample same-name processes without reading credentials, message data or command lines.
.EXAMPLE
.\tools\Measure-Client.ps1 -ProcessName Discord -Scenario idle -DurationSeconds 600
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory=$true)][ValidatePattern('^[A-Za-z0-9_.-]+$')][string]$ProcessName,
    [ValidatePattern('^[A-Za-z0-9_.-]+$')][string]$Scenario = 'idle',
    [ValidateRange(2,86400)][int]$DurationSeconds = 60,
    [ValidateRange(100,60000)][int]$IntervalMs = 1000,
    [string]$OutputDirectory = '.\captures'
)
$ErrorActionPreference = 'Stop'
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
$stamp = (Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssZ')
$path = Join-Path $OutputDirectory "$ProcessName-$Scenario-$stamp.csv"
$machine = Get-CimInstance Win32_ComputerSystem
$logical = [int]$machine.NumberOfLogicalProcessors
$clock = [System.Diagnostics.Stopwatch]::StartNew()
$previous = @{}
$lastTime = 0.0
$first = $true
while ($clock.Elapsed.TotalSeconds -lt $DurationSeconds) {
    $now = $clock.Elapsed.TotalSeconds
    $processes = @(Get-Process -Name $ProcessName -ErrorAction SilentlyContinue)
    $current = @{}
    $cpuDelta = 0.0
    [long]$privateBytes = 0
    [long]$workingSet = 0
    [int]$readable = 0
    foreach ($proc in $processes) {
        try {
            $key = "$($proc.Id):$($proc.StartTime.ToUniversalTime().Ticks)"
            $cpu = $proc.TotalProcessorTime.TotalSeconds
            if ($previous.ContainsKey($key)) { $cpuDelta += [Math]::Max(0, $cpu - $previous[$key]) }
            $current[$key] = $cpu
            $privateBytes += $proc.PrivateMemorySize64
            $workingSet += $proc.WorkingSet64
            $readable++
        } catch { } # Processes can exit between enumeration and sample.
    }
    $seconds = $now - $lastTime
    $oneCore = if (!$first -and $seconds -gt 0) { 100.0 * $cpuDelta / $seconds } else { $null }
    [pscustomobject]@{
        timestamp_utc = (Get-Date).ToUniversalTime().ToString('o')
        elapsed_seconds = $now
        scenario = $Scenario
        process_name = $ProcessName
        process_count = $readable
        private_bytes = $privateBytes
        working_set_bytes_sum = $workingSet
        CPU_one_core_percent = $oneCore
        CPU_machine_percent = if ($null -ne $oneCore) { $oneCore / $logical } else { $null }
        logical_processors = $logical
    } | Export-Csv -NoTypeInformation -Append -Path $path
    $first = $false
    $previous = $current
    $lastTime = $now
    Start-Sleep -Milliseconds $IntervalMs
}
$os = Get-CimInstance Win32_OperatingSystem
[pscustomobject]@{
    schema_version = 1
    utc_created = $stamp
    os_caption = $os.Caption
    os_version = $os.Version
    os_build = $os.BuildNumber
    logical_processors = $logical
    ram_bytes = $machine.TotalPhysicalMemory
    process_name = $ProcessName
    scenario = $Scenario
    cpu_note = 'CPU excludes each process first observed sample and work by processes that exit between samples. Best for stable steady-state workloads; use ETW for startup.'
    memory_note = 'Private bytes is committed private memory, not physical residency. Summed working sets include shared-page duplication. GPU and differently named helper processes excluded.'
    complete_manually = @('app_version','app_commit','GPU_driver','DPI','display_refresh','power_plan','workload_fixture')
} | ConvertTo-Json -Depth 3 | Set-Content -Encoding UTF8 -Path ($path + '.metadata.json')
Write-Output $path
