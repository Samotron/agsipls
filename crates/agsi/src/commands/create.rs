use agsi_core::{Document, Material, material::MaterialType, GroundModel, model::{ModelType, ModelDimension}};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn document(id: String, output: PathBuf) -> Result<()> {
    println!("📄 Creating new AGSi document: {}", id);

    let doc = Document::new(&id)
        .with_file_name(output.file_name().unwrap().to_string_lossy().to_string())
        .with_author("agsi-cli");

    doc.to_json_file(&output)
        .with_context(|| format!("Failed to write to {}", output.display()))?;

    println!("✅ Document created: {}", output.display());
    Ok(())
}

pub async fn material(id: String, name: String, output: PathBuf) -> Result<()> {
    println!("🪨 Creating new material: {} ({})", name, id);

    // Create a simple document with just the material
    let mut doc = Document::new(uuid::Uuid::new_v4().to_string());
    
    let material = Material::new(&id, &name, MaterialType::Soil);
    
    let mut model = GroundModel::new(
        "temp_model",
        "Temporary model for material",
        ModelType::Geotechnical,
        ModelDimension::OneD,
    );
    model.add_material(material);
    doc.add_model(model);

    doc.to_json_file(&output)
        .with_context(|| format!("Failed to write to {}", output.display()))?;

    println!("✅ Material created in: {}", output.display());
    Ok(())
}

pub async fn model(id: String, name: String, output: PathBuf) -> Result<()> {
    println!("🗺️  Creating new model: {} ({})", name, id);

    let mut doc = Document::new(uuid::Uuid::new_v4().to_string());
    
    let model = GroundModel::new(
        &id,
        &name,
        ModelType::Stratigraphic,
        ModelDimension::TwoD,
    );
    
    doc.add_model(model);

    doc.to_json_file(&output)
        .with_context(|| format!("Failed to write to {}", output.display()))?;

    println!("✅ Model created in: {}", output.display());
    Ok(())
}
