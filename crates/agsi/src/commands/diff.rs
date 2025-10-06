use agsi_core::Document;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(file1: PathBuf, file2: PathBuf, detailed: bool) -> Result<()> {
    println!("🔍 Comparing AGSi files:");
    println!("   File 1: {}", file1.display());
    println!("   File 2: {}", file2.display());
    println!();

    // Load both documents
    let doc1 = Document::from_json_file(&file1)
        .with_context(|| format!("Failed to load file: {}", file1.display()))?;
    let doc2 = Document::from_json_file(&file2)
        .with_context(|| format!("Failed to load file: {}", file2.display()))?;

    let mut differences = Vec::new();
    let mut identical = true;

    // Compare schema versions
    if doc1.ags_schema.version != doc2.ags_schema.version {
        differences.push(format!(
            "Schema version: {} → {}",
            doc1.ags_schema.version, doc2.ags_schema.version
        ));
        identical = false;
    }

    // Compare file IDs
    if doc1.ags_file.file_id != doc2.ags_file.file_id {
        differences.push(format!(
            "File ID: {} → {}",
            doc1.ags_file.file_id, doc2.ags_file.file_id
        ));
        identical = false;
    }

    // Compare model counts
    let models1 = doc1.agsi_model.len();
    let models2 = doc2.agsi_model.len();
    if models1 != models2 {
        differences.push(format!("Model count: {} → {}", models1, models2));
        identical = false;
    }

    // Compare materials
    let materials1: usize = doc1.agsi_model.iter().map(|m| m.materials.len()).sum();
    let materials2: usize = doc2.agsi_model.iter().map(|m| m.materials.len()).sum();
    if materials1 != materials2 {
        differences.push(format!("Material count: {} → {}", materials1, materials2));
        identical = false;
    }

    // Compare components
    let components1: usize = doc1.agsi_model.iter().map(|m| m.components.len()).sum();
    let components2: usize = doc2.agsi_model.iter().map(|m| m.components.len()).sum();
    if components1 != components2 {
        differences.push(format!("Component count: {} → {}", components1, components2));
        identical = false;
    }

    // Detailed comparison
    if detailed {
        // Compare each model
        for (idx, (m1, m2)) in doc1.agsi_model.iter().zip(doc2.agsi_model.iter()).enumerate() {
            if m1.id != m2.id {
                differences.push(format!("Model {} ID: {} → {}", idx, m1.id, m2.id));
                identical = false;
            }
            if m1.name != m2.name {
                differences.push(format!("Model {} name: {} → {}", idx, m1.name, m2.name));
                identical = false;
            }

            // Compare materials in model
            for mat1 in &m1.materials {
                if let Some(mat2) = m2.materials.iter().find(|m| m.id == mat1.id) {
                    if mat1.name != mat2.name {
                        differences.push(format!(
                            "Material {} name: {} → {}",
                            mat1.id, mat1.name, mat2.name
                        ));
                        identical = false;
                    }
                    if mat1.properties.len() != mat2.properties.len() {
                        differences.push(format!(
                            "Material {} properties: {} → {}",
                            mat1.id,
                            mat1.properties.len(),
                            mat2.properties.len()
                        ));
                        identical = false;
                    }
                } else {
                    differences.push(format!("Material {} removed", mat1.id));
                    identical = false;
                }
            }

            for mat2 in &m2.materials {
                if !m1.materials.iter().any(|m| m.id == mat2.id) {
                    differences.push(format!("Material {} added", mat2.id));
                    identical = false;
                }
            }
        }
    }

    // Display results
    if identical {
        println!("✅ Files are identical");
    } else {
        println!("❌ Files differ");
        println!("\n📝 Differences:");
        for diff in &differences {
            println!("   • {}", diff);
        }
        println!("\n   Total: {} differences", differences.len());
    }

    Ok(())
}
