# Quick Start Guide 🚀

Get started with WebGen in under 5 minutes!

## 1. Build the Tool

```bash
cargo build --release
```

The compiled binary will be at `target/release/webgen` (or `webgen.exe` on Windows).

## 2. Run Your First Command

### Interactive Mode (Easiest)

```bash
./target/release/webgen
```

Just follow the prompts! You'll be asked:

1. What to name your project
2. Which template to use
3. Where to create it

### Direct Command

```bash
./target/release/webgen new my-portfolio --template react
```

## 3. Explore Your New Project

```bash
cd my-portfolio
cat README.md  # Read the project-specific instructions
```

## Templates Overview

### 🐍 Flask (Python)

**Best for:** Backend developers, Python enthusiasts

```bash
webgen new my-site -t flask
cd my-site
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python app.py
```

Visit: http://localhost:5000

### ⚛️ React (JavaScript)

**Best for:** Modern frontend, single-page apps

```bash
webgen new my-site -t react
cd my-site
npm install
npm start
```

Visit: http://localhost:3000

### 🔥 MERN Stack

**Best for:** Full-stack applications

```bash
webgen new my-app -t mern
cd my-app

# Terminal 1: Backend
cd backend && npm install && npm start

# Terminal 2: Frontend
cd frontend && npm install && npm start
```

### ⚡ Next.js

**Best for:** SEO-optimized sites, SSR

```bash
webgen new my-site -t nextjs
cd my-site
npm install
npm run dev
```

Visit: http://localhost:3000

### 💚 Vue

**Best for:** Progressive web apps

```bash
webgen new my-site -t vue
cd my-site
npm install
npm run dev
```

Visit: http://localhost:5173

### 📄 Static HTML

**Best for:** Simple websites, learning

```bash
webgen new my-site -t static-html
cd my-site
# Just open index.html in your browser!
```

## 4. Make It Global (Optional)

### Windows

```powershell
# Add to PATH or copy to a directory in PATH
Copy-Item target/release/webgen.exe C:/Windows/System32/
```

### Linux/Mac

```bash
sudo cp target/release/webgen /usr/local/bin/
```

Now use it from anywhere:

```bash
webgen new cool-project -t react
```

## 5. Customize Your Site

Each generated project includes:

- ✅ Ready-to-run code
- ✅ Project-specific README
- ✅ All necessary dependencies
- ✅ Modern, responsive design
- ✅ Best practices

Just edit the files and make it yours!

## Common Commands

```bash
# List all templates
webgen list

# Create with specific output directory
webgen new my-site -t react -o ~/projects

# Get help
webgen --help
webgen new --help

# Check version
webgen --version
```

## Troubleshooting

### "cargo: command not found"

Install Rust from: https://rustup.rs/

### "npm: command not found" (for JS templates)

Install Node.js from: https://nodejs.org/

### "python: command not found" (for Flask)

Install Python from: https://python.org/

### Build errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Examples

### Portfolio for a Developer

```bash
webgen new john-portfolio -t react
cd john-portfolio
# Edit src/App.js with your information
npm install && npm start
```

### Blog Platform

```bash
webgen new my-blog -t nextjs
cd my-blog
# Add your blog posts in the content folder
npm install && npm run dev
```

### Full-Stack App

```bash
webgen new todo-app -t mern
cd todo-app
# Customize backend/models and frontend/src
cd backend && npm i && npm start &
cd frontend && npm i && npm start
```

## Next Steps

1. ⭐ Star the repository: https://github.com/sumit007-ui/webgen
2. 📖 Read the full [README.md](README.md)
3. 🐛 Report issues: https://github.com/sumit007-ui/webgen/issues
4. 💡 Request features: https://github.com/sumit007-ui/webgen/discussions
5. 🤝 Contribute: Fork and submit PRs!

## Need Help?

- 📚 Check the [README.md](README.md)
- 💬 Ask in [Discussions](https://github.com/sumit007-ui/webgen/discussions)
- 🐛 Report bugs in [Issues](https://github.com/sumit007-ui/webgen/issues)

---

Happy building! 🎉
