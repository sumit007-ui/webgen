# GitHub Repository Setup Script for WebGen
# Run this script to create and push to your GitHub repository

Write-Host "🚀 WebGen GitHub Repository Setup" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Check if gh is installed
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ GitHub CLI (gh) is not installed!" -ForegroundColor Red
    Write-Host "📥 Please install it from: https://cli.github.com/" -ForegroundColor Yellow
    exit 1
}

# Check authentication
Write-Host "🔐 Checking GitHub authentication..." -ForegroundColor Yellow
$authStatus = gh auth status 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ You are not authenticated with GitHub" -ForegroundColor Red
    Write-Host "🔑 Starting authentication process..." -ForegroundColor Yellow
    Write-Host ""
    gh auth login
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Authentication failed!" -ForegroundColor Red
        exit 1
    }
}

Write-Host "✅ Authenticated successfully!" -ForegroundColor Green
Write-Host ""

# Create repository
Write-Host "📦 Creating public repository 'webgen'..." -ForegroundColor Yellow
gh repo create sumit007-ui/webgen `
    --public `
    --source=. `
    --description "A powerful CLI tool to generate pre-built website templates for various tech stacks" `
    --push

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Repository creation failed!" -ForegroundColor Red
    Write-Host "💡 The repository might already exist. Trying to push to existing repository..." -ForegroundColor Yellow
    
    # Check if remote exists
    $remoteExists = git remote get-url origin 2>&1
    if ($LASTEXITCODE -ne 0) {
        # Add remote
        git remote add origin https://github.com/sumit007-ui/webgen.git
    }
    
    # Push to repository
    git branch -M main
    git push -u origin main
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Push failed!" -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
Write-Host "✅ Repository created and pushed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "🌐 Repository URL: https://github.com/sumit007-ui/webgen" -ForegroundColor Cyan
Write-Host ""
Write-Host "📝 Next steps:" -ForegroundColor Yellow
Write-Host "  1. Visit your repository: gh repo view --web" -ForegroundColor White
Write-Host "  2. Add topics: rust, cli, template-generator, web-templates, portfolio" -ForegroundColor White
Write-Host "  3. Enable Discussions and Issues in Settings" -ForegroundColor White
Write-Host "  4. Star your repository! ⭐" -ForegroundColor White
Write-Host ""

# Ask if user wants to open in browser
$openBrowser = Read-Host "Would you like to open the repository in your browser? (Y/n)"
if ($openBrowser -eq "" -or $openBrowser -eq "Y" -or $openBrowser -eq "y") {
    gh repo view --web
}

Write-Host ""
Write-Host "🎉 All done! Happy coding!" -ForegroundColor Green
