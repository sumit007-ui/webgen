use std::path::Path;
use anyhow::Result;
use crate::generator::write_file;

pub fn generate(project_path: &Path) -> Result<()> {
    write_file(&project_path.join("package.json"), PACKAGE_JSON)?;
    write_file(&project_path.join("next.config.js"), NEXT_CONFIG)?;
    write_file(&project_path.join("pages/_app.js"), APP_JS)?;
    write_file(&project_path.join("pages/index.js"), INDEX_JS)?;
    write_file(&project_path.join("pages/about.js"), ABOUT_JS)?;
    write_file(&project_path.join("pages/api/hello.js"), API_HELLO)?;
    write_file(&project_path.join("styles/globals.css"), GLOBALS_CSS)?;
    write_file(&project_path.join("components/Layout.js"), LAYOUT)?;
    write_file(&project_path.join("public/.gitkeep"), "")?;
    write_file(&project_path.join("README.md"), README)?;
    write_file(&project_path.join(".gitignore"), GITIGNORE)?;
    
    Ok(())
}

const PACKAGE_JSON: &str = r###"{
  "name": "nextjs-app",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
  },
  "dependencies": {
    "next": "14.0.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0"
  },
  "devDependencies": {
    "eslint": "^8",
    "eslint-config-next": "14.0.0"
  }
}
"###;

const NEXT_CONFIG: &str = r###"/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
}

module.exports = nextConfig
"###;

const APP_JS: &str = r###"import '../styles/globals.css'

function MyApp({ Component, pageProps }) {
  return <Component {...pageProps} />
}

export default MyApp
"###;

const INDEX_JS: &str = r###"import Layout from '../components/Layout'

export default function Home() {
  return (
    <Layout>
      <div className="hero">
        <h1>Welcome to Next.js</h1>
        <p>A modern React framework with SSR and routing</p>
        <div className="buttons">
          <a href="/about" className="btn btn-primary">About</a>
          <a href="/api/hello" className="btn btn-secondary">API Demo</a>
        </div>
      </div>
      
      <div className="features">
        <div className="feature-card">
          <h3>⚡ Fast</h3>
          <p>Server-side rendering and static generation</p>
        </div>
        <div className="feature-card">
          <h3>🔄 File-based Routing</h3>
          <p>Automatic routing based on file structure</p>
        </div>
        <div className="feature-card">
          <h3>🎨 CSS Support</h3>
          <p>Built-in CSS and Sass support</p>
        </div>
        <div className="feature-card">
          <h3>📦 API Routes</h3>
          <p>Build APIs with serverless functions</p>
        </div>
      </div>
    </Layout>
  )
}
"###;

const ABOUT_JS: &str = r###"import Layout from '../components/Layout'

export default function About() {
  return (
    <Layout>
      <div className="page-content">
        <h1>About This App</h1>
        <p>This is a Next.js template application.</p>
        <p>It includes:</p>
        <ul>
          <li>Server-side rendering (SSR)</li>
          <li>Static site generation (SSG)</li>
          <li>API routes</li>
          <li>File-based routing</li>
          <li>Optimized performance</li>
        </ul>
        <a href="/" className="btn btn-primary">Back to Home</a>
      </div>
    </Layout>
  )
}
"###;

const API_HELLO: &str = r###"export default function handler(req, res) {
  res.status(200).json({ 
    message: 'Hello from Next.js API!',
    timestamp: new Date().toISOString()
  })
}
"###;

const GLOBALS_CSS: &str = r###"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --primary: #0070f3;
  --secondary: #7928ca;
  --dark: #1a1a1a;
  --light: #f5f5f5;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Oxygen,
    Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif;
  color: var(--dark);
  background: var(--light);
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

.navbar {
  background: white;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
  padding: 1rem 0;
}

.nav-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--primary);
}

.nav-links {
  display: flex;
  gap: 2rem;
  list-style: none;
}

.nav-links a {
  color: var(--dark);
  text-decoration: none;
  transition: color 0.3s;
}

.nav-links a:hover {
  color: var(--primary);
}

.hero {
  text-align: center;
  padding: 100px 20px;
  background: linear-gradient(135deg, var(--primary), var(--secondary));
  color: white;
}

.hero h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.hero p {
  font-size: 1.5rem;
  margin-bottom: 2rem;
  opacity: 0.9;
}

.buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

.btn {
  padding: 12px 30px;
  border-radius: 5px;
  text-decoration: none;
  font-weight: 500;
  transition: all 0.3s;
  display: inline-block;
}

.btn-primary {
  background: white;
  color: var(--primary);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0,0,0,0.2);
}

.btn-secondary {
  background: transparent;
  color: white;
  border: 2px solid white;
}

.btn-secondary:hover {
  background: white;
  color: var(--primary);
}

.features {
  padding: 80px 20px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.feature-card {
  background: white;
  padding: 2rem;
  border-radius: 10px;
  box-shadow: 0 5px 15px rgba(0,0,0,0.1);
  transition: transform 0.3s;
}

.feature-card:hover {
  transform: translateY(-5px);
}

.feature-card h3 {
  color: var(--primary);
  margin-bottom: 1rem;
}

.page-content {
  max-width: 800px;
  margin: 0 auto;
  padding: 80px 20px;
}

.page-content h1 {
  font-size: 2.5rem;
  margin-bottom: 2rem;
}

.page-content ul {
  margin: 2rem 0;
  padding-left: 2rem;
}

.page-content li {
  margin: 0.5rem 0;
}

@media (max-width: 768px) {
  .hero h1 {
    font-size: 2rem;
  }
  
  .nav-links {
    gap: 1rem;
  }
}
"###;

const LAYOUT: &str = r###"export default function Layout({ children }) {
  return (
    <>
      <nav className="navbar">
        <div className="container">
          <div className="nav-content">
            <div className="logo">Next.js App</div>
            <ul className="nav-links">
              <li><a href="/">Home</a></li>
              <li><a href="/about">About</a></li>
            </ul>
          </div>
        </div>
      </nav>
      <main>{children}</main>
    </>
  )
}
"###;

const README: &str = r###"# Next.js Application

A modern web application built with Next.js.

## Features

- ⚡ Server-side rendering (SSR)
- 📄 Static site generation (SSG)
- 🔄 File-based routing
- 📦 API routes
- 🎨 CSS support
- ⚛️ React 18

## Getting Started

Install dependencies:
```bash
npm install
```

Run the development server:
```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser.

## Build for Production

```bash
npm run build
npm start
```

## Learn More

- [Next.js Documentation](https://nextjs.org/docs)
- [Learn Next.js](https://nextjs.org/learn)

## Deploy

Deploy easily to Vercel, Netlify, or any Node.js hosting platform.

## License

MIT
"###;

const GITIGNORE: &str = r###"# Dependencies
/node_modules
/.pnp
.pnp.js

# Testing
/coverage

# Next.js
/.next/
/out/

# Production
/build

# Misc
.DS_Store
*.pem

# Debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Local env files
.env*.local

# Vercel
.vercel
"###;


