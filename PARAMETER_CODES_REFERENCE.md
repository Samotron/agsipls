# AGSi Parameter Codes Quick Reference

This is a quick reference for the standard AGSi parameter codes supported in the library.

## Usage

```rust
use agsi_core::{AgsiDataParameterValue, AgsiParameterCode};

// Using standard code
let param = AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::UndrainedShearStrength,
    100.0
);

// Using free text (for custom parameters)
let custom = AgsiDataParameterValue::numeric("CustomParam", 50.0);
```

## Standard Parameter Codes

| Code | Description | Units | Category |
|------|-------------|-------|----------|
| `Depth` | Depth | m | General |
| `Elevation` | Elevation | m | General |
| `AnalysisDrainageCondition` | Drainage condition assumed for analysis: Drained or Undrained | - | General |
| `UnitWeightBulk` | Bulk unit weight | kN/m³ | Density |
| `AngleFriction` | Effective angle of shearing resistance | deg | Strength |
| `AngleFrictionPeak` | Peak effective angle of shearing resistance | deg | Strength |
| `AngleFrictionCritical` | Critical state effective angle of shearing resistance | deg | Strength |
| `AngleFrictionResidual` | Residual effective angle of shearing resistance | deg | Strength |
| `AngleDilation` | Angle of dilation | deg | Strength |
| `Cohesion` | Effective cohesion | kPa | Strength |
| `UndrainedShearStrength` | Undrained shear strength | kPa | Strength |
| `UndrainedShearStrengthTriaxial` | Undrained shear strength from triaxial tests | kPa | Strength |
| `UniaxialCompressiveStrength` | Uniaxial Compressive Strength | MPa | Strength |
| `YoungsModulusDrained` | Drained Young's Modulus | MPa | Stiffness |
| `YoungsModulusUndrained` | Undrained Young's Modulus | MPa | Stiffness |
| `YoungsModulusDrainedVertical` | Vertical drained Young's Modulus | MPa | Stiffness |
| `YoungsModulusUndrainedVertical` | Vertical undrained Young's Modulus | MPa | Stiffness |
| `YoungsModulusDrainedHorizontal` | Horizontal drained Young's Modulus | MPa | Stiffness |
| `YoungsModulusUndrainedHorizontal` | Horizontal undrained Young's Modulus | MPa | Stiffness |
| `BulkModulus` | Bulk modulus | MPa | Stiffness |
| `ShearModulusDrained` | Drained shear Modulus | MPa | Stiffness |
| `ShearModulusUndrained` | Undrained shear Modulus | MPa | Stiffness |
| `PoissonsRatio` | Poisson's ratio | - | Stiffness |
| `CoefficientLateralEarthPressureAtRest` | Coefficient of earth pressure at rest | - | Ret wall |
| `CoefficientLateralEarthPressureActive` | Coefficient of active earth pressure | - | Ret wall |
| `CoefficientLateralEarthPressurePassive` | Coefficient of passive earth pressure | - | Ret wall |
| `CoefficientLateralEarthPressureStar` | Coefficient of earth pressure for integral bridge abutments subject to strain ratcheting | - | Ret wall |
| `CBR` | California bearing ratio (CBR) | % | Pavement |
| `SubgradeSurfaceModulus` | Subgrade surface modulus | MPa | Pavement |
| `Permeability` | Permeability | m/s | Permeability |
| `PermeabilityHorizontal` | Horizontal permeability | m/s | Permeability |
| `PermeabilityVertical` | Vertical permeability | m/s | Permeability |
| `ACECClass` | ACEC Aggressive chemical environment class | - | ACEC |
| `ACECDSClass` | ACEC Design sulphate class | - | ACEC |
| `ACECCDCClass` | ACEC Design chemical class | - | ACEC |

## Code Examples by Category

### Strength Parameters

```rust
// Clay with undrained strength
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::UndrainedShearStrength,
    100.0
));

// Sand with friction angle and cohesion
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::AngleFriction,
    35.0
));
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::Cohesion,
    0.0
));
```

### Stiffness Parameters

```rust
// Elastic modulus and Poisson's ratio
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::YoungsModulusDrained,
    30.0
));
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::PoissonsRatio,
    0.3
));
```

### Permeability Parameters

```rust
// Anisotropic permeability
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::PermeabilityHorizontal,
    1e-8
));
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::PermeabilityVertical,
    1e-9
));
```

### Density Parameters

```rust
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::UnitWeightBulk,
    20.0
));
```

### Pavement Parameters

```rust
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::CBR,
    5.0
));
element.add_parameter(AgsiDataParameterValue::from_standard_code(
    AgsiParameterCode::SubgradeSurfaceModulus,
    30.0
));
```

### ACEC Parameters (for concrete design)

```rust
// Text-based ACEC classifications
element.add_parameter(AgsiDataParameterValue::text(
    "ACECClass",
    "AC-3"
));
element.add_parameter(AgsiDataParameterValue::text(
    "ACECDSClass",
    "DS-2"
));
```

## Case-Specific Parameters

```rust
// Different values for different design cases
element.add_parameter(
    AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength,
        100.0
    ).with_case("Characteristic")
);

element.add_parameter(
    AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength,
        80.0
    ).with_case("Conservative")
);
```

## Custom Parameters

```rust
// Free text parameter codes
element.add_parameter(AgsiDataParameterValue::text(
    "MaterialOrigin",
    "Natural deposit"
));

element.add_parameter(AgsiDataParameterValue::numeric(
    "PlasticityIndex",
    25.0
));

element.add_parameter(AgsiDataParameterValue::text(
    "GeologicalAge",
    "Quaternary"
));
```

## Getting Parameter Metadata

```rust
let code = AgsiParameterCode::UndrainedShearStrength;

// Get the code ID string
let code_id = code.as_code_id(); // "UndrainedShearStrength"

// Get standard units
let units = code.units(); // Some("kPa")

// Get category
let category = code.category(); // "Strength"

// Get description
let desc = code.description(); // "Undrained shear strength"

// Parse from string
let parsed = AgsiParameterCode::from_code_id("AngleFriction");
if let Some(angle_code) = parsed {
    println!("Units: {:?}", angle_code.units()); // Some("deg")
}
```

## Common Material Definitions

### Sand

```rust
let sand = AgsiModelElement::new()
    .with_name("Dense Sand")
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UnitWeightBulk, 19.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::AngleFriction, 35.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::Cohesion, 0.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::YoungsModulusDrained, 30.0
    ));
```

### Clay

```rust
let clay = AgsiModelElement::new()
    .with_name("Stiff Clay")
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UnitWeightBulk, 20.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UndrainedShearStrength, 100.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::YoungsModulusUndrained, 50.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::PoissonsRatio, 0.5
    ));
```

### Rock

```rust
let rock = AgsiModelElement::new()
    .with_name("Limestone")
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UnitWeightBulk, 24.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::UniaxialCompressiveStrength, 50.0
    ))
    .with_parameter(AgsiDataParameterValue::from_standard_code(
        AgsiParameterCode::YoungsModulusDrained, 15000.0
    ));
```

## Notes

- All numeric parameter values should use appropriate units as shown in the table
- Free text parameters can be used for any custom properties not covered by standard codes
- Case IDs allow multiple parameter values for different analysis scenarios
- Standard codes provide type safety and metadata (units, descriptions, categories)
