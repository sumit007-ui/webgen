use std::path::Path;
use anyhow::Result;
use crate::generator::write_file;

pub fn generate(project_path: &Path) -> Result<()> {
    write_file(&project_path.join("index.html"), INDEX_HTML)?;
    write_file(&project_path.join("css/style.css"), STYLE_CSS)?;
    write_file(&project_path.join("js/main.js"), MAIN_JS)?;
    write_file(&project_path.join("README.md"), README)?;
    write_file(&project_path.join(".gitignore"), GITIGNORE)?;
    
    Ok(())
}

const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>My Portfolio</title>
    <link rel="stylesheet" href="css/style.css">
</head>
<body>
    <nav class="navbar">
        <div class="container">
            <div class="logo">My Portfolio</div>
            <ul class="nav-menu" id="navMenu">
                <li><a href="#home">Home</a></li>
                <li><a href="#about">About</a></li>
                <li><a href="#projects">Projects</a></li>
                <li><a href="#contact">Contact</a></li>
            </ul>
            <div class="hamburger" id="hamburger">
                <span></span>
                <span></span>
                <span></span>
            </div>
        </div>
    </nav>

    <section class="hero" id="home">
        <div class="container">
            <h1 class="hero-title">Hi, I'm Your Name</h1>
            <p class="hero-subtitle">Full Stack Developer</p>
            <div class="hero-buttons">
                <a href="#projects" class="btn btn-primary">View Projects</a>
                <a href="#contact" class="btn btn-secondary">Get in Touch</a>
            </div>
        </div>
    </section>

    <section class="about" id="about">
        <div class="container">
            <h2 class="section-title">About Me</h2>
            <div class="skills-grid">
                <div class="skill-card">
                    <div class="skill-icon">💻</div>
                    <h3>Frontend</h3>
                    <p>HTML, CSS, JavaScript, React, Vue</p>
                </div>
                <div class="skill-card">
                    <div class="skill-icon">⚙️</div>
                    <h3>Backend</h3>
                    <p>Node.js, Python, Flask, Express</p>
                </div>
                <div class="skill-card">
                    <div class="skill-icon">🗄️</div>
                    <h3>Database</h3>
                    <p>MongoDB, PostgreSQL, MySQL</p>
                </div>
                <div class="skill-card">
                    <div class="skill-icon">🛠️</div>
                    <h3>Tools</h3>
                    <p>Git, Docker, AWS</p>
                </div>
            </div>
        </div>
    </section>

    <section class="projects" id="projects">
        <div class="container">
            <h2 class="section-title">My Projects</h2>
            <div class="projects-grid">
                <div class="project-card">
                    <h3>Project 1</h3>
                    <p>A cool project built with modern web technologies</p>
                    <div class="tech-tags">
                        <span class="tag">HTML</span>
                        <span class="tag">CSS</span>
                        <span class="tag">JavaScript</span>
                    </div>
                    <a href="https://github.com" class="btn btn-primary">View</a>
                </div>
                <div class="project-card">
                    <h3>Project 2</h3>
                    <p>An awesome web application</p>
                    <div class="tech-tags">
                        <span class="tag">React</span>
                        <span class="tag">Node.js</span>
                    </div>
                    <a href="https://github.com" class="btn btn-primary">View</a>
                </div>
                <div class="project-card">
                    <h3>Project 3</h3>
                    <p>A modern responsive website</p>
                    <div class="tech-tags">
                        <span class="tag">Vue</span>
                        <span class="tag">Firebase</span>
                    </div>
                    <a href="https://github.com" class="btn btn-primary">View</a>
                </div>
            </div>
        </div>
    </section>

    <section class="contact" id="contact">
        <div class="container">
            <h2 class="section-title">Get In Touch</h2>
            <form class="contact-form" id="contactForm">
                <div class="form-group">
                    <label for="name">Name</label>
                    <input type="text" id="name" name="name" required>
                </div>
                <div class="form-group">
                    <label for="email">Email</label>
                    <input type="email" id="email" name="email" required>
                </div>
                <div class="form-group">
                    <label for="message">Message</label>
                    <textarea id="message" name="message" rows="5" required></textarea>
                </div>
                <button type="submit" class="btn btn-primary">Send Message</button>
            </form>
            <div class="social-links">
                <a href="https://github.com">GitHub</a>
                <a href="https://linkedin.com">LinkedIn</a>
                <a href="https://twitter.com">Twitter</a>
            </div>
        </div>
    </section>

    <footer class="footer">
        <div class="container">
            <p>&copy; 2024 My Portfolio. All rights reserved.</p>
        </div>
    </footer>

    <script src="js/main.js"></script>
