/// Example demonstrating the AGSi schema-compliant model structures
/// 
/// This example shows how to:
/// - Create an AGSi root structure
/// - Add models with elements (materials)
/// - Use AgsiModelElement to represent materials
/// - Add parameter values using both standard codes and free text
/// - Serialize to JSON

use agsi_core::{
    AgsiRoot, AgsiModel, AgsiModelElement, AgsiDataParameterValue, 
    AgsiParameterCode, AgsiModelBoundary,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the root AGSi structure
    let mut root = AgsiRoot::new("Example Geotechnical Project");
    
    // Add project metadata
    root.ags_file.title = Some("Ground Model for Example Site".to_string());
    root.ags_file.description = Some("Geological model based on 2024 site investigation".to_string());
    root.ags_file.project_country = Some("United Kingdom".to_string());
    
    // Create a model
    let mut model = AgsiModel::new();
    model.model_name = Some("Sitewide Geological Model".to_string());
    model.description = Some("3D geological model incorporating all site investigation data".to_string());
    model.model_type = Some("Geological model".to_string());
    model.category = Some("Observational".to_string());
    model.domain = Some("Engineering geology".to_string());
    model.usage = Some("For reference and visualization. Not suitable for direct use in design.".to_string());
    
    // Add model boundary
    model.agsi_model_boundary = Some(AgsiModelBoundary {
        boundary_id: Some("SiteBoundary".to_string()),
        description: Some("Site boundary box".to_string()),
        min_x: Some(520000.0),
        max_x: Some(521000.0),
        min_y: Some(180000.0),
        max_y: Some(181000.0),
        top_elevation: Some(50.0),
        bottom_elevation: Some(-30.0),
        remarks: None,
    });
    
    // Example 1: Dense Sand material using AgsiModelElement
    let dense_sand = AgsiModelElement::new()
        .with_name("Dense Sand")
        .with_description("Dense, medium to coarse sand with occasional gravel")
        .with_parameter(AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::UnitWeightBulk,
            19.0
        ))
        .with_parameter(AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::AngleFriction,
            35.0
        ))
        .with_parameter(AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::Cohesion,
            0.0
        ))
        .with_parameter(AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::YoungsModulusDrained,
            30.0
        ))
        .with_parameter(AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::PoissonsRatio,
            0.3
        ));
    
    // Example 2: Clay material with comprehensive properties
    let mut clay = AgsiModelElement::new()
        .with_name("London Clay")
        .with_description("Stiff to very stiff slightly sandy blue/grey CLAY");
    
    // Add standard parameters using enum codes
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UnitWeightBulk,
        20.0
    ));
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength,
        100.0
    ));
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::YoungsModulusUndrained,
        50.0
    ));
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::PoissonsRatio,
        0.5
    ));
    
    // Add drained strength parameters
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::AngleFriction,
        25.0
    ));
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::Cohesion,
        5.0
    ));
    
    // Add permeability
    clay.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::PermeabilityVertical,
        1e-9
    ));
    
    // Example 3: Material with custom/free text parameters
    let mut fill = AgsiModelElement::new()
        .with_name("Made Ground")
        .with_description("Variable fill material containing brick, concrete, ash and soil");
    
    // Standard parameters
    fill.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UnitWeightBulk,
        18.0
    ));
    fill.add_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::AngleFriction,
        30.0
    ));
    
    // Free text custom parameters
    fill.add_parameter(AgsiDataParameterValue::text(
        "MaterialOrigin",
        "Historical demolition fill"
    ));
    fill.add_parameter(AgsiDataParameterValue::numeric(
        "EstimatedAge",
        50.0
    ));
    fill.add_parameter(AgsiDataParameterValue::text(
        "ContaminationRisk",
        "Low to moderate"
    ));
    
    // Example 4: Material with case-specific parameters
    let mut gravel = AgsiModelElement::new()
        .with_name("River Terrace Deposits")
        .with_description("Dense sandy gravel");
    
    // Parameters for different analysis cases
    gravel.add_parameter(
        AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::AngleFriction,
            38.0
        ).with_case("Characteristic")
    );
    gravel.add_parameter(
        AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::AngleFriction,
            35.0
        ).with_case("Conservative")
    );
    gravel.add_parameter(
        AgsiDataParameterValue::from_standard_code(
            AgsiParameterCode::UnitWeightBulk,
            20.0
        )
    );
    
    // Example 5: Material with ACEC parameters (for concrete design)
    let mut aggressive_ground = AgsiModelElement::new()
        .with_name("Aggressive Groundwater Zone")
        .with_description("Zone with aggressive groundwater chemistry");
    
    aggressive_ground.add_parameter(AgsiDataParameterValue::text(
        "ACECClass",
        "AC-3"
    ));
    aggressive_ground.add_parameter(AgsiDataParameterValue::text(
        "ACECDSClass",
        "DS-2"
    ));
    aggressive_ground.add_parameter(AgsiDataParameterValue::numeric(
        "SulphateConcentration",
        2400.0
    ));
    
    // Add all materials to the model
    model.add_element(dense_sand);
    model.add_element(clay);
    model.add_element(fill);
    model.add_element(gravel);
    model.add_element(aggressive_ground);
    
    // Add model to root
    root.add_model(model);
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&root)?;
    println!("AGSi JSON Output:");
    println!("{}", json);
    
    // Show how to access parameter information
    println!("\n\n=== Parameter Information ===\n");
    for code in [
        AgsiParameterCode::UndrainedShearStrength,
        AgsiParameterCode::AngleFriction,
        AgsiParameterCode::YoungsModulusDrained,
        AgsiParameterCode::Permeability,
    ] {
        println!("Code: {}", code.as_code_id());
        println!("Description: {}", code.description());
        println!("Category: {}", code.category());
        if let Some(units) = code.units() {
            println!("Units: {}", units);
        }
        println!();
    }
    
    Ok(())
}
