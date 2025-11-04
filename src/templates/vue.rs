use std::path::Path;
use anyhow::Result;
use crate::generator::write_file;

pub fn generate(project_path: &Path) -> Result<()> {
    write_file(&project_path.join("package.json"), PACKAGE_JSON)?;
    write_file(&project_path.join("vite.config.js"), VITE_CONFIG)?;
    write_file(&project_path.join("index.html"), INDEX_HTML)?;
    write_file(&project_path.join("src/main.js"), MAIN_JS)?;
    write_file(&project_path.join("src/App.vue"), APP_VUE)?;
    write_file(&project_path.join("src/style.css"), STYLE_CSS)?;
    write_file(&project_path.join("src/components/Hero.vue"), HERO_VUE)?;
    write_file(&project_path.join("src/components/About.vue"), ABOUT_VUE)?;
    write_file(&project_path.join("src/components/Projects.vue"), PROJECTS_VUE)?;
    write_file(&project_path.join("README.md"), README)?;
    write_file(&project_path.join(".gitignore"), GITIGNORE)?;
    
    Ok(())
}

const PACKAGE_JSON: &str = r#"{
  "name": "vue-portfolio",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "vue": "^3.3.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^4.5.0",
    "vite": "^5.0.0"
  }
}
"#;

const VITE_CONFIG: &str = r#"import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
})
"#;

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <link rel="icon" type="image/svg+xml" href="/vite.svg">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Vue Portfolio</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.js"></script>
  </body>
</html>
"#;

const MAIN_JS: &str = r#"import { createApp } from 'vue'
import './style.css'
import App from './App.vue'

createApp(App).mount('#app')
"#;

const APP_VUE: &str = r#"<template>
  <div id="app">
    <nav class="navbar">
      <div class="container">
        <div class="logo">My Portfolio</div>
        <ul class="nav-menu">
          <li><a href="#hero">Home</a></li>
          <li><a href="#about">About</a></li>
          <li><a href="#projects">Projects</a></li>
        </ul>
      </div>
    </nav>

    <Hero />
    <About />
    <Projects />

    <footer class="footer">
      <div class="container">
        <p>&copy; 2024 My Portfolio. Built with Vue.js</p>
      </div>
    </footer>
  </div>
</template>

<script>
import Hero from './components/Hero.vue'
import About from './components/About.vue'
import Projects from './components/Projects.vue'

export default {
  name: 'App',
  components: {
    Hero,
    About,
    Projects
  }
}
</script>
"#;

const STYLE_CSS: &str = r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --primary: #42b883;
  --secondary: #35495e;
  --dark: #1e293b;
  --light: #f8fafc;
  --gray: #64748b;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: var(--dark);
  background: var(--light);
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
}

/* Navbar */
.navbar {
  background: white;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.navbar .container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 20px;
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--primary);
}

.nav-menu {
  display: flex;
  list-style: none;
  gap: 2rem;
}

.nav-menu a {
  color: var(--dark);
  text-decoration: none;
  transition: color 0.3s;
}

.nav-menu a:hover {
  color: var(--primary);
}

/* Hero */
.hero {
  padding: 100px 20px;
  text-align: center;
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

.hero-buttons {
  display: flex;
  gap: 1rem;
  justify-content: center;
}

/* Buttons */
.btn {
  padding: 12px 30px;
  border-radius: 5px;
  text-decoration: none;
  font-weight: 500;
  transition: all 0.3s;
  display: inline-block;
  border: none;
  cursor: pointer;
  font-size: 1rem;
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

/* About */
.about {
  padding: 80px 20px;
}

.about h2 {
  text-align: center;
  font-size: 2.5rem;
  margin-bottom: 3rem;
}

.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
}

.skill-card {
  background: white;
  padding: 2rem;
  border-radius: 10px;
  box-shadow: 0 5px 15px rgba(0,0,0,0.1);
  transition: transform 0.3s;
}

.skill-card:hover {
  transform: translateY(-5px);
}

.skill-card h3 {
  color: var(--primary);
  margin-bottom: 1rem;
}

/* Projects */
.projects {
  padding: 80px 20px;
  background: white;
}

.projects h2 {
  text-align: center;
  font-size: 2.5rem;
  margin-bottom: 3rem;
}

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 2rem;
}

