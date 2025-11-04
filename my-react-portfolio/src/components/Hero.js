import React from 'react';

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