</body>
</html>
"##;

const STYLE_CSS: &str = r##"* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

:root {
    --primary: #6366f1;
    --secondary: #8b5cf6;
    --dark: #1e293b;
    --light: #f8fafc;
}

html {
    scroll-behavior: smooth;
}

body {
    font-family: system-ui, -apple-system, sans-serif;
    line-height: 1.6;
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
    padding: 1rem 0;
}

.navbar .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
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

.hamburger {
    display: none;
    flex-direction: column;
    cursor: pointer;
}

.hamburger span {
    width: 25px;
    height: 3px;
    background: var(--dark);
    margin: 3px 0;
    transition: 0.3s;
}

/* Hero Section */
.hero {
    padding: 100px 20px;
    text-align: center;
    background: linear-gradient(135deg, var(--primary), var(--secondary));
    color: white;
}

.hero-title {
    font-size: 3rem;
    margin-bottom: 1rem;
    animation: fadeInUp 0.8s ease-out;
}

.hero-subtitle {
    font-size: 1.5rem;
    margin-bottom: 2rem;
    opacity: 0.9;
    animation: fadeInUp 0.8s ease-out 0.2s both;
}

.hero-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    animation: fadeInUp 0.8s ease-out 0.4s both;
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

/* Sections */
.about, .projects, .contact {
    padding: 80px 20px;
}

.section-title {
    text-align: center;
    font-size: 2.5rem;
    margin-bottom: 3rem;
    color: var(--dark);
}

/* Skills Grid */
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
    text-align: center;
}

.skill-card:hover {
    transform: translateY(-5px);
}

.skill-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
}

.skill-card h3 {
    color: var(--primary);
    margin-bottom: 1rem;
}

/* Projects */
.projects {
    background: white;
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
    transition: transform 0.3s;
}

.project-card:hover {
    transform: translateY(-5px);
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
    color: var(--dark);
}

/* Contact Form */
.contact-form {
    max-width: 600px;
    margin: 0 auto 3rem;
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

.form-group input:focus,
.form-group textarea:focus {
    outline: none;
    border-color: var(--primary);
}

.social-links {
    display: flex;
    justify-content: center;
    gap: 2rem;
}

.social-links a {
    color: var(--primary);
    text-decoration: none;
    font-weight: 500;
    transition: color 0.3s;
}

.social-links a:hover {
    color: var(--secondary);
}

/* Footer */
.footer {
    background: var(--dark);
    color: white;
    text-align: center;
    padding: 2rem 0;
}

/* Animations */
@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Responsive */
@media (max-width: 768px) {
    .hero-title {
        font-size: 2rem;
    }
    
    .hero-subtitle {
        font-size: 1.2rem;
    }
    
    .nav-menu {
        position: fixed;
        left: -100%;
        top: 70px;
        flex-direction: column;
        background-color: white;
        width: 100%;
        text-align: center;
        transition: 0.3s;
        box-shadow: 0 10px 27px rgba(0,0,0,0.05);
        padding: 2rem 0;
    }
    
    .nav-menu.active {
        left: 0;
    }
    
    .hamburger {
        display: flex;
    }
    
    .hero-buttons {
        flex-direction: column;
        align-items: center;
    }
}
"##;

const MAIN_JS: &str = r##"const hamburger = document.getElementById('hamburger');
const navMenu = document.getElementById('navMenu');

hamburger.addEventListener('click', () => {
    navMenu.classList.toggle('active');
});

const contactForm = document.getElementById('contactForm');

contactForm.addEventListener('submit', (e) => {
    e.preventDefault();
    alert('Message sent! (Demo)');
    contactForm.reset();
});

document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({ behavior: 'smooth' });
        }
    });
});
"##;

const README: &str = r##"# Static HTML Portfolio

A modern, responsive portfolio website built with pure HTML, CSS, and JavaScript.

## Getting Started

Simply open `index.html` in your web browser!

For development:

```bash
python -m http.server 8000
```

Then open http://localhost:8000

## Structure

```
├── index.html
├── css/
│   └── style.css
└── js/
    └── main.js
```

## Customization

1. Edit `index.html` to update your information
2. Modify `css/style.css` for styling
3. Update `js/main.js` for behavior

## Deploy

Deploy to:
- GitHub Pages
- Netlify
- Vercel
- Any static hosting

No build required!

## License

MIT
"##;

const GITIGNORE: &str = ".DS_Store\nThumbs.db\n*.log\n";
