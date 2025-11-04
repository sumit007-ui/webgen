#!/bin/bash

# GitHub Repository Setup Script for WebGen
# Run this script to create and push to your GitHub repository

echo "🚀 WebGen GitHub Repository Setup"
echo "================================="
echo ""

# Check if gh is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed!"
    echo "📥 Please install it from: https://cli.github.com/"
    exit 1
fi

# Check authentication
echo "🔐 Checking GitHub authentication..."
if ! gh auth status &> /dev/null; then
    echo "❌ You are not authenticated with GitHub"
    echo "🔑 Starting authentication process..."
    echo ""
    gh auth login
    if [ $? -ne 0 ]; then
        echo "❌ Authentication failed!"
        exit 1
    fi
fi

echo "✅ Authenticated successfully!"
echo ""

# Create repository
echo "📦 Creating public repository 'webgen'..."
gh repo create sumit007-ui/webgen \
    --public \
    --source=. \
    --description "A powerful CLI tool to generate pre-built website templates for various tech stacks" \
    --push

if [ $? -ne 0 ]; then
    echo "❌ Repository creation failed!"
    echo "💡 The repository might already exist. Trying to push to existing repository..."
    
    # Check if remote exists
    if ! git remote get-url origin &> /dev/null; then
        # Add remote
        git remote add origin https://github.com/sumit007-ui/webgen.git
    fi
    
    # Push to repository
    git branch -M main
    git push -u origin main
    
    if [ $? -ne 0 ]; then
        echo "❌ Push failed!"
        exit 1
    fi
fi

echo ""
echo "✅ Repository created and pushed successfully!"
echo ""
echo "🌐 Repository URL: https://github.com/sumit007-ui/webgen"
echo ""
echo "📝 Next steps:"
echo "  1. Visit your repository: gh repo view --web"
echo "  2. Add topics: rust, cli, template-generator, web-templates, portfolio"
echo "  3. Enable Discussions and Issues in Settings"
echo "  4. Star your repository! ⭐"
echo ""

# Ask if user wants to open in browser
read -p "Would you like to open the repository in your browser? (Y/n) " openBrowser
if [[ $openBrowser == "" || $openBrowser == "Y" || $openBrowser == "y" ]]; then
    gh repo view --web
fi

echo ""
echo "🎉 All done! Happy coding!"
