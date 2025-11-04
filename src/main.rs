use clap::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;

mod templates;
mod generator;

#[derive(Parser)]
#[command(name = "webgen")]
#[command(author = "sumit007-ui")]
#[command(version = "1.0.0")]
#[command(about = "Generate pre-built website templates for various tech stacks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from a template
    New {
        /// Project name
        name: Option<String>,
        
        /// Template type (flask, mern, nextjs, react, vue, portfolio)
        #[arg(short, long)]
        template: Option<String>,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// List all available templates
    List,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::New { name, template, output }) => {
            handle_new_project(name.clone(), template.clone(), output.clone())?;
        }
        Some(Commands::List) => {
            list_templates();
        }
        None => {
            // Interactive mode
            interactive_mode()?;
        }
    }

    Ok(())
}

fn handle_new_project(
    name: Option<String>,
    template: Option<String>,
    output: Option<PathBuf>,
) -> anyhow::Result<()> {
    let project_name = name.unwrap_or_else(|| {
        dialoguer::Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project name")
            .default("my-website".to_string())
            .interact()
            .unwrap()
    });

    let template_type = if let Some(t) = template {
        t
    } else {
        let templates = templates::get_available_templates();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose a template")
            .items(&templates)
            .default(0)
            .interact()
            .unwrap();
        templates[selection].to_lowercase().replace(" ", "-")
    };

    let output_dir = output.unwrap_or_else(|| PathBuf::from("."));

    println!("\n{}", "🚀 Generating your project...".bright_cyan().bold());
    println!("   📁 Name: {}", project_name.bright_white());
    println!("   🎨 Template: {}", template_type.bright_white());
    println!();

    generator::generate_project(&project_name, &template_type, &output_dir)?;

    println!("\n{}", "✨ Project created successfully!".bright_green().bold());
    println!("\n{}", "Next steps:".bright_yellow().bold());
    println!("   cd {}", project_name);
    
    match template_type.as_str() {
        "flask" | "flask-portfolio" => {
            println!("   python -m venv venv");
            println!("   source venv/bin/activate  # On Windows: venv\\Scripts\\activate");
            println!("   pip install -r requirements.txt");
            println!("   python app.py");
        }
        "mern" | "mern-stack" => {
            println!("   # Backend:");
            println!("   cd backend && npm install && npm start");
            println!("   # Frontend:");
            println!("   cd frontend && npm install && npm start");
        }
        "nextjs" | "next.js" => {
            println!("   npm install");
            println!("   npm run dev");
        }
        "react" | "react-portfolio" => {
            println!("   npm install");
            println!("   npm start");
        }
        "vue" | "vue-portfolio" => {
            println!("   npm install");
            println!("   npm run dev");
        }
        _ => {
            println!("   Check README.md for setup instructions");
        }
    }

    Ok(())
}

fn list_templates() {
    println!("\n{}", "📚 Available Templates:".bright_cyan().bold());
    println!();
    
    let templates = vec![
        ("Flask Portfolio", "Python Flask-based portfolio website with modern UI"),
        ("MERN Stack", "MongoDB, Express, React, Node.js full-stack application"),
        ("Next.js App", "Modern React framework with SSR and routing"),
        ("React Portfolio", "Single-page portfolio with React and Tailwind CSS"),
        ("Vue Portfolio", "Vue.js 3 portfolio with Composition API"),
        ("Static HTML", "Pure HTML/CSS/JS portfolio template"),
    ];

    for (name, description) in templates {
        println!("  {} {}", "•".bright_green(), name.bright_white().bold());
        println!("    {}", description.dimmed());
        println!();
    }
}

fn interactive_mode() -> anyhow::Result<()> {
    println!("\n{}", "🎨 Website Template Generator".bright_cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    
    handle_new_project(None, None, None)?;
    
    Ok(())
}
