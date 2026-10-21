# Installs plaa's MSI on this Windows and checks it as someone installing
# it would meet it: DLLs that only Windows itself provides or the MSI
# brings, what the installer sets up, the program and the FFmpeg it
# finds, the window, a video played in it, and what uninstalling leaves
# behind. It installs for the current user, replacing any plaa installed,
# and removes it at the end.
#
#   packaging\tests\windows\test-msi.ps1 -Msi plaa-0.2.0-x64.msi [-Previous plaa-0.1.0-x64.msi] [-NoWindow]
#
# Previous, an older MSI, goes in first, so Msi upgrades it. NoWindow
# leaves out the window and the video, for a Windows without a desktop,
# such as a container. Runs in Windows PowerShell 5.1 too, as that is all
# a clean Windows has.
param(
    [Parameter(Mandatory)] [string] $Msi,
    [string] $Previous = '',
    [switch] $NoWindow
)
$ErrorActionPreference = 'Stop'

# The DLLs Windows 10 and 11 themselves provide that plaa and FFmpeg may
# use; the API sets (api-ms-win-*) are Windows' too, the C runtime's
# among them. A DLL outside this list, such as the Visual C++ runtime's
# VCRUNTIME140.dll, is missing on a clean Windows, so it must be linked
# in or installed beside plaa. Add one here only once sure every Windows
# plaa supports has it.
$SystemDlls = @(
    'advapi32.dll', 'avicap32.dll', 'avrt.dll', 'bcrypt.dll', 'bcryptprimitives.dll',
    'cfgmgr32.dll', 'combase.dll', 'comctl32.dll', 'comdlg32.dll', 'crypt32.dll', 'd2d1.dll',
    'd3d11.dll', 'd3d12.dll', 'd3dcompiler_47.dll', 'dnsapi.dll', 'dwmapi.dll', 'dwrite.dll',
    'dxgi.dll', 'gdi32.dll', 'imm32.dll', 'iphlpapi.dll', 'kernel32.dll', 'kernelbase.dll',
    'mmdevapi.dll', 'msimg32.dll', 'ncrypt.dll', 'ntdll.dll', 'ole32.dll', 'oleaut32.dll',
    'opengl32.dll', 'propsys.dll', 'rpcrt4.dll', 'secur32.dll', 'shcore.dll', 'shell32.dll',
    'shlwapi.dll', 'user32.dll', 'userenv.dll', 'usp10.dll', 'uxtheme.dll', 'version.dll',
    'winmm.dll', 'ws2_32.dll'
)

function Section([string] $Text) { Write-Host "`n== $Text" }
function Fail([string] $Text) { throw "FAIL: $Text" }

# The DLL names a PE file imports, at load and delay-loaded.
function Get-Imports([string] $Path) {
    $bytes = [System.IO.File]::ReadAllBytes($Path)
    $pe = [BitConverter]::ToInt32($bytes, 0x3c)
    $sections = [BitConverter]::ToUInt16($bytes, $pe + 6)
    $optional = $pe + 24
    $optionalSize = [BitConverter]::ToUInt16($bytes, $pe + 20)
    $magic = [BitConverter]::ToUInt16($bytes, $optional)
    $directories = $optional + $(if ($magic -eq 0x20b) { 112 } else { 96 })
    $table = $optional + $optionalSize
    $offset = {
        param([uint32] $Rva)
        for ($index = 0; $index -lt $sections; $index++) {
            $header = $table + 40 * $index
            $address = [BitConverter]::ToUInt32($bytes, $header + 12)
            $size = [Math]::Max([BitConverter]::ToUInt32($bytes, $header + 8), [BitConverter]::ToUInt32($bytes, $header + 16))
            if ($Rva -ge $address -and $Rva -lt $address + $size) {
                return $Rva - $address + [BitConverter]::ToUInt32($bytes, $header + 20)
            }
        }
        throw "$Path has an address outside its sections"
    }
    $text = {
        param([uint32] $Rva)
        $start = & $offset $Rva
        $end = [Array]::IndexOf($bytes, [byte] 0, [int] $start)
        [System.Text.Encoding]::ASCII.GetString($bytes, $start, $end - $start)
    }
    $names = @()
    # Each import descriptor is 20 bytes, the name's address at 12; each
    # delay-load one is 32, the name's at 4.
    foreach ($kind in @(@{ Directory = 1; Size = 20; Name = 12 }, @{ Directory = 13; Size = 32; Name = 4 })) {
        $rva = [BitConverter]::ToUInt32($bytes, $directories + 8 * $kind.Directory)
        if ($rva -eq 0) { continue }
        $descriptor = & $offset $rva
        while ($true) {
            $name = [BitConverter]::ToUInt32($bytes, $descriptor + $kind.Name)
            if ($name -eq 0) { break }
            $names += (& $text $name).ToLowerInvariant()
            $descriptor += $kind.Size
        }
    }
    $names | Sort-Object -Unique
}

