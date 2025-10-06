use agsi_core::Document;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(file: PathBuf, show_materials: bool, show_models: bool) -> Result<()> {
    let doc = Document::from_json_file(&file)
        .with_context(|| format!("Failed to load file: {}", file.display()))?;

    println!("📄 AGSi Document Information");
    println!("═══════════════════════════════════════");
    
    // File info
    println!("\n🗂️  File Information:");
    println!("   ID: {}", doc.ags_file.file_id);
    if let Some(ref name) = doc.ags_file.file_name {
        println!("   Name: {}", name);
    }
    if let Some(ref author) = doc.ags_file.file_author {
        println!("   Author: {}", author);
    }
    if let Some(ref software) = doc.ags_file.file_software {
        println!("   Software: {}", software);
    }
    println!("   Schema Version: {}", doc.ags_schema.version);

    // Project info
    if let Some(ref project) = doc.ags_project {
        println!("\n🏗️  Project:");
        println!("   ID: {}", project.id);
        println!("   Name: {}", project.name);
        if let Some(ref desc) = project.description {
            println!("   Description: {}", desc);
        }
        if let Some(ref client) = project.client {
            println!("   Client: {}", client);
        }
        if let Some(ref location) = project.location {
            println!("   Location: {}", location.name);
        }
    }

    // Models summary
    println!("\n🗺️  Models: {}", doc.agsi_model.len());
    
    let total_materials: usize = doc.agsi_model.iter().map(|m| m.materials.len()).sum();
    let total_components: usize = doc.agsi_model.iter().map(|m| m.components.len()).sum();
    
    println!("   Total Materials: {}", total_materials);
    println!("   Total Components: {}", total_components);

    // Detailed models
    if show_models || (!show_materials && !show_models) {
        for model in &doc.agsi_model {
            println!("\n   • Model: {} ({})", model.name, model.id);
            println!("     Type: {:?}", model.model_type);
            println!("     Dimension: {:?}", model.dimension);
            println!("     Materials: {}", model.materials.len());
            println!("     Components: {}", model.components.len());
            
            if let Some(ref crs) = model.crs {
                println!("     CRS: {}", crs);
            }
            
            if let Some(ref extent) = model.extent {
                println!("     Extent: [{:.2}, {:.2}] x [{:.2}, {:.2}]",
                    extent.min_x, extent.max_x, extent.min_y, extent.max_y);
            }
        }
    }

    // Detailed materials
    if show_materials {
        for (_model_idx, model) in doc.agsi_model.iter().enumerate() {
            if !model.materials.is_empty() {
                println!("\n   Materials in model {} ({}):", model.name, model.id);
                for material in &model.materials {
                    println!("     • {} ({}) - {:?}",
                        material.name,
                        material.id,
                        material.material_type
                    );
                    
                    if !material.properties.is_empty() {
                        println!("       Properties: {}", material.properties.len());
                        for prop in &material.properties {
                            let unit = prop.unit.as_ref().map(|u| format!(" {}", u)).unwrap_or_default();
                            println!("         - {}: {:?}{}", prop.name, prop.value, unit);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
