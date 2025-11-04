use std::path::Path;
use anyhow::Result;
use crate::generator::write_file;

pub fn generate(project_path: &Path) -> Result<()> {
    // Package.json
    write_file(&project_path.join("package.json"), PACKAGE_JSON)?;
    
    // Public files
    write_file(&project_path.join("public/index.html"), PUBLIC_INDEX)?;
    
    // Source files
    write_file(&project_path.join("src/index.js"), INDEX_JS)?;
    write_file(&project_path.join("src/App.js"), APP_JS)?;
    write_file(&project_path.join("src/App.css"), APP_CSS)?;
    write_file(&project_path.join("src/components/Header.js"), HEADER_JS)?;
    write_file(&project_path.join("src/components/Hero.js"), HERO_JS)?;
    write_file(&project_path.join("src/components/About.js"), ABOUT_JS)?;
    write_file(&project_path.join("src/components/Projects.js"), PROJECTS_JS)?;
    write_file(&project_path.join("src/components/Contact.js"), CONTACT_JS)?;
    write_file(&project_path.join("src/components/Footer.js"), FOOTER_JS)?;
    
    // README & .gitignore
    write_file(&project_path.join("README.md"), README)?;
    write_file(&project_path.join(".gitignore"), GITIGNORE)?;
    
    Ok(())
}

const PACKAGE_JSON: &str = r#"{
  "name": "react-portfolio",
  "version": "0.1.0",
  "private": true,
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-scripts": "5.0.1"
  },
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  },
  "eslintConfig": {
    "extends": [
      "react-app"
    ]
  },
  "browserslist": {
    "production": [
      ">0.2%",
      "not dead",
      "not op_mini all"
    ],
    "development": [
      "last 1 chrome version",
      "last 1 firefox version",
      "last 1 safari version"
    ]
  }
}
"#;

const PUBLIC_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="theme-color" content="#000000" />
    <meta name="description" content="Portfolio website created with React" />
    <title>My Portfolio</title>
  </head>
  <body>
    <noscript>You need to enable JavaScript to run this app.</noscript>
    <div id="root"></div>
  </body>
</html>
"#;

const INDEX_JS: &str = r#"import React from 'react';
import ReactDOM from 'react-dom/client';
import './App.css';
import App from './App';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
"#;

const APP_JS: &str = r#"import React from 'react';
import './App.css';
import Header from './components/Header';
import Hero from './components/Hero';
import About from './components/About';
import Projects from './components/Projects';
import Contact from './components/Contact';
import Footer from './components/Footer';

function App() {
  return (
    <div className="App">
      <Header />
      <Hero />
      <About />
      <Projects />
      <Contact />
      <Footer />
    </div>
  );
}

export default App;
"#;

const APP_CSS: &str = r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --primary: #6366f1;
  --secondary: #8b5cf6;
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

/* Header */
.header {
  background: white;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
  position: sticky;
  top: 0;
  z-index: 100;
}

