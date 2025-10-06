use agsi_core::Document;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(file: PathBuf) -> Result<()> {
    println!("✏️  Editing file: {}", file.display());
    
    let doc = Document::from_json_file(&file)
        .with_context(|| format!("Failed to load file: {}", file.display()))?;

    println!("\n📄 Document: {}", doc.ags_file.file_id);
    
    if let Some(ref project) = doc.ags_project {
        println!("   Project: {}", project.name);
    }
    
    println!("   Models: {}", doc.agsi_model.len());

    // For now, just show the info. Full editor to be implemented
    println!("\n⚠️  Full interactive editor not yet implemented.");
    println!("   Use 'agsi form' for interactive creation.");
    
    Ok(())
}
