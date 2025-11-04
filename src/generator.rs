use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

use crate::templates::*;

pub fn generate_project(name: &str, template: &str, output_dir: &Path) -> Result<()> {
    let project_path = output_dir.join(name);
    
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists!", name);
    }

    fs::create_dir_all(&project_path)
        .context("Failed to create project directory")?;

    match template {
        "flask-portfolio" | "flask" => {
            flask::generate(&project_path)?;
        }
        "mern-stack" | "mern" => {
            mern::generate(&project_path)?;
        }
        "next.js" | "nextjs" => {
            nextjs::generate(&project_path)?;
        }
        "react-portfolio" | "react" => {
            react::generate(&project_path)?;
        }
        "vue-portfolio" | "vue" => {
            vue::generate(&project_path)?;
        }
        "static-html" | "static" => {
            static_html::generate(&project_path)?;
        }
        _ => {
            anyhow::bail!("Unknown template: {}", template);
        }
    }

    Ok(())
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
