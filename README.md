# WebGen - Website Template Generator CLI 🚀

A powerful command-line tool that generates pre-built, production-ready website templates for
various tech stacks. Create beautiful portfolio websites and web applications with a single command!

## ✨ Features

- 🎨 **Multiple Templates**: Flask, MERN Stack, Next.js, React, Vue, and Static HTML
- 🚀 **Quick Setup**: Generate complete projects in seconds
- 💼 **Portfolio Ready**: Professional portfolio templates included
- 🎯 **Interactive Mode**: User-friendly CLI with prompts
- 📦 **Production Ready**: Best practices and modern code structure
- 🔧 **Customizable**: Easy to modify and extend

## 📦 Available Templates

| Template | Description | Tech Stack |
|----------|-------------|------------|
| **Flask Portfolio** | Python-based portfolio with modern UI | Flask, Jinja2, SQLite |
| **MERN Stack** | Full-stack web application | MongoDB, Express, React, Node.js |
| **Next.js App** | Modern React with SSR | Next.js, React, TypeScript |
| **React Portfolio** | Single-page portfolio | React, Tailwind CSS |
| **Vue Portfolio** | Vue.js 3 portfolio | Vue 3, Composition API, Vite |
| **Static HTML** | Pure HTML/CSS/JS | HTML5, CSS3, JavaScript |

## 🚀 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/sumit007-ui/webgen.git
cd webgen

# Build the project
cargo build --release

# The binary will be at target/release/webgen
```

### Add to PATH (Optional)

**Linux/Mac:**

```bash
sudo cp target/release/webgen /usr/local/bin/
```

**Windows:**

```powershell
# Add the target/release directory to your PATH environment variable
```

## 💻 Usage

### Interactive Mode (Recommended)

Simply run the command and follow the prompts:

```bash
webgen
```

### Quick Generation

Generate a project directly:

```bash
# Basic usage
webgen new my-portfolio --template react

# Specify output directory
webgen new my-site --template flask --output ./projects

# Short form
webgen new blog -t nextjs -o ~/websites
```

### List Available Templates

```bash
webgen list
```

## 📚 Template Details

### 1. Flask Portfolio

Perfect for Python developers wanting a portfolio website.

**Features:**

- Contact form with email integration
- Project showcase
- Blog section
- Admin dashboard
- SQLite database
- Responsive design

**Setup:**

```bash
webgen new my-portfolio -t flask
cd my-portfolio
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
pip install -r requirements.txt
python app.py
```

### 2. MERN Stack

Full-stack application with MongoDB, Express, React, and Node.js.

**Features:**

- User authentication
- RESTful API
- React frontend
- MongoDB integration
- JWT tokens
- Redux state management

**Setup:**

```bash
webgen new my-app -t mern
cd my-app

# Backend
cd backend
npm install
npm start

# Frontend (in new terminal)
cd frontend
npm install
npm start
```

### 3. Next.js App

Modern React framework with server-side rendering.

**Features:**

- File-based routing
- API routes
- SSR & SSG support
- TypeScript ready
- Optimized images
- SEO friendly

**Setup:**

```bash
webgen new my-nextapp -t nextjs
cd my-nextapp
npm install
npm run dev
```

### 4. React Portfolio

Beautiful single-page portfolio with React.

**Features:**

- Tailwind CSS styling
- Smooth animations
- Responsive design
- Contact form
- Project gallery
- Dark mode support

**Setup:**

```bash
webgen new portfolio -t react
cd portfolio
npm install
npm start
```

### 5. Vue Portfolio

Vue.js 3 portfolio with Composition API.

**Features:**

- Vue 3 Composition API
- Vite for fast builds
- Vue Router
- Pinia state management
- TypeScript support
- Modern UI components

**Setup:**

```bash
webgen new vue-site -t vue
cd vue-site
npm install
npm run dev
```

### 6. Static HTML

Pure HTML, CSS, and JavaScript template.

**Features:**

- No build process
- Lightweight
- Easy to deploy
- Vanilla JavaScript
- CSS Grid & Flexbox
- Mobile responsive

**Setup:**

```bash
webgen new simple-site -t static-html
cd simple-site
# Open index.html in browser or use a local server
python -m http.server 8000
```

## 🎨 Project Structure

### Flask Template

```
my-portfolio/
├── app.py
├── requirements.txt
├── static/
│   ├── css/
│   ├── js/
│   └── images/
├── templates/
│   ├── base.html
│   ├── index.html
│   └── ...
└── README.md
```

### MERN Template

```
my-app/
├── backend/
│   ├── server.js
│   ├── models/
│   ├── routes/
│   ├── controllers/
│   └── package.json
├── frontend/
│   ├── src/
│   ├── public/
│   └── package.json
└── README.md
```

## 🛠️ Development

### Building from Source

```bash
# Clone the repo
git clone https://github.com/sumit007-ui/webgen.git
cd webgen

# Build
cargo build

# Run in development
cargo run

# Run tests
cargo test
```

### Adding New Templates

1. Create a new template file in `src/templates/`
2. Implement the template structure
3. Add to `templates.rs`
4. Update the CLI options

## 📖 Examples

### Create a Flask Portfolio

```bash
webgen new john-doe-portfolio -t flask -o ~/projects
```

### Create a MERN Application

```bash
webgen new fullstack-app -t mern
```

### Interactive Selection

```bash
webgen
# Follow the prompts to choose template and configure
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Areas for Contribution

- Add new templates (Django, Laravel, Svelte, etc.)
- Improve existing templates
- Add more customization options
- Write documentation
- Report bugs

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with Rust 🦀
- Uses [clap](https://github.com/clap-rs/clap) for CLI
- Uses [dialoguer](https://github.com/console-rs/dialoguer) for interactive prompts
- Inspired by create-react-app, vue-cli, and other scaffolding tools

## 📧 Support

- GitHub Issues: [Report a bug](https://github.com/sumit007-ui/webgen/issues)
- Discussions: [Ask questions](https://github.com/sumit007-ui/webgen/discussions)
- Author: [@sumit007-ui](https://github.com/sumit007-ui)

## 🚀 Roadmap

- [ ] Add Django template
- [ ] Add Laravel template
- [ ] Add Svelte template
- [ ] Add Angular template
- [ ] Add custom theme support
- [ ] Add CI/CD configuration templates
- [ ] Add Docker support
- [ ] Add deployment scripts

---

**Made with ❤️ by [sumit007-ui](https://github.com/sumit007-ui)**

Happy coding! 🎉
