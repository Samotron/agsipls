use agsi_core::{
    Document, Material, MaterialProperty, GroundModel,
    material::{MaterialType, PropertySource, PropertyValue},
    model::{ModelType, ModelDimension, ComponentType, ModelComponent, ModelExtent},
    geometry::Geometry,
    project::{Project, Location},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating a complete 3D ground model...\n");

    // Create project
    let location = Location::new("Silvertown, London")
        .with_coordinates(-0.0249, 51.5033, Some("EPSG:4326".into()));
    
    let project = Project::new("SILVER-2024", "Silvertown Tunnel Site Investigation")
        .with_client("Transport for London")
        .with_contractor("Ground Investigation Ltd")
        .with_location(location);

    // Create document
    let mut doc = Document::new("DOC-SILVER-001")
        .with_file_name("silvertown-3d-model.agsi.json")
        .with_author("Senior Geotechnical Engineer")
        .with_project(project);

    // Create detailed materials
    let made_ground = Material::new("MAT001", "Made Ground", MaterialType::MadeGround)
        .with_description("Heterogeneous fill comprising brick, concrete fragments, sand and gravel")
        .with_property(
            MaterialProperty::range("bulk_density", 1700.0, 1900.0, Some("kg/m3".into()))
                .with_source(PropertySource::Estimated)
        )
        .with_property(
            MaterialProperty::text("consistency", "Loose to medium dense")
        );

    let alluvium = Material::new("MAT002", "Alluvium", MaterialType::Soil)
        .with_description("Soft to firm, silty clay with occasional peat lenses")
        .with_property(
            MaterialProperty::numeric("undrained_shear_strength", 35.0, Some("kPa".into()))
                .with_source(PropertySource::Tested)
                .with_method("Triaxial UU")
        )
        .with_property(
            MaterialProperty::range("water_content", 25.0, 45.0, Some("%".into()))
                .with_source(PropertySource::Tested)
        )
        .with_property(
            MaterialProperty::numeric("plasticity_index", 28.0, Some("%".into()))
        );

    let terrace_gravel = Material::new("MAT003", "River Terrace Deposits", MaterialType::Soil)
        .with_description("Dense, sandy GRAVEL with occasional cobbles")
        .with_property(
            MaterialProperty::numeric("spt_n", 42.0, Some("blows/300mm".into()))
                .with_source(PropertySource::Tested)
        )
        .with_property(
            MaterialProperty::numeric("friction_angle", 38.0, Some("degrees".into()))
                .with_source(PropertySource::Estimated)
        )
        .with_property(
            MaterialProperty::numeric("permeability", 1e-4, Some("m/s".into()))
                .with_source(PropertySource::Tested)
        );

    let london_clay = Material::new("MAT004", "London Clay", MaterialType::Soil)
        .with_description("Very stiff to hard, fissured, grey-brown CLAY")
        .with_property(
            MaterialProperty::numeric("undrained_shear_strength", 150.0, Some("kPa".into()))
                .with_source(PropertySource::Tested)
                .with_method("Triaxial UU")
        )
        .with_property(
            MaterialProperty::range("plasticity_index", 40.0, 55.0, Some("%".into()))
        )
        .with_property(
            MaterialProperty::numeric("bulk_density", 2050.0, Some("kg/m3".into()))
                .with_source(PropertySource::Tested)
        )
        .with_property(
            MaterialProperty::numeric("cohesion", 15.0, Some("kPa".into()))
                .with_source(PropertySource::Calculated)
        );

    let chalk = Material::new("MAT005", "Thanet Sand", MaterialType::Soil)
        .with_description("Dense to very dense, fine to medium SAND")
        .with_property(
            MaterialProperty::numeric("spt_n", 55.0, Some("blows/300mm".into()))
                .with_source(PropertySource::Tested)
        )
        .with_property(
            MaterialProperty::numeric("friction_angle", 40.0, Some("degrees".into()))
        );

    // Create 3D ground model
    let mut model = GroundModel::new(
        "MODEL-3D-001",
        "Silvertown 3D Geological Model",
        ModelType::Stratigraphic,
        ModelDimension::ThreeD,
    )
    .with_crs("EPSG:27700") // British National Grid
    .with_extent(ModelExtent::new_3d(
        540000.0, 541000.0,  // 1km x extent
        179000.0, 180000.0,  // 1km y extent
        -50.0, 5.0,          // 55m vertical extent
    ));

    // Add materials
    model.add_material(made_ground);
    model.add_material(alluvium);
    model.add_material(terrace_gravel);
    model.add_material(london_clay);
    model.add_material(chalk);

    // Create 3D volumes using surface geometry
    // In a real application, these would be proper OBJ files
    let made_ground_vol = ModelComponent::new(
        "VOL001",
        "Made Ground Volume",
        ComponentType::Volume,
        "MAT001",
        Geometry::polygon(
            vec![
                [540000.0, 179000.0, 5.0],
                [541000.0, 179000.0, 5.0],
                [541000.0, 180000.0, 4.5],
                [540000.0, 180000.0, 4.5],
                [540000.0, 179000.0, 5.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(5.0, 2.0)
    .with_attribute("confidence", "medium")
    .with_attribute("data_density", "high");

    let alluvium_vol = ModelComponent::new(
        "VOL002",
        "Alluvium Volume",
        ComponentType::Volume,
        "MAT002",
        Geometry::polygon(
            vec![
                [540000.0, 179000.0, 2.0],
                [541000.0, 179000.0, 2.0],
                [541000.0, 180000.0, 1.5],
                [540000.0, 180000.0, 1.5],
                [540000.0, 179000.0, 2.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(2.0, -5.0)
    .with_attribute("confidence", "high");

    let gravel_vol = ModelComponent::new(
        "VOL003",
        "River Terrace Gravel Volume",
        ComponentType::Volume,
        "MAT003",
        Geometry::polygon(
            vec![
                [540000.0, 179000.0, -5.0],
                [541000.0, 179000.0, -5.0],
                [541000.0, 180000.0, -6.0],
                [540000.0, 180000.0, -6.0],
                [540000.0, 179000.0, -5.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(-5.0, -12.0)
    .with_attribute("confidence", "high");

    let clay_vol = ModelComponent::new(
        "VOL004",
        "London Clay Formation",
        ComponentType::Volume,
        "MAT004",
        Geometry::polygon(
            vec![
                [540000.0, 179000.0, -12.0],
                [541000.0, 179000.0, -12.0],
                [541000.0, 180000.0, -13.0],
                [540000.0, 180000.0, -13.0],
                [540000.0, 179000.0, -12.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(-12.0, -45.0)
    .with_attribute("confidence", "high");

    let sand_vol = ModelComponent::new(
        "VOL005",
        "Thanet Sand",
        ComponentType::Volume,
        "MAT005",
        Geometry::polygon(
            vec![
                [540000.0, 179000.0, -45.0],
                [541000.0, 179000.0, -45.0],
                [541000.0, 180000.0, -46.0],
                [540000.0, 180000.0, -46.0],
                [540000.0, 179000.0, -45.0],
            ],
            vec![],
        )?,
    )
    .with_elevations(-45.0, -50.0)
    .with_attribute("confidence", "medium");

    model.add_component(made_ground_vol);
    model.add_component(alluvium_vol);
    model.add_component(gravel_vol);
    model.add_component(clay_vol);
    model.add_component(sand_vol);

    // Add model to document
    doc.add_model(model);

    // Validate
    println!("Validating 3D model...");
    let validation_result = agsi_core::validation::validate_document(&doc)?;
    
    if validation_result.is_valid() {
        println!("✅ 3D model is valid!\n");
    } else {
        println!("❌ Validation errors:\n");
        for error in validation_result.errors() {
            println!("  - {}: {}", error.path, error.message);
        }
    }

    // Display summary
    println!("3D Ground Model Summary:");
    println!("  Project: {}", doc.ags_project.as_ref().unwrap().name);
    println!("  Location: Silvertown, London");
    println!("  Models: {}", doc.agsi_model.len());
    println!("  Materials: {}", doc.agsi_model[0].materials.len());
    println!("  Volumes: {}", doc.agsi_model[0].components.len());
    println!("  Extent: 1km x 1km x 55m");
    println!("  CRS: EPSG:27700 (British National Grid)");

    // Save
    let output_path = "/tmp/silvertown-3d-model.agsi.json";
    doc.to_json_file(output_path)?;
    println!("\n✅ Saved to: {}", output_path);

    Ok(())
}