.header .container {
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

.nav {
  display: flex;
  list-style: none;
  gap: 2rem;
}

.nav a {
  color: var(--dark);
  text-decoration: none;
  transition: color 0.3s;
}

.nav a:hover {
  color: var(--primary);
}

/* Hero */
.hero {
  padding: 100px 0;
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
  padding: 80px 0;
}

.about h2 {
  text-align: center;
  font-size: 2.5rem;
  margin-bottom: 3rem;
}

.about-content {
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
  padding: 80px 0;
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

/* Contact */
.contact {
  padding: 80px 0;
}

.contact h2 {
  text-align: center;
  font-size: 2.5rem;
  margin-bottom: 3rem;
}

.contact-form {
  max-width: 600px;
  margin: 0 auto;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 5px;
  font-family: inherit;
  font-size: 1rem;
}

/* Footer */
.footer {
  background: var(--dark);
  color: white;
  text-align: center;
  padding: 2rem 0;
}

@media (max-width: 768px) {
  .hero h1 {
    font-size: 2rem;
  }
  
  .nav {
    gap: 1rem;
  }
}
"#;

const HEADER_JS: &str = r#"import React from 'react';

function Header() {
  return (
    <header className="header">
      <div className="container">
        <div className="logo">My Portfolio</div>
        <nav>
          <ul className="nav">
            <li><a href="#hero">Home</a></li>
            <li><a href="#about">About</a></li>
            <li><a href="#projects">Projects</a></li>
            <li><a href="#contact">Contact</a></li>
          </ul>
        </nav>
      </div>
    </header>
  );
}

export default Header;
"#;

const HERO_JS: &str = r#"import React from 'react';

function Hero() {
  return (
    <section className="hero" id="hero">
      <div className="container">
        <h1>Hi, I'm Your Name</h1>
        <p>Full Stack Developer | Designer | Creator</p>
        <div className="hero-buttons">
          <a href="#projects" className="btn btn-primary">View Projects</a>
          <a href="#contact" className="btn btn-secondary">Get in Touch</a>
        </div>
      </div>
    </section>
  );
}

export default Hero;
"#;

const ABOUT_JS: &str = r#"import React from 'react';

function About() {
  const skills = [
    { title: 'Frontend', description: 'HTML, CSS, JavaScript, React, Vue' },
    { title: 'Backend', description: 'Node.js, Python, Flask, Express' },
    { title: 'Database', description: 'MongoDB, PostgreSQL, MySQL' },
    { title: 'Tools', description: 'Git, Docker, AWS, CI/CD' }
  ];

  return (
    <section className="about" id="about">
      <div className="container">
        <h2>About Me</h2>
        <div className="about-content">
          {skills.map((skill, index) => (
            <div key={index} className="skill-card">
              <h3>{skill.title}</h3>
              <p>{skill.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

export default About;
"#;

const PROJECTS_JS: &str = r#"import React from 'react';

function Projects() {
  const projects = [
    {
      title: 'Project 1',
      description: 'A cool project built with React',
      tech: ['React', 'Node.js', 'MongoDB'],
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
      tech: ['React Native', 'Firebase'],
      github: 'https://github.com/yourusername/project3'
    }
  ];

  return (
    <section className="projects" id="projects">
      <div className="container">
        <h2>My Projects</h2>
        <div className="projects-grid">
          {projects.map((project, index) => (
            <div key={index} className="project-card">
              <h3>{project.title}</h3>
              <p>{project.description}</p>
              <div className="tech-tags">
                {project.tech.map((tech, i) => (
                  <span key={i} className="tag">{tech}</span>
                ))}
              </div>
              <a href={project.github} className="btn btn-primary" target="_blank" rel="noopener noreferrer">
                View on GitHub
              </a>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

export default Projects;
"#;

const CONTACT_JS: &str = r#"import React, { useState } from 'react';

function Contact() {
  const [formData, setFormData] = useState({
    name: '',
    email: '',
    message: ''
  });

  const handleSubmit = (e) => {
    e.preventDefault();
    alert('Message sent! (This is a demo)');
    setFormData({ name: '', email: '', message: '' });
  };

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    });
  };

  return (
    <section className="contact" id="contact">
      <div className="container">
        <h2>Get In Touch</h2>
        <form className="contact-form" onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="name">Name</label>
            <input
              type="text"
              id="name"
              name="name"
              value={formData.name}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label htmlFor="email">Email</label>
            <input
              type="email"
              id="email"
              name="email"
              value={formData.email}
              onChange={handleChange}
              required
            />
          </div>
          <div className="form-group">
            <label htmlFor="message">Message</label>
            <textarea
              id="message"
              name="message"
              rows="5"
              value={formData.message}
              onChange={handleChange}
              required
            ></textarea>
          </div>
          <button type="submit" className="btn btn-primary">Send Message</button>
        </form>
      </div>
    </section>
  );
}

export default Contact;
"#;

const FOOTER_JS: &str = r#"import React from 'react';

function Footer() {
  return (
    <footer className="footer">
      <div className="container">
        <p>&copy; 2024 My Portfolio. Built with React.</p>
      </div>
    </footer>
  );
}

export default Footer;
"#;

const README: &str = r#"# React Portfolio

A modern, responsive portfolio website built with React.

## Features

- 🏠 Hero section with call-to-action
- 👤 About section with skills
- 🚀 Projects showcase
- 📧 Contact form
- 📱 Fully responsive design

## Getting Started

### Install dependencies
```bash
npm install
```

### Run development server
```bash
npm start
```

Open [http://localhost:3000](http://localhost:3000) to view it in your browser.

### Build for production
```bash
npm run build
```

## Customization

- Edit components in `src/components/` to modify content
- Modify `src/App.css` to change styling
- Update project data in `Projects.js`

## Deploy

This app can be deployed to:
- Vercel
- Netlify
- GitHub Pages
- AWS Amplify

## License

MIT
"#;

const GITIGNORE: &str = r#"# Dependencies
/node_modules
/.pnp
.pnp.js

# Testing
/coverage

# Production
/build

# Misc
.DS_Store
.env.local
.env.development.local
.env.test.local
.env.production.local

npm-debug.log*
yarn-debug.log*
yarn-error.log*
"#;
