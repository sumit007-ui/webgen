use std::path::Path;
use anyhow::Result;
use crate::generator::write_file;

pub fn generate(project_path: &Path) -> Result<()> {
    // app.py
    write_file(&project_path.join("app.py"), APP_PY)?;
    
    // requirements.txt
    write_file(&project_path.join("requirements.txt"), REQUIREMENTS)?;
    
    // Templates
    write_file(&project_path.join("templates/index.html"), INDEX_HTML)?;
    write_file(&project_path.join("templates/about.html"), ABOUT_HTML)?;
    write_file(&project_path.join("templates/projects.html"), PROJECTS_HTML)?;
    write_file(&project_path.join("templates/contact.html"), CONTACT_HTML)?;
    write_file(&project_path.join("templates/base.html"), BASE_HTML)?;
    
    // Static files
    write_file(&project_path.join("static/css/style.css"), STYLE_CSS)?;
    write_file(&project_path.join("static/js/main.js"), MAIN_JS)?;
    
    // README
    write_file(&project_path.join("README.md"), README)?;
    
    // .gitignore
    write_file(&project_path.join(".gitignore"), GITIGNORE)?;
    
    Ok(())
}

const APP_PY: &str = r#"from flask import Flask, render_template, request, jsonify
import os

app = Flask(__name__)

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/about')
def about():
    return render_template('about.html')

@app.route('/projects')
def projects():
    projects_data = [
        {
            'title': 'Project 1',
            'description': 'A cool project built with Python',
            'tech': ['Python', 'Flask', 'JavaScript'],
            'github': 'https://github.com/yourusername/project1'
        },
        {
            'title': 'Project 2',
            'description': 'An awesome web application',
            'tech': ['React', 'Node.js', 'MongoDB'],
            'github': 'https://github.com/yourusername/project2'
        },
    ]
    return render_template('projects.html', projects=projects_data)

@app.route('/contact', methods=['GET', 'POST'])
def contact():
    if request.method == 'POST':
        data = request.form
        # Handle form submission here
        return jsonify({'status': 'success', 'message': 'Message received!'})
    return render_template('contact.html')

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=5000)
"#;

const REQUIREMENTS: &str = r#"Flask==3.0.0
gunicorn==21.2.0
"#;

const BASE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{% block title %}Portfolio{% endblock %}</title>
    <link rel="stylesheet" href="{{ url_for('static', filename='css/style.css') }}">
</head>
<body>
    <nav class="navbar">
        <div class="container">
            <div class="nav-brand">
                <a href="/">My Portfolio</a>
            </div>
            <ul class="nav-menu">
                <li><a href="/">Home</a></li>
                <li><a href="/about">About</a></li>
                <li><a href="/projects">Projects</a></li>
                <li><a href="/contact">Contact</a></li>
            </ul>
        </div>
    </nav>

    <main>
        {% block content %}{% endblock %}
    </main>

    <footer>
        <div class="container">
            <p>&copy; 2024 My Portfolio. Built with Flask.</p>
        </div>
    </footer>

    <script src="{{ url_for('static', filename='js/main.js') }}"></script>
</body>
</html>
"#;

const INDEX_HTML: &str = r#"{% extends "base.html" %}

{% block title %}Home - Portfolio{% endblock %}

{% block content %}
<section class="hero">
    <div class="container">
        <h1 class="hero-title">Hi, I'm Your Name</h1>
        <p class="hero-subtitle">Full Stack Developer | Designer | Creator</p>
        <div class="hero-buttons">
            <a href="/projects" class="btn btn-primary">View Projects</a>
            <a href="/contact" class="btn btn-secondary">Get in Touch</a>
        </div>
    </div>
</section>

<section class="skills">
    <div class="container">
        <h2>Skills</h2>
        <div class="skills-grid">
            <div class="skill-card">
                <h3>Frontend</h3>
                <p>HTML, CSS, JavaScript, React, Vue</p>
            </div>
            <div class="skill-card">
                <h3>Backend</h3>
                <p>Python, Flask, Node.js, Express</p>
            </div>
            <div class="skill-card">
                <h3>Database</h3>
                <p>MongoDB, PostgreSQL, MySQL</p>
            </div>
            <div class="skill-card">
                <h3>Tools</h3>
                <p>Git, Docker, AWS, CI/CD</p>
            </div>
        </div>
    </div>
</section>
{% endblock %}
"#;

const ABOUT_HTML: &str = r#"{% extends "base.html" %}

{% block title %}About - Portfolio{% endblock %}

{% block content %}
<section class="about-section">
    <div class="container">
        <h1>About Me</h1>
        <div class="about-content">
            <div class="about-text">
                <p>Hi! I'm a passionate full-stack developer with experience in building web applications.</p>
                <p>I love creating efficient, scalable, and user-friendly solutions to complex problems.</p>
                <h3>My Journey</h3>
                <p>Started coding in 2020 and have been constantly learning and building ever since.</p>
            </div>
            <div class="about-image">
                <div class="placeholder-image">Your Photo Here</div>
            </div>
        </div>
    </div>
</section>
{% endblock %}
"#;

