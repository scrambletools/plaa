# Packages a Windows build of plaa: the MSI installer and a portable zip,
# each with the DLLs plaa.exe loads beside it, FFmpeg's and libass's with
# what they need, and their licenses.
#
#   packaging\windows\build.ps1 -Version 0.1.0 -Source dist -Out out [-Arch arm64] [-Icon plaa.ico]
#
# Source holds plaa.exe, the DLLs, FFmpeg's LICENSE.txt, and the other
# libraries' licenses in licenses\*.txt (libass.ps1). Needs WiX 5
# and its UI and Util extensions:
#   dotnet tool install --global wix --version 5.0.2
#   wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2
param(
    [Parameter(Mandatory)] [string] $Version,
    [Parameter(Mandatory)] [string] $Source,
    [Parameter(Mandatory)] [string] $Out,
    [ValidateSet('x64', 'arm64')] [string] $Arch = 'x64',
    [string] $Icon = ''
)
$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$root = Resolve-Path (Join-Path $here '..\..')
$Source = (Resolve-Path $Source).Path
New-Item -ItemType Directory -Force $Out | Out-Null

# The DLLs, in one component: their names change with FFmpeg's versions,
# and a major upgrade removes the old ones before the new go in.
$dlls = @(Get-ChildItem (Join-Path $Source '*.dll'))
if ($dlls.Count -eq 0) { throw "no DLLs in $Source" }
$licenses = @(Get-ChildItem (Join-Path $Source 'licenses\*.txt') -ErrorAction SilentlyContinue)
$files = (@($dlls) + @($licenses) | ForEach-Object { "        <File Source=`"$($_.FullName)`" />" }) -join "`n"
$libraries = Join-Path ([System.IO.Path]::GetTempPath()) "plaa-libraries-$PID.wxs"
@"
<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
  <Fragment>
    <ComponentGroup Id="Libraries" Directory="INSTALLFOLDER">
      <Component Id="Libraries" Guid="7f3c9b2e-1d48-4a6f-b5e0-2c9a8d4f6e17">
$files
        <File Id="FFmpegLicense" Source="$(Join-Path $Source 'LICENSE.txt')" Name="FFmpeg-LICENSE.txt" />
        <RegistryValue Root="HKCU" Key="Software\Scramble Tools\plaa" Name="Libraries" Type="integer" Value="1" KeyPath="yes" />
      </Component>
    </ComponentGroup>
  </Fragment>
</Wix>
"@ | Set-Content -Encoding UTF8 $libraries

$msi = Join-Path $Out "plaa-$Version-$Arch.msi"
$defines = @('-d', "Version=$Version", '-d', "Source=$Source")
if ($Icon) { $defines += @('-d', "Icon=$((Resolve-Path $Icon).Path)") }
wix build (Join-Path $here 'plaa.wxs') $libraries -arch $Arch -ext WixToolset.UI.wixext -ext WixToolset.Util.wixext @defines -o $msi
if ($LASTEXITCODE -ne 0) { throw "wix build failed" }
Remove-Item $libraries
# WiX's debug symbols are no use to people installing plaa.
Remove-Item (Join-Path $Out '*.wixpdb') -ErrorAction SilentlyContinue

$zip = Join-Path $Out "plaa-$Version-$Arch-windows.zip"
$stage = Join-Path ([System.IO.Path]::GetTempPath()) "plaa-$Version-$Arch"
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory $stage | Out-Null
Copy-Item (Join-Path $Source 'plaa.exe') $stage
$dlls | Copy-Item -Destination $stage
$licenses | Copy-Item -Destination $stage
Copy-Item (Join-Path $Source 'LICENSE.txt') (Join-Path $stage 'FFmpeg-LICENSE.txt')
Copy-Item (Join-Path $root 'LICENSE'), (Join-Path $root 'README.md') $stage
New-Item -ItemType Directory (Join-Path $stage 'fonts') | Out-Null
Copy-Item (Join-Path $root 'packaging\licenses\*') (Join-Path $stage 'fonts')
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $zip -Force

Get-ChildItem $Out
