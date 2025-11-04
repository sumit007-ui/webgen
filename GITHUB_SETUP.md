# GitHub Repository Setup Instructions

Follow these steps to create your public GitHub repository and push the code:

## Option 1: Using GitHub CLI (Recommended)

### Step 1: Authenticate with GitHub

```bash
gh auth login
```

Follow the prompts:

- Choose: **GitHub.com**
- Choose: **HTTPS**
- Authenticate with your browser or paste a token

### Step 2: Create the Repository

```bash
cd D:/cli/web-template-cli
gh repo create webgen --public --source=. --description "A powerful CLI tool to generate pre-built website templates for various tech stacks" --push
```

### Step 3: View Your Repository

```bash
gh repo view --web
```

## Option 2: Using GitHub Web Interface

### Step 1: Create Repository on GitHub

1. Go to https://github.com/new
2. Fill in the details:
    - **Repository name**: `webgen`
    - **Description**:
      `A powerful CLI tool to generate pre-built website templates for various tech stacks`
    - **Visibility**: Public
    - **DO NOT** initialize with README, .gitignore, or license (we already have these)
3. Click "Create repository"

### Step 2: Push Your Local Repository

```bash
cd D:/cli/web-template-cli
git remote add origin https://github.com/sumit007-ui/webgen.git
git branch -M main
git push -u origin main
```

## Option 3: Quick Commands (if already authenticated)

```bash
cd D:/cli/web-template-cli

# Authenticate (only once)
gh auth login

# Create and push in one command
gh repo create sumit007-ui/webgen --public --source=. --description "A powerful CLI tool to generate pre-built website templates for various tech stacks" --push

# Open in browser
gh repo view --web
```

## Verify Your Repository

After pushing, verify everything is uploaded:

```bash
gh repo view
```

Or visit: https://github.com/sumit007-ui/webgen

## Next Steps

1. **Enable Issues**: Go to Settings → Features → Enable Issues
2. **Add Topics**: Add topics like `rust`, `cli`, `template-generator`, `web-templates`, `portfolio`
3. **Add Description**: Make sure the description is set
4. **Star Your Repo**: Give yourself a star! ⭐
5. **Share**: Share with the community!

## Troubleshooting

### "gh: command not found"

GitHub CLI is not installed. Install it from: https://cli.github.com/

### Authentication Issues

```bash
gh auth refresh
gh auth status
```

### Permission Denied

Make sure you have write access to the repository and are authenticated with the correct account.

---

**Repository Details:**

- **Name**: webgen
- **Owner**: sumit007-ui
- **Visibility**: Public
- **URL**: https://github.com/sumit007-ui/webgen
