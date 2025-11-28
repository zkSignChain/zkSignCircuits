<#
PowerShell helper: copy wasm pkg from Rust crate to TS package
Usage: ./scripts/copy-wasm.ps1
#>
$src = "./cargo-workspace/crates/zksign-wasm/pkg"
$dst = "./packages/zk-prover/wasm"
if (Test-Path $src) {
  Remove-Item -Recurse -Force $dst -ErrorAction SilentlyContinue
  Copy-Item -Recurse $src $dst
  Write-Host "Copied wasm pkg to $dst"
} else {
  Write-Error "Source pkg not found. Build wasm first with wasm-pack."
}
