pub fn get_available_templates() -> Vec<String> {
    vec![
        "Flask Portfolio".to_string(),
        "MERN Stack".to_string(),
        "Next.js App".to_string(),
        "React Portfolio".to_string(),
        "Vue Portfolio".to_string(),
        "Static HTML".to_string(),
    ]
}

pub mod flask;
pub mod mern;
pub mod nextjs;
pub mod react;
pub mod vue;
pub mod static_html;
