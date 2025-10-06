use agsi_core::{
    Document, Material, MaterialProperty, GroundModel,
    material::{MaterialType, PropertySource},
    model::{ModelType, ModelDimension, ComponentType, ModelComponent, ModelExtent},
    geometry::Geometry,
    project::Project,
    validation,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a comprehensive AGSi ground model example...\n");

    // Create project information
    let project = Project::new("PROJ2024-001", "City Centre Development")
        .with_client("Urban Development Corp")
        .with_contractor("Ground Investigation Ltd");

    // Create document
    let mut doc = Document::new("DOC2024-001")
        .with_file_name("city-centre-site.agsi.json")
        .with_author("Geotechnical Engineering Team")
        .with_project(project);

    // Create materials
    let made_ground = Material::new("MAT001", "Made Ground", MaterialType::MadeGround)
        .with_description("Mixed fill material with brick and concrete fragments")
        .with_property(
            MaterialProperty::numeric("bulk_density", 1800.0, Some("kg/m3".to_string()))
                .with_source(PropertySource::Estimated)
        );

    let london_clay = Material::new("MAT002", "London Clay", MaterialType::Soil)
        .with_description("Stiff to very stiff, fissured, grey-brown clay")
        .with_property(
            MaterialProperty::numeric("undrained_shear_strength", 100.0, Some("kPa".to_string()))
                .with_source(PropertySource::Tested)
                .with_method("Triaxial UU")
        )
        .with_property(
            MaterialProperty::range("plasticity_index", 35.0, 50.0, Some("%".to_string()))
        )
        .with_property(
            MaterialProperty::numeric("bulk_density", 2000.0, Some("kg/m3".to_string()))
                .with_source(PropertySource::Tested)
        );

    let terrace_gravel = Material::new("MAT003", "River Terrace Deposits", MaterialType::Soil)
        .with_description("Dense sandy gravel")
        .with_property(
            MaterialProperty::numeric("relative_density", 75.0, Some("%".to_string()))
                .with_source(PropertySource::Tested)
        )
        .with_property(
            MaterialProperty::numeric("friction_angle", 38.0, Some("degrees".to_string()))
                .with_source(PropertySource::Tested)
        );

    // Create ground model
    let mut model = GroundModel::new(
        "MODEL001",
        "Site Stratigraphy - 2D Cross Section",
        ModelType::Stratigraphic,
        ModelDimension::TwoD,
    )
    .with_crs("EPSG:27700")
    .with_extent(ModelExtent::new_3d(
        530000.0, 530200.0,  // X extent
        180000.0, 180200.0,  // Y extent
        -20.0, 5.0,          // Z extent (elevation)
    ));

    // Add materials
    model.add_material(made_ground);
    model.add_material(london_clay);
    model.add_material(terrace_gravel);

    // Create layers as components
    let made_ground_layer = ModelComponent::new(
        "COMP001",
        "Made Ground Layer",
        ComponentType::Layer,
        "MAT001",
        Geometry::polygon(
            vec![
                [530000.0, 180000.0, 5.0],
                [530200.0, 180000.0, 5.0],
                [530200.0, 180200.0, 3.0],
                [530000.0, 180200.0, 3.0],
                [530000.0, 180000.0, 5.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(5.0, 3.0)
    .with_attribute("confidence", "medium")
    .with_attribute("interpretation_method", "borehole_correlation");

    let gravel_layer = ModelComponent::new(
        "COMP002",
        "River Terrace Gravel",
        ComponentType::Layer,
        "MAT003",
        Geometry::polygon(
            vec![
                [530000.0, 180000.0, 3.0],
                [530200.0, 180000.0, 3.0],
                [530200.0, 180200.0, -2.0],
                [530000.0, 180200.0, -2.0],
                [530000.0, 180000.0, 3.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(3.0, -2.0)
    .with_attribute("confidence", "high");

    let clay_layer = ModelComponent::new(
        "COMP003",
        "London Clay Formation",
        ComponentType::Layer,
        "MAT002",
        Geometry::polygon(
            vec![
                [530000.0, 180000.0, -2.0],
                [530200.0, 180000.0, -2.0],
                [530200.0, 180200.0, -20.0],
                [530000.0, 180200.0, -20.0],
                [530000.0, 180000.0, -2.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(-2.0, -20.0)
    .with_attribute("confidence", "high");

    model.add_component(made_ground_layer);
    model.add_component(gravel_layer);
    model.add_component(clay_layer);

    // Add model to document
    doc.add_model(model);

    // Validate
    println!("Validating document...");
    let validation_result = validation::validate_document(&doc)?;
    
    if validation_result.is_valid() {
        println!("✅ Document is valid!\n");
    } else {
        println!("❌ Validation errors found:\n");
        for error in validation_result.errors() {
            println!("  - {}: {}", error.path, error.message);
        }
    }

    // Display summary
    println!("Document Summary:");
    println!("  Project: {}", doc.ags_project.as_ref().unwrap().name);
    println!("  Models: {}", doc.agsi_model.len());
    println!("  Materials: {}", doc.agsi_model[0].materials.len());
    println!("  Components: {}", doc.agsi_model[0].components.len());

    // Save to file
    let output_path = "/tmp/example-ground-model.agsi.json";
    doc.to_json_file(output_path)?;
    println!("\n✅ Saved to: {}", output_path);

    Ok(())
}
