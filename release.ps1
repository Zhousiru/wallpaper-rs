$ErrorActionPreference = "Stop"

$sourcePath = "target/release/wallpaper-rs.exe"
$destPath = "dist/wallpaper-rs.exe"
$daemonDestPath = "dist/wallpaper-rs-daemon.exe"

New-Item -ItemType Directory -Force -Path ./dist | Out-Null

Invoke-Expression "cargo build -r"
if ($LastExitCode -eq 0) {
    Copy-Item -Path $sourcePath -Destination $destPath -Force
} else {
    Throw "Failed to build wallpaper-rs."
}

Invoke-Expression "cargo build -r -F daemon"
if ($LastExitCode -eq 0) {
    Copy-Item -Path $sourcePath -Destination $daemonDestPath -Force
} else {
    Throw "Failed to build wallpaper-rs-daemon."
}
