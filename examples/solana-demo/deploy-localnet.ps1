# PowerShell helper to deploy anchor program to localnet
set -e
cd ../../cargo-workspace
anchor build
anchor deploy --provider.cluster localnet