const PROJECTS_HTML: &str = r#"{% extends "base.html" %}

{% block title %}Projects - Portfolio{% endblock %}

{% block content %}
<section class="projects-section">
    <div class="container">
        <h1>My Projects</h1>
        <div class="projects-grid">
            {% for project in projects %}
            <div class="project-card">
                <h3>{{ project.title }}</h3>
                <p>{{ project.description }}</p>
                <div class="tech-tags">
                    {% for tech in project.tech %}
                    <span class="tag">{{ tech }}</span>
                    {% endfor %}
                </div>
                <a href="{{ project.github }}" class="btn btn-small" target="_blank">View on GitHub</a>
            </div>
            {% endfor %}
        </div>
    </div>
</section>
{% endblock %}
"#;

const CONTACT_HTML: &str = r#"{% extends "base.html" %}

{% block title %}Contact - Portfolio{% endblock %}

{% block content %}
<section class="contact-section">
    <div class="container">
        <h1>Get In Touch</h1>
        <form id="contactForm" class="contact-form">
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
            <a href="https://github.com/yourusername">GitHub</a>
            <a href="https://linkedin.com/in/yourusername">LinkedIn</a>
            <a href="https://twitter.com/yourusername">Twitter</a>
        </div>
    </div>
</section>
{% endblock %}
"#;

const STYLE_CSS: &str = r#"* {
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
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
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
}

.navbar .container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 20px;
}

.nav-brand a {
    font-size: 1.5rem;
    font-weight: bold;
    color: var(--primary);
    text-decoration: none;
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

/* Hero Section */
.hero {
    padding: 100px 0;
    text-align: center;
    background: linear-gradient(135deg, var(--primary), var(--secondary));
    color: white;
}

.hero-title {
    font-size: 3rem;
    margin-bottom: 1rem;
}

.hero-subtitle {
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

.btn-small {
    padding: 8px 20px;
    font-size: 0.9rem;
}

/* Skills Section */
.skills {
    padding: 80px 0;
}

.skills h2 {
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

/* About Section */
.about-section {
    padding: 80px 0;
}

.about-content {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 3rem;
    margin-top: 2rem;
}

.about-text p {
    margin-bottom: 1rem;
}

.placeholder-image {
    background: var(--gray);
    color: white;
    padding: 100px 20px;
    border-radius: 10px;
    text-align: center;
}

/* Projects Section */
.projects-section {
    padding: 80px 0;
}

.projects-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
}

.project-card {
    background: white;
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
    background: var(--light);
    padding: 5px 15px;
    border-radius: 20px;
    font-size: 0.85rem;
    color: var(--dark);
}

/* Contact Section */
.contact-section {
    padding: 80px 0;
}

.contact-form {
    max-width: 600px;
    margin: 2rem auto;
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
}

.social-links {
    display: flex;
    justify-content: center;
    gap: 2rem;
    margin-top: 3rem;
}

.social-links a {
    color: var(--primary);
    text-decoration: none;
    font-weight: 500;
}

/* Footer */
footer {
    background: var(--dark);
    color: white;
    text-align: center;
    padding: 2rem 0;
    margin-top: 4rem;
}

@media (max-width: 768px) {
    .hero-title {
        font-size: 2rem;
    }
    
    .about-content {
        grid-template-columns: 1fr;
    }
    
    .nav-menu {
        gap: 1rem;
    }
}
"#;

const MAIN_JS: &str = r#"// Main JavaScript file

document.addEventListener('DOMContentLoaded', function() {
    // Handle contact form submission
    const contactForm = document.getElementById('contactForm');
    if (contactForm) {
        contactForm.addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const formData = new FormData(contactForm);
            
            try {
                const response = await fetch('/contact', {
                    method: 'POST',
                    body: formData
                });
                
                const result = await response.json();
                
                if (result.status === 'success') {
                    alert('Message sent successfully!');
                    contactForm.reset();
                } else {
                    alert('Failed to send message. Please try again.');
                }
            } catch (error) {
                console.error('Error:', error);
                alert('An error occurred. Please try again.');
            }
        });
    }
    
    // Smooth scrolling
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth'
                });
            }
        });
    });
});
"#;

const README: &str = r#"# Flask Portfolio Website

A modern, responsive portfolio website built with Flask.

## Features

- 🏠 Home page with hero section
- 👤 About page
- 🚀 Projects showcase
- 📧 Contact form
- 📱 Fully responsive design

## Setup

1. Create a virtual environment:
```bash
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

2. Install dependencies:
```bash
pip install -r requirements.txt
```

3. Run the application:
```bash
python app.py
```

4. Open your browser and visit: `http://localhost:5000`

## Customization

- Edit `templates/*.html` to modify the content
- Modify `static/css/style.css` to change the styling
- Update `app.py` to add new routes or functionality

## Deployment

This app can be deployed to:
- Heroku
- PythonAnywhere
- AWS
- DigitalOcean

## License

MIT
"#;

const GITIGNORE: &str = r#"# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
venv/
env/
ENV/
.venv

# Flask
instance/
.webassets-cache

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
"#;
