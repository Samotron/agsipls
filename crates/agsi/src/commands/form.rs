use agsi_core::{
    Document, Material, MaterialProperty,
    material::{MaterialType, PropertySource, PropertyValue},
    GroundModel, model::{ModelType, ModelDimension},
    project::{Project, Location},
};
use anyhow::{Context, Result};
use inquire::{Confirm, Select, Text};
use std::path::PathBuf;

pub async fn execute(item: crate::FormItem, output: Option<PathBuf>) -> Result<()> {
    match item {
        crate::FormItem::Document => create_document_form(output).await,
        crate::FormItem::Material => create_material_form(output).await,
        crate::FormItem::Model => create_model_form(output).await,
        crate::FormItem::Component => {
            println!("⚠️  Component form not yet implemented");
            Ok(())
        }
    }
}

async fn create_document_form(output: Option<PathBuf>) -> Result<()> {
    println!("📄 Create New AGSi Document (Interactive Form)\n");

    // File ID
    let file_id = Text::new("Document ID:")
        .with_default(&format!("DOC-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")))
        .prompt()?;

    // File name
    let file_name = Text::new("File name (optional):")
        .with_default("")
        .prompt()?;

    // Author
    let author = Text::new("Author:")
        .with_default(&whoami::username())
        .prompt()?;

    // Comments
    let comments = Text::new("Comments (optional):")
        .with_default("")
        .prompt()?;

    // Project info
    let add_project = Confirm::new("Add project information?")
        .with_default(false)
        .prompt()?;

    let mut doc = Document::new(&file_id)
        .with_author(&author);

    if !file_name.is_empty() {
        doc = doc.with_file_name(&file_name);
    }

    if !comments.is_empty() {
        doc = doc.with_comments(&comments);
    }

    if add_project {
        let project = create_project_form().await?;
        doc = doc.with_project(project);
    }

    // Save
    let output_path = if let Some(path) = output {
        path
    } else {
        PathBuf::from(Text::new("Output file:")
            .with_default(&format!("{}.agsi.json", file_id))
            .prompt()?)
    };

    doc.to_json_file(&output_path)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    println!("\n✅ Document created: {}", output_path.display());
    Ok(())
}

async fn create_project_form() -> Result<Project> {
    println!("\n🏗️  Project Information\n");

    let project_id = Text::new("Project ID:")
        .prompt()?;

    let project_name = Text::new("Project name:")
        .prompt()?;

    let description = Text::new("Description (optional):")
        .with_default("")
        .prompt()?;

    let client = Text::new("Client (optional):")
        .with_default("")
        .prompt()?;

    let contractor = Text::new("Contractor (optional):")
        .with_default("")
        .prompt()?;

    let mut project = Project::new(&project_id, &project_name);

    if !description.is_empty() {
        project.description = Some(description);
    }
    if !client.is_empty() {
        project = project.with_client(&client);
    }
    if !contractor.is_empty() {
        project = project.with_contractor(&contractor);
    }

    // Location
    let add_location = Confirm::new("Add location information?")
        .with_default(false)
        .prompt()?;

    if add_location {
        let location_name = Text::new("Location name:")
            .prompt()?;
        
        let location = Location::new(&location_name);
        project = project.with_location(location);
    }

    Ok(project)
}

async fn create_material_form(output: Option<PathBuf>) -> Result<()> {
    println!("🪨 Create New Material (Interactive Form)\n");

    // Material ID
    let material_id = Text::new("Material ID:")
        .with_default(&format!("MAT-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")))
        .prompt()?;

    // Material name
    let material_name = Text::new("Material name:")
        .prompt()?;

    // Material type
    let material_types = vec![
        "Soil", "Rock", "Fill", "Made Ground", 
        "Anthropogenic", "Water", "Void", "Unknown"
    ];
    let selected_type = Select::new("Material type:", material_types).prompt()?;

    let material_type = match selected_type {
        "Soil" => MaterialType::Soil,
        "Rock" => MaterialType::Rock,
        "Fill" => MaterialType::Fill,
        "Made Ground" => MaterialType::MadeGround,
        "Anthropogenic" => MaterialType::Anthropogenic,
        "Water" => MaterialType::Water,
        "Void" => MaterialType::Void,
        _ => MaterialType::Unknown,
    };

    // Description
    let description = Text::new("Description (optional):")
        .with_default("")
        .prompt()?;

    // Geology
    let geology = Text::new("Geological description (optional):")
        .with_default("")
        .prompt()?;

    let mut material = Material::new(&material_id, &material_name, material_type);

    if !description.is_empty() {
        material = material.with_description(&description);
    }
    if !geology.is_empty() {
        material.geology = Some(geology);
    }

    // Properties
    let add_properties = Confirm::new("Add material properties?")
        .with_default(true)
        .prompt()?;

    if add_properties {
        loop {
            let prop = create_property_form().await?;
            material = material.with_property(prop);

            let add_more = Confirm::new("Add another property?")
                .with_default(false)
                .prompt()?;
            
            if !add_more {
                break;
            }
        }
    }

    // Create a document with the material
    let mut doc = Document::new(uuid::Uuid::new_v4().to_string());
    let mut model = GroundModel::new(
        "temp_model",
        "Temporary model for material",
        ModelType::Geotechnical,
        ModelDimension::OneD,
    );
    model.add_material(material);
    doc.add_model(model);

    // Save
    let output_path = if let Some(path) = output {
        path
    } else {
        PathBuf::from(Text::new("Output file:")
            .with_default(&format!("{}.agsi.json", material_id))
            .prompt()?)
    };

    doc.to_json_file(&output_path)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    println!("\n✅ Material created: {}", output_path.display());
    Ok(())
}

