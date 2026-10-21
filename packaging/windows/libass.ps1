# Builds libass and the libraries it needs as DLLs, with vcpkg at the
# commit given, and puts them where plaa.exe loads them: in Out, each
# library's license in Out\licenses, and the source archives vcpkg built
# them from in Sources. The Visual C++ runtime DLLs they use, which a
# clean Windows lacks and Microsoft has apps carry beside them, go to
# Runtime, Out unless given, so they can join the others once those are
# signed, keeping Microsoft's own signatures.
#
#   packaging\windows\libass.ps1 -Arch x64 -Out dist -Sources sources -Commit <vcpkg commit> [-Runtime runtime]
#
# Needs git and Visual Studio's C++ tools, as vcpkg does.
param(
    [ValidateSet('x64', 'arm64')] [string] $Arch = 'x64',
    [Parameter(Mandatory)] [string] $Out,
    [Parameter(Mandatory)] [string] $Sources,
    [Parameter(Mandatory)] [string] $Commit,
    [string] $Runtime = $Out,
    [string] $Vcpkg = (Join-Path ([System.IO.Path]::GetTempPath()) "plaa-vcpkg")
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$triplet = "$Arch-windows-release"
New-Item -ItemType Directory -Force $Out, $Sources, $Runtime, (Join-Path $Out 'licenses') | Out-Null

if (-not (Test-Path (Join-Path $Vcpkg '.git'))) {
    git clone -q https://github.com/microsoft/vcpkg.git $Vcpkg
    if ($LASTEXITCODE -ne 0) { throw 'cloning vcpkg failed' }
}
git -C $Vcpkg fetch -q origin $Commit
git -C $Vcpkg -c advice.detachedHead=false checkout -q $Commit
if ($LASTEXITCODE -ne 0) { throw "vcpkg has no commit $Commit" }
& (Join-Path $Vcpkg 'bootstrap-vcpkg.bat') -disableMetrics | Out-Null
& (Join-Path $Vcpkg 'vcpkg.exe') install "libass:$triplet" --overlay-triplets (Join-Path $here 'triplets')
if ($LASTEXITCODE -ne 0) { throw 'vcpkg could not build libass' }
$installed = Join-Path $Vcpkg "installed\$triplet"
$bin = Join-Path $installed 'bin'

# The DLLs a PE file imports, from its import table.
function Get-Imports([string] $Path) {
    $bytes = [System.IO.File]::ReadAllBytes($Path)
    $pe = [BitConverter]::ToInt32($bytes, 0x3c)
    $sections = [BitConverter]::ToUInt16($bytes, $pe + 6)
    $optional = $pe + 24
    $magic = [BitConverter]::ToUInt16($bytes, $optional)
    $directories = $optional + $(if ($magic -eq 0x20b) { 112 } else { 96 })
    $importRva = [BitConverter]::ToUInt32($bytes, $directories + 8)
    $table = $optional + [BitConverter]::ToUInt16($bytes, $pe + 20)
    function Offset([uint32] $Rva) {
        for ($i = 0; $i -lt $sections; $i++) {
            $section = $table + 40 * $i
            $size = [BitConverter]::ToUInt32($bytes, $section + 8)
            $address = [BitConverter]::ToUInt32($bytes, $section + 12)
            if ($Rva -ge $address -and $Rva -lt $address + $size) {
                return $Rva - $address + [BitConverter]::ToUInt32($bytes, $section + 20)
            }
        }
        throw "$Path has no section for $Rva"
    }
    $names = @()
    if ($importRva -eq 0) { return $names }
    $entry = Offset $importRva
    while ($true) {
        $nameRva = [BitConverter]::ToUInt32($bytes, $entry + 12)
        if ($nameRva -eq 0) { break }
        $start = Offset $nameRva
        $end = [Array]::IndexOf($bytes, [byte] 0, [int] $start)
        $names += [System.Text.Encoding]::ASCII.GetString($bytes, $start, $end - $start)
        $entry += 20
    }
    $names
}

# libass's DLL and those it needs, among vcpkg's, followed through their
# imports; and the Visual C++ runtime DLLs any of them imports.
$wanted = @{}
$runtime = @{}
$queue = [System.Collections.Generic.Queue[string]]::new()
$queue.Enqueue((Get-ChildItem (Join-Path $bin 'ass*.dll') | Select-Object -First 1).Name)
while ($queue.Count -gt 0) {
    $name = $queue.Dequeue()
    if ($wanted.ContainsKey($name.ToLower())) { continue }
    $wanted[$name.ToLower()] = $name
    foreach ($import in Get-Imports (Join-Path $bin $name)) {
        if (Test-Path (Join-Path $bin $import)) { $queue.Enqueue($import) }
        elseif ($import -match '^vcruntime\d+(_\d+)?\.dll$') { $runtime[$import.ToLower()] = $import }
    }
}
$wanted.Values | ForEach-Object { Copy-Item (Join-Path $bin $_) $Out }

$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio\Installer\vswhere.exe'
$redist = & $vswhere -latest -products * -find "VC\Redist\MSVC\*\$Arch\Microsoft.VC*.CRT" | Select-Object -Last 1
if (-not $redist) { throw "no Visual C++ runtime for $Arch" }
$runtime.Values | ForEach-Object { Copy-Item (Join-Path $redist $_) $Runtime }

# Each port a DLL came from: its license, and its source archive.
$ports = @{}
Get-ChildItem (Join-Path $Vcpkg 'installed\vcpkg\info\*.list') | ForEach-Object {
    $port = ($_.BaseName -split '_')[0]
    foreach ($line in Get-Content $_.FullName) {
        if ($line -match "^$triplet/bin/(.+\.dll)$" -and $wanted.ContainsKey($Matches[1].ToLower())) {
            $ports[$port] = $true
        }
    }
}
foreach ($port in $ports.Keys) {
    Copy-Item (Join-Path $installed "share\$port\copyright") (Join-Path $Out "licenses\$port-LICENSE.txt")
    Get-ChildItem (Join-Path $Vcpkg 'downloads') -File | Where-Object { $_.Name -like "*$port*" } |
        Copy-Item -Destination $Sources
}
Write-Host "libass for $Arch`: $(@($wanted.Values) -join ', '); runtime: $(@($runtime.Values) -join ', ')"
Write-Host "licenses and sources: $(@($ports.Keys | Sort-Object) -join ', ')"