.project-card {
  background: var(--light);
  padding: 2rem;
  border-radius: 10px;
  box-shadow: 0 5px 15px rgba(0,0,0,0.1);
}

.project-card h3 {
  color: var(--primary);
  margin-bottom: 1rem;
}

.tech-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin: 1rem 0;
}

.tag {
  background: white;
  padding: 5px 15px;
  border-radius: 20px;
  font-size: 0.85rem;
}

/* Footer */
.footer {
  background: var(--secondary);
  color: white;
  text-align: center;
  padding: 2rem 0;
}

@media (max-width: 768px) {
  .hero h1 {
    font-size: 2rem;
  }
  
  .nav-menu {
    gap: 1rem;
  }
}
"#;

const HERO_VUE: &str = r#"<template>
  <section class="hero" id="hero">
    <div class="container">
      <h1>Hi, I'm Your Name</h1>
      <p>Full Stack Developer | Designer | Creator</p>
      <div class="hero-buttons">
        <a href="#projects" class="btn btn-primary">View Projects</a>
        <a href="#about" class="btn btn-secondary">Learn More</a>
      </div>
    </div>
  </section>
</template>

<script>
export default {
  name: 'Hero'
}
</script>
"#;

const ABOUT_VUE: &str = r#"<template>
  <section class="about" id="about">
    <div class="container">
      <h2>About Me</h2>
      <div class="skills-grid">
        <div v-for="skill in skills" :key="skill.title" class="skill-card">
          <h3>{{ skill.title }}</h3>
          <p>{{ skill.description }}</p>
        </div>
      </div>
    </div>
  </section>
</template>

<script>
export default {
  name: 'About',
  data() {
    return {
      skills: [
        { title: 'Frontend', description: 'HTML, CSS, JavaScript, Vue, React' },
        { title: 'Backend', description: 'Node.js, Python, Flask, Express' },
        { title: 'Database', description: 'MongoDB, PostgreSQL, MySQL' },
        { title: 'Tools', description: 'Git, Docker, AWS, CI/CD' }
      ]
    }
  }
}
</script>
"#;

const PROJECTS_VUE: &str = r#"<template>
  <section class="projects" id="projects">
    <div class="container">
      <h2>My Projects</h2>
      <div class="projects-grid">
        <div v-for="project in projects" :key="project.title" class="project-card">
          <h3>{{ project.title }}</h3>
          <p>{{ project.description }}</p>
          <div class="tech-tags">
            <span v-for="tech in project.tech" :key="tech" class="tag">{{ tech }}</span>
          </div>
          <a :href="project.github" class="btn btn-primary" target="_blank">View on GitHub</a>
        </div>
      </div>
    </div>
  </section>
</template>

<script>
export default {
  name: 'Projects',
  data() {
    return {
      projects: [
        {
          title: 'Project 1',
          description: 'A cool project built with Vue',
          tech: ['Vue', 'Node.js', 'MongoDB'],
          github: 'https://github.com/yourusername/project1'
        },
        {
          title: 'Project 2',
          description: 'An awesome web application',
          tech: ['Python', 'Flask', 'PostgreSQL'],
          github: 'https://github.com/yourusername/project2'
        },
        {
          title: 'Project 3',
          description: 'A modern mobile app',
          tech: ['Vue', 'Firebase'],
          github: 'https://github.com/yourusername/project3'
        }
      ]
    }
  }
}
</script>
"#;

const README: &str = r#"# Vue Portfolio

A modern, responsive portfolio website built with Vue.js 3 and Vite.

## Features

- ⚡ Vue 3 with Composition API
- 🛠️ Vite for fast development
- 🏠 Hero section
- 👤 About section with skills
- 🚀 Projects showcase
- 📱 Fully responsive design

## Getting Started

### Install dependencies
```bash
npm install
```

### Run development server
```bash
npm run dev
```

Open [http://localhost:5173](http://localhost:5173) to view it in your browser.

### Build for production
```bash
npm run build
```

### Preview production build
```bash
npm run preview
```

## Customization

- Edit components in `src/components/` to modify content
- Modify `src/style.css` to change styling
- Update data in component's `data()` functions

## Deploy

This app can be deployed to:
- Vercel
- Netlify
- GitHub Pages
- Any static hosting

## License

MIT
"#;

const GITIGNORE: &str = r#"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?
"#;
