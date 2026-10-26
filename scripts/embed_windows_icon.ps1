# Embed Tangent's application icon directly in the built Windows PE executable.
# No downloaded tooling is needed; Win32 resource functions are part of Windows.
param(
    [Parameter(Mandatory = $true)][string]$Exe,
    [Parameter(Mandatory = $true)][string]$Icon
)
$ErrorActionPreference = 'Stop'
$exePath = (Resolve-Path -LiteralPath $Exe).Path
$iconPath = (Resolve-Path -LiteralPath $Icon).Path

Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Runtime.InteropServices;

public static class TangentWindowsIcon {
    private const uint LOAD_LIBRARY_AS_DATAFILE = 0x00000002;

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    private static extern IntPtr BeginUpdateResource(string file, bool deleteExisting);

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool UpdateResource(IntPtr handle, IntPtr type, IntPtr name,
                                              ushort language, byte[] data, uint size);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool EndUpdateResource(IntPtr handle, bool discard);

    [DllImport("kernel32.dll", CharSet = CharSet.Unicode, SetLastError = true)]
    private static extern IntPtr LoadLibraryEx(string file, IntPtr reserved, uint flags);

    [DllImport("kernel32.dll", SetLastError = true)]
    private static extern IntPtr FindResource(IntPtr module, IntPtr name, IntPtr type);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    private static extern bool FreeLibrary(IntPtr module);

    private static void Check(bool ok, string operation) {
        if (!ok) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error(), operation);
    }

    public static void Embed(string exe, string icon) {
        byte[] ico = File.ReadAllBytes(icon);
        if (ico.Length < 6 || BitConverter.ToUInt16(ico, 0) != 0 ||
            BitConverter.ToUInt16(ico, 2) != 1) throw new InvalidDataException("Invalid ICO header");
        int count = BitConverter.ToUInt16(ico, 4);
        if (count < 1 || count > 64 || ico.Length < 6 + count * 16)
            throw new InvalidDataException("Invalid ICO directory");

        // GROUP_ICON uses ICO directory entries, with each 4-byte image offset
        // replaced by a 2-byte RT_ICON resource identifier.
        byte[] group = new byte[6 + count * 14];
        Buffer.BlockCopy(ico, 0, group, 0, 6);
        IntPtr update = BeginUpdateResource(exe, false);
        if (update == IntPtr.Zero) Check(false, "BeginUpdateResource");
        bool done = false;
        try {
            for (int i = 0; i < count; ++i) {
                int entry = 6 + 16 * i;
                int size = BitConverter.ToInt32(ico, entry + 8);
                int offset = BitConverter.ToInt32(ico, entry + 12);
                if (size <= 0 || offset < 0 || (long)offset + size > ico.Length)
                    throw new InvalidDataException("Invalid ICO image range");
                byte[] image = new byte[size];
                Buffer.BlockCopy(ico, offset, image, 0, size);
                Check(UpdateResource(update, new IntPtr(3), new IntPtr(i + 1), 0,
                                     image, (uint)size), "UpdateResource(RT_ICON)");
                Buffer.BlockCopy(ico, entry, group, 6 + i * 14, 12);
                Buffer.BlockCopy(BitConverter.GetBytes((ushort)(i + 1)), 0,
                                 group, 6 + i * 14 + 12, 2);
            }
            Check(UpdateResource(update, new IntPtr(14), new IntPtr(1), 0,
                                 group, (uint)group.Length), "UpdateResource(RT_GROUP_ICON)");
            Check(EndUpdateResource(update, false), "EndUpdateResource");
            done = true;
        } finally {
            if (!done) EndUpdateResource(update, true);
        }

        // Verify the resource in the actual PE rather than relying on a command exit.
        IntPtr module = LoadLibraryEx(exe, IntPtr.Zero, LOAD_LIBRARY_AS_DATAFILE);
        if (module == IntPtr.Zero) Check(false, "LoadLibraryEx");
        try {
            if (FindResource(module, new IntPtr(1), new IntPtr(14)) == IntPtr.Zero)
                Check(false, "FindResource(RT_GROUP_ICON)");
        } finally {
            FreeLibrary(module);
        }
    }
}
'@
[TangentWindowsIcon]::Embed($exePath, $iconPath)
Write-Host "Embedded and verified Tangent app icon in $exePath"
