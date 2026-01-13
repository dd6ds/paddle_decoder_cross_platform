# Build script for Windows PowerShell

Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  Building Paddle Decoder for Windows     ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Check Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust not found!" -ForegroundColor Red
    Write-Host "   Install from: https://rustup.rs" -ForegroundColor Yellow
    exit 1
}

$rustVersion = rustc --version
Write-Host "✓ Rust found: $rustVersion" -ForegroundColor Green
Write-Host ""

# Build
Write-Host "Building release version..." -ForegroundColor Yellow
Write-Host ""
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║  ✅ Build successful!                     ║" -ForegroundColor Green
    Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    Write-Host "Executable location: target\release\paddle_decoder.exe" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "To run:" -ForegroundColor Yellow
    Write-Host "  cargo run --release" -ForegroundColor White
    Write-Host "  OR" -ForegroundColor White
    Write-Host "  .\target\release\paddle_decoder.exe" -ForegroundColor White
    Write-Host "  OR" -ForegroundColor White
    Write-Host "  Double-click paddle_decoder.exe in File Explorer" -ForegroundColor White
    Write-Host ""
} else {
    Write-Host ""
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
