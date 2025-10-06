use agsi_core::Document;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(file: PathBuf) -> Result<()> {
    println!("📊 Statistics for: {}", file.display());
    println!();

    let doc = Document::from_json_file(&file)
        .with_context(|| format!("Failed to load file: {}", file.display()))?;

    // Overall stats
    println!("📄 Document:");
    println!("   ID: {}", doc.ags_file.file_id);
    println!("   Schema: {}", doc.ags_schema.version);
    if let Some(ref author) = doc.ags_file.file_author {
        println!("   Author: {}", author);
    }
    println!();

    // Model stats
    println!("🗺️  Models: {}", doc.agsi_model.len());
    for model in &doc.agsi_model {
        println!("   • {} ({})", model.name, model.id);
        println!("     Type: {:?}, Dimension: {:?}", model.model_type, model.dimension);
        println!("     Materials: {}", model.materials.len());
        println!("     Components: {}", model.components.len());
        
        if let Some(ref extent) = model.extent {
            println!("     Extent: [{:.2}, {:.2}] x [{:.2}, {:.2}]",
                extent.min_x, extent.max_x, extent.min_y, extent.max_y);
        }
    }
    println!();

    // Material statistics
    let total_materials: usize = doc.agsi_model.iter().map(|m| m.materials.len()).sum();
    println!("🪨 Materials: {}", total_materials);
    
    if total_materials > 0 {
        let mut material_types = std::collections::HashMap::new();
        let mut property_counts = Vec::new();
        
        for model in &doc.agsi_model {
            for material in &model.materials {
                *material_types.entry(format!("{:?}", material.material_type))
                    .or_insert(0) += 1;
                property_counts.push(material.properties.len());
            }
        }
        
        println!("   Types:");
        for (mat_type, count) in material_types {
            println!("     • {}: {}", mat_type, count);
        }
        
        if !property_counts.is_empty() {
            let avg_props: f64 = property_counts.iter().sum::<usize>() as f64 / property_counts.len() as f64;
            let min_props = property_counts.iter().min().unwrap();
            let max_props = property_counts.iter().max().unwrap();
            println!("   Properties per material:");
            println!("     • Average: {:.1}", avg_props);
            println!("     • Min: {}", min_props);
            println!("     • Max: {}", max_props);
        }
    }
    println!();

    // Component statistics
    let total_components: usize = doc.agsi_model.iter().map(|m| m.components.len()).sum();
    println!("🔧 Components: {}", total_components);
    
    if total_components > 0 {
        let mut component_types = std::collections::HashMap::new();
        
        for model in &doc.agsi_model {
            for component in &model.components {
                *component_types.entry(format!("{:?}", component.component_type))
                    .or_insert(0) += 1;
            }
        }
        
        println!("   Types:");
        for (comp_type, count) in component_types {
            println!("     • {}: {}", comp_type, count);
        }
    }

    Ok(())
}
