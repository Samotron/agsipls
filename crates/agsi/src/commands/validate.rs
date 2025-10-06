use agsi_core::{validation, Document};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(file: PathBuf, detailed: bool) -> Result<()> {
    println!("🔍 Validating AGSi file: {}", file.display());

    // Load document
    let doc = Document::from_json_file(&file)
        .with_context(|| format!("Failed to load file: {}", file.display()))?;

    // Validate
    let result = validation::validate_document(&doc)
        .with_context(|| "Validation failed")?;

    // Display results
    if result.is_valid() {
        println!("✅ Document is valid!");
    } else {
        println!("❌ Document has validation errors");
    }

    if detailed || !result.errors().is_empty() || !result.warnings().is_empty() {
        println!("\n{}", result.to_string());
    }

    // Summary
    let model_count = doc.agsi_model.len();
    let material_count: usize = doc.agsi_model.iter().map(|m| m.materials.len()).sum();
    let component_count: usize = doc.agsi_model.iter().map(|m| m.components.len()).sum();

    println!("\n📊 Summary:");
    println!("   Models: {}", model_count);
    println!("   Materials: {}", material_count);
    println!("   Components: {}", component_count);

    if !result.is_valid() {
        std::process::exit(1);
    }

    Ok(())
}
