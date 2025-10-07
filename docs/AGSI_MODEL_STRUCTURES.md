# AGSi Model Structures

This document describes the AGSi schema-compliant data structures in the agsi-core library, based on the AGSi JSON Schema v1.0.1.

## Overview

The library provides Rust structures that perfectly align with the [AGSi (Association of Geotechnical & Geoenvironmental Specialists interchange format) standard](https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/). These structures enable you to create, manipulate, and serialize ground model data in a standardized format.

## Key Structures

### AgsiRoot

The root container for an AGSi file, containing:
- `ags_schema`: Schema version information
- `ags_file`: File and project metadata
- `ags_project`: Optional project details
- `agsi_model`: Array of ground models

### AgsiModel

Represents a ground model with:
- Model metadata (name, description, type, etc.)
- `agsi_model_element`: Array of model elements (materials/geological units)
- `agsi_model_boundary`: Optional boundary definition
- Usage, uncertainty, and method information

### AgsiModelElement

**This is the primary structure for representing materials in AGSi.**

Each element represents a material or geological unit and contains:
- `element_id`, `element_name`, `description`: Identification
- `element_type`: Type of element (e.g., "Geological unit")
- `agsi_geometry`: Geometry definition (flexible JSON)
- `agsi_data_parameter_value`: Array of parameter values
- `colour_rgb`: Display color

### AgsiDataParameterValue

Represents parameter data for model elements with support for:
- **Standard parameter codes** via `AgsiParameterCode` enum
- **Free text codes** for custom parameters
- Numeric values (`value_numeric`)
- Text values (`value_text`)
- Profiles (`value_profile`)
- Case-specific values (`case_id`)

## Standard Parameter Codes

The library includes comprehensive support for standard AGSi parameter codes:

### General
- `Depth` (m)
- `Elevation` (m)
- `AnalysisDrainageCondition`

### Density
- `UnitWeightBulk` (kN/m³)

### Strength
- `AngleFriction` (deg) - Effective angle of shearing resistance
- `AngleFrictionPeak` (deg)
- `AngleFrictionCritical` (deg)
- `AngleFrictionResidual` (deg)
- `AngleDilation` (deg)
- `Cohesion` (kPa) - Effective cohesion
- `UndrainedShearStrength` (kPa)
- `UndrainedShearStrengthTriaxial` (kPa)
- `UniaxialCompressiveStrength` (MPa)

### Stiffness
- `YoungsModulusDrained` (MPa)
- `YoungsModulusUndrained` (MPa)
- `YoungsModulusDrainedVertical` (MPa)
- `YoungsModulusUndrainedVertical` (MPa)
- `YoungsModulusDrainedHorizontal` (MPa)
- `YoungsModulusUndrainedHorizontal` (MPa)
- `BulkModulus` (MPa)
- `ShearModulusDrained` (MPa)
- `ShearModulusUndrained` (MPa)
- `PoissonsRatio`

### Retaining Wall
- `CoefficientLateralEarthPressureAtRest`
- `CoefficientLateralEarthPressureActive`
- `CoefficientLateralEarthPressurePassive`
- `CoefficientLateralEarthPressureStar`

### Pavement
- `CBR` (%)
- `SubgradeSurfaceModulus` (MPa)

### Permeability
- `Permeability` (m/s)
- `PermeabilityHorizontal` (m/s)
- `PermeabilityVertical` (m/s)

### ACEC (Aggressive Chemical Environment for Concrete)
- `ACECClass`
- `ACECDSClass`
- `ACECCDCClass`

## Usage Examples

### Basic Model Creation

```rust
use agsi_core::{AgsiRoot, AgsiModel, AgsiModelElement, AgsiDataParameterValue, AgsiParameterCode};

// Create root structure
let mut root = AgsiRoot::new("My Geotechnical Project");
root.ags_file.description = Some("Site investigation ground model".to_string());

// Create a model
let mut model = AgsiModel::new();
model.model_name = Some("Geological Model".to_string());

// Add the model to the root
root.add_model(model);
```

### Creating a Material using AgsiModelElement

```rust
// Dense sand material with standard parameters
let dense_sand = AgsiModelElement::new()
    .with_name("Dense Sand")
    .with_description("Dense, medium to coarse sand")
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
    ));
```

### Using Free Text Parameters

```rust
let mut fill = AgsiModelElement::new()
    .with_name("Made Ground");

// Standard parameters
fill.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::UnitWeightBulk,
    18.0
));

// Custom free text parameters
fill.add_parameter(AgsiDataParameterValue::text(
    "MaterialOrigin",
    "Demolition fill"
));
fill.add_parameter(AgsiDataParameterValue::numeric(
    "EstimatedAge",
    50.0
));
```

### Case-Specific Parameters

```rust
let mut clay = AgsiModelElement::new()
    .with_name("Clay");

// Different parameter values for different analysis cases
clay.add_parameter(
    AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength,
        100.0
    ).with_case("Characteristic")
);

clay.add_parameter(
    AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength,
        80.0
    ).with_case("Conservative")
);
```

### Getting Parameter Information

```rust
// Get standard information about parameter codes
let code = AgsiParameterCode::UndrainedShearStrength;
println!("Code ID: {}", code.as_code_id());
println!("Description: {}", code.description());
println!("Category: {}", code.category());
println!("Units: {:?}", code.units());

// Output:
// Code ID: UndrainedShearStrength
// Description: Undrained shear strength
// Category: Strength
// Units: Some("kPa")
```

### Setting Model Boundary

```rust
use agsi_core::AgsiModelBoundary;

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
```

### Serialization

```rust
// Serialize to JSON
let json = serde_json::to_string_pretty(&root)?;
println!("{}", json);

// Save to file
std::fs::write("model.agsi.json", json)?;
```

## Design Principles

1. **Schema Compliance**: Structures exactly match the AGSi JSON Schema v1.0.1
2. **Flexibility**: Support both standard enum codes and free text for extensibility
3. **Type Safety**: Rust's type system ensures correctness
4. **Validation**: Built-in validation using the `validator` crate
5. **Serialization**: Seamless JSON serialization/deserialization with `serde`

## Material Representation

In AGSi, **materials are represented using `AgsiModelElement`**. This is the standard way to define:
- Geological units
- Soil layers
- Rock formations
- Fill materials
- Hydrogeological units

Each element can have:
- Multiple parameter values (strength, stiffness, permeability, etc.)
- Property values (discrete measurements)
- Property summaries (statistical summaries)
- Geometry (spatial representation)
- Visual attributes (color for display)

## Migration from Legacy Structures

The library maintains backward compatibility with legacy `Material` and `GroundModel` structures. To use the new AGSi-compliant structures:

```rust
// New AGSi-compliant approach
use agsi_core::{AgsiRoot, AgsiModel, AgsiModelElement};

// Legacy approach (still available)
use agsi_core::{Material, GroundModel};
```

## Running the Example

To see a comprehensive example:

```bash
cargo run --example agsi_model_example -p agsi-core
```

This example demonstrates:
- Creating an AGSi root structure
- Adding models with multiple materials
- Using both standard and custom parameters
- Case-specific parameters
- ACEC parameters for concrete design
- JSON serialization

## References

- [AGSi Standard v1.0.1](https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/)
- [AGS Data Format Working Group](https://www.ags.org.uk/)
- AGSi JSON Schema: `/schemas/AGSi_JSONSchema_v1-0-1_2020-12.json`

## License

MIT OR Apache-2.0
