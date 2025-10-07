# AGSi Schema Implementation - Summary

## Overview

This update implements complete AGSi JSON Schema v1.0.1 compliance in the agsi-core library. The structures now perfectly represent the official AGSi schema for ground model and interpreted data transfer.

## What Changed

### New Module: `agsi_model.rs`

Created a comprehensive new module (`crates/agsi-core/src/agsi_model.rs`) containing schema-compliant structures:

1. **AgsiRoot** - Root container for AGSi files
2. **AgsSchema** - Schema version information
3. **AgsFile** - File and project metadata
4. **AgsProject** - Optional project details
5. **AgsiModel** - Ground model container
6. **AgsiModelElement** - Materials/geological units (primary structure for materials)
7. **AgsiDataParameterValue** - Parameter values with full flexibility
8. **AgsiParameterCode** - Standard parameter codes enum
9. **AgsiDataPropertyValue** - Property values
10. **AgsiDataPropertySummary** - Statistical summaries
11. **AgsiModelBoundary** - Model spatial boundaries

### Key Features

#### 1. Materials Represented as AgsiModelElement

As specified in the AGSi schema, materials are represented using `AgsiModelElement`. Each element can contain:
- Identification (ID, name, description)
- Element type (e.g., "Geological unit")
- Geometry (flexible JSON representation)
- Multiple parameter values
- Visual attributes (color)

#### 2. Standard Parameter Codes

Implemented comprehensive support for all standard AGSi parameter codes organized by category:

**General**
- Depth, Elevation, AnalysisDrainageCondition

**Density**
- UnitWeightBulk

**Strength** (13 parameters)
- AngleFriction (and Peak, Critical, Residual variants)
- AngleDilation
- Cohesion
- UndrainedShearStrength (and Triaxial variant)
- UniaxialCompressiveStrength

**Stiffness** (10 parameters)
- YoungsModulus (Drained/Undrained, Vertical/Horizontal variants)
- BulkModulus
- ShearModulus (Drained/Undrained)
- PoissonsRatio

**Retaining Wall** (4 parameters)
- CoefficientLateralEarthPressure (AtRest, Active, Passive, Star)

**Pavement** (2 parameters)
- CBR
- SubgradeSurfaceModulus

**Permeability** (3 parameters)
- Permeability (general, Horizontal, Vertical)

**ACEC** (3 parameters)
- ACECClass, ACECDSClass, ACECCDCClass

Each parameter code provides:
- `as_code_id()` - String representation
- `units()` - Standard units (e.g., "kPa", "deg", "MPa")
- `category()` - Category classification
- `description()` - Full description

#### 3. Free Text Parameter Support

The `AgsiDataParameterValue` structure supports both:
- **Standard codes** via the `AgsiParameterCode` enum
- **Free text codes** for custom/project-specific parameters

This provides maximum flexibility while maintaining type safety for standard parameters.

#### 4. Case-Specific Parameters

Support for case-specific parameter values using the `case_id` field, allowing different values for different analysis scenarios (e.g., "Characteristic", "Conservative", "EC7Pile").

### Updated Files

1. **crates/agsi-core/src/agsi_model.rs** (NEW)
   - 650+ lines of schema-compliant structures
   - Complete parameter code enum with helpers
   - Builder pattern implementations
   - Comprehensive tests

2. **crates/agsi-core/src/lib.rs** (UPDATED)
   - Added agsi_model module
   - Exported new structures
   - Maintained backward compatibility with legacy structures

3. **crates/agsi-core/Cargo.toml** (UPDATED)
   - Added new example reference

4. **examples/agsi_model_example.rs** (NEW)
   - Comprehensive example demonstrating all features
   - Shows 5 different material types
   - Demonstrates standard codes, free text, and case-specific parameters
   - Includes JSON serialization

5. **AGSI_MODEL_STRUCTURES.md** (NEW)
   - Complete documentation of the new structures
   - Usage examples for common scenarios
   - Reference guide for parameter codes
   - Migration guidance

## Backward Compatibility

The update maintains full backward compatibility:
- Legacy `Material` and `GroundModel` structures remain available
- Existing code continues to work without modification
- New structures are opt-in

## Testing

All tests pass successfully:
- 21 unit tests in agsi-core
- New tests for AgsiRoot, AgsiModelElement, and parameter codes
- Full project builds without errors

## Usage Example

```rust
use agsi_core::{
    AgsiRoot, AgsiModel, AgsiModelElement, 
    AgsiDataParameterValue, AgsiParameterCode
};

// Create AGSi structure
let mut root = AgsiRoot::new("My Project");

// Create model
let mut model = AgsiModel::new();
model.model_name = Some("Geological Model".to_string());

// Create material using AgsiModelElement
let clay = AgsiModelElement::new()
    .with_name("London Clay")
    .with_description("Stiff to very stiff clay")
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UnitWeightBulk,
        20.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength,
        100.0
    ));

model.add_element(clay);
root.add_model(model);

// Serialize to JSON
let json = serde_json::to_string_pretty(&root)?;
```

## Running the Example

```bash
cargo run --example agsi_model_example -p agsi-core
```

This produces valid AGSi JSON output with:
- 5 different material types
- Standard and custom parameters
- Case-specific values
- Model boundary definition

## Schema Compliance

The implementation follows the AGSi JSON Schema v1.0.1 specification exactly:
- Property names match schema (camelCase)
- Optional fields use `Option<T>`
- Arrays use `Vec<T>`
- Validation using the `validator` crate
- Flexible geometry as `serde_json::Value`

## Benefits

1. **Standards Compliance** - Perfect alignment with AGSi v1.0.1
2. **Type Safety** - Rust's type system prevents errors
3. **Flexibility** - Support for both standard and custom parameters
4. **Extensibility** - Easy to add new parameter codes
5. **Documentation** - Comprehensive docs and examples
6. **Validation** - Built-in validation rules
7. **Interoperability** - Standard JSON format for data exchange

## Next Steps

To use the new structures in your code:

1. Import the new types:
   ```rust
   use agsi_core::{AgsiRoot, AgsiModel, AgsiModelElement, AgsiDataParameterValue};
   ```

2. Replace legacy Material usage with AgsiModelElement

3. Use AgsiParameterCode for standard parameters

4. Use free text for custom parameters

5. Serialize to JSON for AGSi file output

## References

- AGSi Standard: https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/
- Schema file: `/schemas/AGSi_JSONSchema_v1-0-1_2020-12.json`
- Documentation: `AGSI_MODEL_STRUCTURES.md`
- Example: `examples/agsi_model_example.rs`
