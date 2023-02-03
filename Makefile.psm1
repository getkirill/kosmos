# IS THAT A RIT REFERENCE?!?!?!?!?!?!?!!??!

function Write-Targets {
  if ($null -eq $Arch) {
    Write-Host "List of targets:"
    rustc --print target-list
    return
  }
}

function Build-Executable([string]$Target="x86_64-pc-windows-msvc") {
  cargo build --target $Target
}

function Start-Executable([string]$Target="x86_64-pc-windows-msvc") {
  cargo run --target $Target
}