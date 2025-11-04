import React from 'react';

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