function Get-MsiProperty([string] $Path, [string] $Name) {
    $installer = New-Object -ComObject WindowsInstaller.Installer
    $database = $installer.GetType().InvokeMember('OpenDatabase', 'InvokeMethod', $null, $installer, @($Path, 0))
    $view = $database.GetType().InvokeMember('OpenView', 'InvokeMethod', $null, $database,
        @("SELECT Value FROM Property WHERE Property = '$Name'"))
    $view.GetType().InvokeMember('Execute', 'InvokeMethod', $null, $view, $null) | Out-Null
    $record = $view.GetType().InvokeMember('Fetch', 'InvokeMethod', $null, $view, $null)
    $value = $record.GetType().InvokeMember('StringData', 'GetProperty', $null, $record, 1)
    $view.GetType().InvokeMember('Close', 'InvokeMethod', $null, $view, $null) | Out-Null
    foreach ($object in @($record, $view, $database, $installer)) {
        [System.Runtime.InteropServices.Marshal]::ReleaseComObject($object) | Out-Null
    }
    $value
}

function Invoke-Msiexec([string[]] $Arguments, [string] $Log) {
    $run = Start-Process msiexec.exe -ArgumentList ($Arguments + @('/qn', '/l*v', "`"$Log`"")) -Wait -PassThru
    if ($run.ExitCode -notin @(0, 3010)) {
        # The verbose log's lines around the failing action.
        Select-String -Path $Log -Pattern 'Return value 3' -Context 30, 2 | Select-Object -First 1 | Out-Host
        Fail "msiexec $($Arguments -join ' ') ended with $($run.ExitCode)"
    }
}

# plaa's entries in Apps, the installer's own uninstall entries.
function Get-Entries {
    $keys = @(
        'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*'
    )
    @(Get-ItemProperty $keys -ErrorAction SilentlyContinue | Where-Object { $_.DisplayName -eq 'plaa' })
}

# A GUI program's output, which PowerShell neither waits for nor shows.
function Get-Printed([string] $Program, [string] $Argument) {
    $printed = Join-Path $work 'printed.txt'
    $run = Start-Process $Program -ArgumentList $Argument -Wait -PassThru -NoNewWindow -RedirectStandardOutput $printed
    if ($run.ExitCode -ne 0) {
        Fail "$Program $Argument ended with $($run.ExitCode) ($('0x{0:X8}' -f $run.ExitCode))"
    }
    (Get-Content -Raw $printed).Trim()
}

# Waits for plaa's window titled $Title, while plaa runs.
function Wait-Window($Process, [string] $Title) {
    for ($wait = 0; $wait -lt 60 -and -not $Process.HasExited; $wait++) {
        Start-Sleep -Milliseconds 500
        $Process.Refresh()
        if ($Process.MainWindowHandle -ne 0 -and $Process.MainWindowTitle -eq $Title) { return $true }
    }
    $false
}

$Msi = (Resolve-Path $Msi).Path
if ($Previous) { $Previous = (Resolve-Path $Previous).Path }
$work = Join-Path ([System.IO.Path]::GetTempPath()) "plaa-msi-test-$PID"
New-Item -ItemType Directory -Force $work | Out-Null
$version = Get-MsiProperty $Msi 'ProductVersion'
$productCode = Get-MsiProperty $Msi 'ProductCode'
$folder = Join-Path $env:LOCALAPPDATA 'Programs\plaa'
$openWith = 'HKCU:\Software\Classes\Applications\plaa.exe'
$os = Get-CimInstance Win32_OperatingSystem
$architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
Section "$($os.Caption) $($os.Version), $($architecture): plaa $version"

Section 'DLLs the programs need'
$image = Join-Path $work 'image'
Invoke-Msiexec @('/a', "`"$Msi`"", "TARGETDIR=`"$image`"") (Join-Path $work 'extract.log')
$binaries = @(Get-ChildItem $image -Recurse -Include *.exe, *.dll)
if ($binaries.Count -eq 0) { Fail 'the MSI holds no programs' }
$shipped = @($binaries | ForEach-Object { $_.Name.ToLowerInvariant() })
$problems = @()
foreach ($binary in $binaries) {
    $imports = @(Get-Imports $binary.FullName)
    Write-Host "$($binary.Name): $($imports -join ', ')"
    foreach ($import in $imports) {
        if ($import -like 'api-ms-win-*' -or $import -in $SystemDlls -or $import -in $shipped) { continue }
        $problems += "$($binary.Name) needs $import, which Windows does not have"
    }
    # On this Windows, as a container is a clean one.
    foreach ($import in $imports) {
        if ($import -like 'api-ms-win-*' -or $import -in $shipped) { continue }
        if (-not (Test-Path (Join-Path $env:SystemRoot "System32\$import"))) {
            Write-Host "  $import is not on this Windows"
        }
    }
}
if ($problems) { Fail ($problems -join '; ') }

if ($Previous) {
    Section "Install $(Split-Path -Leaf $Previous), to upgrade from"
    Invoke-Msiexec @('/i', "`"$Previous`"") (Join-Path $work 'previous.log')
    $previousCode = @(Get-Entries)[0].PSChildName
}

Section "Install $(Split-Path -Leaf $Msi)"
Invoke-Msiexec @('/i', "`"$Msi`"") (Join-Path $work 'install.log')

Section 'What it installed'
foreach ($file in @('plaa.exe', 'FFmpeg-LICENSE.txt')) {
    if (-not (Test-Path (Join-Path $folder $file))) { Fail "$folder\$file is missing" }
}
if (@(Get-ChildItem (Join-Path $folder 'avcodec-*.dll')).Count -ne 1) { Fail "$folder has no FFmpeg" }
$entries = @(Get-Entries)
if ($entries.Count -ne 1) { Fail "Apps lists plaa $($entries.Count) times" }
if ($entries[0].PSChildName -ne $productCode) { Fail "Apps lists $($entries[0].PSChildName), not $productCode" }
if ($entries[0].DisplayVersion -ne $version) { Fail "Apps lists version $($entries[0].DisplayVersion), not $version" }
if ($entries[0].Publisher -ne 'Scramble Tools') { Fail "Apps names $($entries[0].Publisher) as the publisher" }
if ($Previous -and $previousCode -eq $productCode) { Fail 'the previous version has the same product code' }
$shortcut = Join-Path ([Environment]::GetFolderPath('Programs')) 'plaa.lnk'
if (-not (Test-Path $shortcut)) { Fail 'the Start menu has no plaa' }
$target = (New-Object -ComObject WScript.Shell).CreateShortcut($shortcut).TargetPath
if ($target -ne (Join-Path $folder 'plaa.exe')) { Fail "the Start menu's plaa opens $target" }
$command = (Get-ItemProperty "$openWith\shell\open\command" -ErrorAction SilentlyContinue).'(default)'
if ($command -ne "`"$(Join-Path $folder 'plaa.exe')`" `"%1`"") { Fail "Open With runs '$command'" }
foreach ($extension in @('.mkv', '.mp4', '.webm', '.flac', '.mp3')) {
    if ($null -eq (Get-ItemProperty "$openWith\SupportedTypes" -Name $extension -ErrorAction SilentlyContinue)) {
        Fail "Open With does not offer plaa for $extension"
    }
}
Write-Host 'all there'

# Its version line names the FFmpeg and the libass it found, so their
# DLLs, and those they need, load.
Section 'plaa'
$printed = Get-Printed (Join-Path $folder 'plaa.exe') '--version'
Write-Host $printed
if ($printed -notmatch "^plaa $([regex]::Escape($version)) \(.*\), FFmpeg \d") { Fail "plaa --version printed '$printed'" }
if ($printed -notmatch ', libass \d') { Fail "plaa found no libass for ASS subtitles: '$printed'" }

if (-not $NoWindow) {
    # Plain GDI, as System.Drawing's assemblies differ between Windows
    # PowerShell and each PowerShell 7.
    Add-Type @"
using System;
using System.IO;
using System.Runtime.InteropServices;
public static class Screen {
    [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr window, out RECT rect);
    [DllImport("user32.dll")] static extern bool PrintWindow(IntPtr window, IntPtr context, uint flags);
    [DllImport("gdi32.dll")] static extern IntPtr CreateCompatibleDC(IntPtr context);
    [DllImport("gdi32.dll")] static extern bool DeleteDC(IntPtr context);
    [DllImport("gdi32.dll")] static extern IntPtr CreateDIBSection(IntPtr context, ref BITMAPINFOHEADER info,
        uint usage, out IntPtr bits, IntPtr section, uint offset);
    [DllImport("gdi32.dll")] static extern IntPtr SelectObject(IntPtr context, IntPtr picked);
    [DllImport("gdi32.dll")] static extern bool DeleteObject(IntPtr picked);
    [DllImport("gdi32.dll")] static extern bool GdiFlush();
    [StructLayout(LayoutKind.Sequential)] struct RECT { public int Left, Top, Right, Bottom; }
    [StructLayout(LayoutKind.Sequential)] struct BITMAPINFOHEADER {
        public uint Size; public int Width, Height; public ushort Planes, BitCount;
        public uint Compression, SizeImage; public int XPerMeter, YPerMeter; public uint Used, Important;
    }
    // The window's picture, drawn by the window itself, even where others
    // cover it; the fewest pixels of any of the yellow, cyan and magenta
    // bars of the test clip in it, in hundredths of the picture.
    public static int Bars(IntPtr window, string save) {
        RECT rect;
        GetWindowRect(window, out rect);
        int width = Math.Max(rect.Right - rect.Left, 1), height = Math.Max(rect.Bottom - rect.Top, 1);
        BITMAPINFOHEADER info = new BITMAPINFOHEADER();
        info.Size = (uint)Marshal.SizeOf(info);
        info.Width = width;
        info.Height = -height;
        info.Planes = 1;
        info.BitCount = 32;
        byte[] bytes = new byte[width * height * 4];
        IntPtr context = CreateCompatibleDC(IntPtr.Zero);
        IntPtr bits;
        IntPtr picture = CreateDIBSection(context, ref info, 0, out bits, IntPtr.Zero, 0);
        if (picture == IntPtr.Zero) { DeleteDC(context); return 0; }
        IntPtr previous = SelectObject(context, picture);
        PrintWindow(window, context, 2);
        GdiFlush();
        Marshal.Copy(bits, bytes, 0, bytes.Length);
        SelectObject(context, previous);
        DeleteObject(picture);
        DeleteDC(context);
        if (save != null) Save(save, bytes, width, height);
        int yellow = 0, cyan = 0, magenta = 0;
        for (int at = 0; at + 3 < bytes.Length; at += 4) {
            int blue = bytes[at], green = bytes[at + 1], red = bytes[at + 2];
            if (red > 180 && green > 180 && blue < 80) yellow++;
            else if (red < 80 && green > 180 && blue > 180) cyan++;
            else if (red > 180 && green < 80 && blue > 180) magenta++;
        }
        long least = Math.Min(yellow, Math.Min(cyan, magenta));
        return (int)(least * 10000 / ((long)width * height));
    }
    // As a BMP, its top row first.
    static void Save(string path, byte[] bytes, int width, int height) {
        using (BinaryWriter writer = new BinaryWriter(File.Create(path))) {
            writer.Write((ushort)0x4D42);
            writer.Write(14 + 40 + bytes.Length);
            writer.Write(0);
            writer.Write(14 + 40);
            writer.Write(40);
            writer.Write(width);
            writer.Write(-height);
            writer.Write((ushort)1);
            writer.Write((ushort)32);
            writer.Write(0);
            writer.Write(bytes.Length);
            writer.Write(new byte[16]);
            writer.Write(bytes);
        }
    }
}
"@
    [Screen]::SetProcessDPIAware() | Out-Null

    Section 'The window'
    $process = Start-Process (Join-Path $folder 'plaa.exe') -PassThru
    if (-not (Wait-Window $process 'plaa')) {
        if ($process.HasExited) { Fail "plaa quit before its window opened, with $('0x{0:X8}' -f $process.ExitCode)" }
        Stop-Process -Id $process.Id
        Fail "plaa runs but its window did not open; a session without a desktop needs -NoWindow"
    }
    Start-Sleep -Seconds 5
    if ($process.HasExited) { Fail "plaa quit after opening its window, with $('0x{0:X8}' -f $process.ExitCode)" }
    Write-Host "the window opened: $($process.MainWindowTitle)"
    Stop-Process -Id $process.Id
    $process.WaitForExit()

    # A clip of VP9 and Opus, its test pattern's yellow, cyan and magenta
    # bars in the window once its pictures are drawn.
    Section 'A video, played'
    $clip = Join-Path $work 'test-clip.webm'
    Copy-Item (Join-Path $PSScriptRoot '..\media\test-clip.webm') $clip
    $process = Start-Process (Join-Path $folder 'plaa.exe') -ArgumentList "`"$clip`"" -PassThru
    if (-not (Wait-Window $process 'test-clip.webm')) {
        if ($process.HasExited) { Fail "plaa quit before the video's window opened, with $('0x{0:X8}' -f $process.ExitCode)" }
        Stop-Process -Id $process.Id
        Fail "the video's window did not open"
    }
    $seen = 0
    for ($wait = 0; $wait -lt 20 -and $seen -lt 100; $wait++) {
        Start-Sleep -Milliseconds 500
        $seen = [Screen]::Bars($process.MainWindowHandle, (Join-Path $work 'video.bmp'))
    }
    if ($process.HasExited) { Fail "plaa quit while playing, with $('0x{0:X8}' -f $process.ExitCode)" }
    if ($seen -lt 100) { Fail "the video's pictures did not show; the window is in $work\video.bmp" }
    Write-Host "it played, each bar $($seen / 100)% of the window"
    Stop-Process -Id $process.Id
    $process.WaitForExit()
}

Section 'Uninstall'
Invoke-Msiexec @('/x', $productCode) (Join-Path $work 'uninstall.log')
$leftovers = @()
if (Test-Path $folder) { $leftovers += $folder }
if (Test-Path $shortcut) { $leftovers += $shortcut }
if (Test-Path 'HKCU:\Software\Scramble Tools\plaa') { $leftovers += 'HKCU\Software\Scramble Tools\plaa' }
if (Test-Path $openWith) { $leftovers += 'HKCU\Software\Classes\Applications\plaa.exe' }
if (@(Get-Entries).Count -ne 0) { $leftovers += 'the entry in Apps' }
if ($leftovers) { Fail "uninstalling left $($leftovers -join ', ')" }
Write-Host 'nothing left'

Section "Passed: $(Split-Path -Leaf $Msi)"
Remove-Item -Recurse -Force $work -ErrorAction SilentlyContinue