async fn create_property_form() -> Result<MaterialProperty> {
    println!("\n  ➕ New Property\n");

    let prop_name = Text::new("  Property name (e.g., density, cohesion):")
        .prompt()?;

    let value_types = vec!["Number", "Text", "Boolean", "Range"];
    let value_type = Select::new("  Value type:", value_types).prompt()?;

    let value = match value_type {
        "Number" => {
            let num: f64 = Text::new("  Value:")
                .prompt()?
                .parse()
                .context("Invalid number")?;
            PropertyValue::Number(num)
        }
        "Text" => {
            let text = Text::new("  Value:").prompt()?;
            PropertyValue::Text(text)
        }
        "Boolean" => {
            let bool_val = Confirm::new("  Value:").prompt()?;
            PropertyValue::Boolean(bool_val)
        }
        "Range" => {
            let min: f64 = Text::new("  Minimum value:")
                .prompt()?
                .parse()
                .context("Invalid number")?;
            let max: f64 = Text::new("  Maximum value:")
                .prompt()?
                .parse()
                .context("Invalid number")?;
            PropertyValue::Range { min, max }
        }
        _ => PropertyValue::Text("".to_string()),
    };

    let unit = Text::new("  Unit (optional, e.g., kg/m3, kPa):")
        .with_default("")
        .prompt()?;

    let method = Text::new("  Test method (optional):")
        .with_default("")
        .prompt()?;

    let sources = vec!["Tested", "Estimated", "Literature", "Assumed", "Calculated"];
    let source_str = Select::new("  Source:", sources).prompt()?;
    
    let source = match source_str {
        "Tested" => PropertySource::Tested,
        "Estimated" => PropertySource::Estimated,
        "Literature" => PropertySource::Literature,
        "Assumed" => PropertySource::Assumed,
        "Calculated" => PropertySource::Calculated,
        _ => PropertySource::Assumed,
    };

    let mut prop = MaterialProperty {
        name: prop_name,
        value,
        unit: if unit.is_empty() { None } else { Some(unit) },
        method: if method.is_empty() { None } else { Some(method) },
        source: Some(source),
    };

    Ok(prop)
}

async fn create_model_form(output: Option<PathBuf>) -> Result<()> {
    println!("🗺️  Create New Ground Model (Interactive Form)\n");

    // Model ID
    let model_id = Text::new("Model ID:")
        .with_default(&format!("MODEL-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")))
        .prompt()?;

    // Model name
    let model_name = Text::new("Model name:")
        .prompt()?;

    // Model type
    let model_types = vec![
        "Stratigraphic", "Structural", "Hydrogeological",
        "Geotechnical", "Environmental", "Composite"
    ];
    let selected_type = Select::new("Model type:", model_types).prompt()?;

    let model_type = match selected_type {
        "Stratigraphic" => ModelType::Stratigraphic,
        "Structural" => ModelType::Structural,
        "Hydrogeological" => ModelType::Hydrogeological,
        "Geotechnical" => ModelType::Geotechnical,
        "Environmental" => ModelType::Environmental,
        _ => ModelType::Composite,
    };

    // Dimension
    let dimensions = vec!["1D (Borehole)", "2D (Cross-section)", "3D (Full 3D)"];
    let selected_dim = Select::new("Dimension:", dimensions).prompt()?;

    let dimension = match selected_dim {
        "1D (Borehole)" => ModelDimension::OneD,
        "2D (Cross-section)" => ModelDimension::TwoD,
        _ => ModelDimension::ThreeD,
    };

    // Description
    let description = Text::new("Description (optional):")
        .with_default("")
        .prompt()?;

    // CRS
    let crs = Text::new("Coordinate Reference System (optional, e.g., EPSG:27700):")
        .with_default("")
        .prompt()?;

    let mut model = GroundModel::new(&model_id, &model_name, model_type, dimension);

    if !description.is_empty() {
        model.description = Some(description);
    }
    if !crs.is_empty() {
        model = model.with_crs(&crs);
    }

    // Create document
    let mut doc = Document::new(uuid::Uuid::new_v4().to_string());
    doc.add_model(model);

    // Save
    let output_path = if let Some(path) = output {
        path
    } else {
        PathBuf::from(Text::new("Output file:")
            .with_default(&format!("{}.agsi.json", model_id))
            .prompt()?)
    };

    doc.to_json_file(&output_path)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    println!("\n✅ Model created: {}", output_path.display());
    Ok(())
}
