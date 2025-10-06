use agsi_core::{serialization, Document};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub async fn execute(input: PathBuf, output: PathBuf, format: String) -> Result<()> {
    println!("🔄 Converting: {} -> {}", input.display(), output.display());

    // Load document
    let doc = Document::from_json_file(&input)
        .with_context(|| format!("Failed to load file: {}", input.display()))?;

    // Determine output format
    let format_enum = match format.to_lowercase().as_str() {
        "json" => serialization::Format::Json,
        "json-compact" => serialization::Format::JsonCompact,
        "avro" => serialization::Format::Avro,
        "protobuf" | "proto" | "pb" => serialization::Format::Protobuf,
        _ => anyhow::bail!("Unsupported format: {}. Use json, avro, or protobuf", format),
    };

    // Serialize
    let data = serialization::serialize(&doc, format_enum)
        .with_context(|| format!("Failed to serialize to {}", format))?;

    // Write output
    std::fs::write(&output, data)
        .with_context(|| format!("Failed to write to {}", output.display()))?;

    println!("✅ Converted successfully");
    println!("   Format: {}", format);
    println!("   Output: {}", output.display());

    Ok(())
}
