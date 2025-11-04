import React from 'react';

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
