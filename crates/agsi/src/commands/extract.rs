use agsi_core::Document;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(
    file: PathBuf,
    model_id: Option<String>,
    output: Option<PathBuf>,
) -> Result<()> {
    println!("📤 Extracting materials from: {}", file.display());

    let doc = Document::from_json_file(&file)
        .with_context(|| format!("Failed to load file: {}", file.display()))?;

    // Determine which model to extract from
    let model = if let Some(id) = model_id {
        doc.get_model(&id)
            .with_context(|| format!("Model not found: {}", id))?
    } else if doc.agsi_model.len() == 1 {
        &doc.agsi_model[0]
    } else {
        anyhow::bail!(
            "Multiple models found. Please specify --model <id>. Available: {}",
            doc.agsi_model
                .iter()
                .map(|m| m.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    };

    println!("   From model: {} ({})", model.name, model.id);
    println!("   Materials: {}", model.materials.len());

    // Create output
    let output_data = serde_json::to_string_pretty(&model.materials)?;

    if let Some(output_path) = output {
        std::fs::write(&output_path, output_data)
            .with_context(|| format!("Failed to write to {}", output_path.display()))?;
        println!("✅ Materials extracted to: {}", output_path.display());
    } else {
        println!("\n{}", output_data);
    }

    // Summary
    for material in &model.materials {
        println!("   • {} ({}) - {} properties", 
            material.name, 
            material.id, 
            material.properties.len()
        );
    }

    Ok(())
}
