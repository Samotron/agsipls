use serde::{Deserialize, Serialize};
use validator::Validate;

/// Root AGSi structure
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AgsiRoot {
    pub ags_schema: AgsSchema,
    pub ags_file: AgsFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ags_project: Option<AgsProject>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agsi_model: Vec<AgsiModel>,
}

/// AGS schema information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AgsSchema {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

/// AGS file metadata
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AgsFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// AGS project information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsProject {
    // Project metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_project_client: Option<String>,
}

/// AGSi Model - represents a ground model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModel {
    #[serde(rename = "modelID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coord_system_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncertainty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agsi_model_element: Vec<AgsiModelElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agsi_model_boundary: Option<AgsiModelBoundary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// AGSi Model Element - represents materials/geological units in the model
/// This is what we use to represent materials as per AGSi spec
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelElement {
    #[serde(rename = "elementID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry_object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agsi_geometry: Option<serde_json::Value>, // Flexible geometry representation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agsi_geometry_area_limit: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agsi_data_parameter_value: Vec<AgsiDataParameterValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agsi_data_property_value: Vec<AgsiDataPropertyValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub agsi_data_property_summary: Vec<AgsiDataPropertySummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agsi_data_property_from_file: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour_rgb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// AGSi Data Parameter Value - represents parameter data for model elements
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataParameterValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    /// Parameter code - can be standard codes or free text
    #[serde(rename = "codeID")]
    #[validate(length(min = 1))]
    pub code_id: String,
    #[serde(rename = "caseID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_numeric: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_profile_ind_var_code_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_profile: Option<Vec<[f64; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// Standard parameter codes as per AGSi specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgsiParameterCode {
    // General
    Depth,
    Elevation,
    AnalysisDrainageCondition,
    
    // Density
    UnitWeightBulk,
    
    // Strength
    AngleFriction,
    AngleFrictionPeak,
    AngleFrictionCritical,
    AngleFrictionResidual,
    AngleDilation,
    Cohesion,
    UndrainedShearStrength,
    UndrainedShearStrengthTriaxial,
    UniaxialCompressiveStrength,
    
    // Stiffness
    YoungsModulusDrained,
    YoungsModulusUndrained,
    YoungsModulusDrainedVertical,
    YoungsModulusUndrainedVertical,
    YoungsModulusDrainedHorizontal,
    YoungsModulusUndrainedHorizontal,
    BulkModulus,
    ShearModulusDrained,
    ShearModulusUndrained,
    PoissonsRatio,
    
    // Retaining wall
    CoefficientLateralEarthPressureAtRest,
    CoefficientLateralEarthPressureActive,
    CoefficientLateralEarthPressurePassive,
    CoefficientLateralEarthPressureStar,
    
    // Pavement
    CBR,
    SubgradeSurfaceModulus,
    
    // Permeability
    Permeability,
    PermeabilityHorizontal,
    PermeabilityVertical,
    
    // ACEC
    ACECClass,
    ACECDSClass,
    ACECCDCClass,
}

impl AgsiParameterCode {
    /// Get the standard units for this parameter
    pub fn units(&self) -> Option<&'static str> {
        match self {
            Self::Depth | Self::Elevation => Some("m"),
            Self::UnitWeightBulk => Some("kN/m3"),
            Self::AngleFriction | Self::AngleFrictionPeak | Self::AngleFrictionCritical 
                | Self::AngleFrictionResidual | Self::AngleDilation => Some("deg"),
            Self::Cohesion | Self::UndrainedShearStrength | Self::UndrainedShearStrengthTriaxial => Some("kPa"),
            Self::UniaxialCompressiveStrength | Self::YoungsModulusDrained | Self::YoungsModulusUndrained
                | Self::YoungsModulusDrainedVertical | Self::YoungsModulusUndrainedVertical
                | Self::YoungsModulusDrainedHorizontal | Self::YoungsModulusUndrainedHorizontal
                | Self::BulkModulus | Self::ShearModulusDrained | Self::ShearModulusUndrained
                | Self::SubgradeSurfaceModulus => Some("MPa"),
            Self::CBR => Some("%"),
            Self::Permeability | Self::PermeabilityHorizontal | Self::PermeabilityVertical => Some("m/s"),
            _ => None,
        }
    }
    
    /// Get the category for this parameter
    pub fn category(&self) -> &'static str {
        match self {
            Self::Depth | Self::Elevation | Self::AnalysisDrainageCondition => "General",
            Self::UnitWeightBulk => "Density",
            Self::AngleFriction | Self::AngleFrictionPeak | Self::AngleFrictionCritical 
                | Self::AngleFrictionResidual | Self::AngleDilation | Self::Cohesion 
                | Self::UndrainedShearStrength | Self::UndrainedShearStrengthTriaxial => "Strength",
            Self::UniaxialCompressiveStrength | Self::YoungsModulusDrained | Self::YoungsModulusUndrained
                | Self::YoungsModulusDrainedVertical | Self::YoungsModulusUndrainedVertical
                | Self::YoungsModulusDrainedHorizontal | Self::YoungsModulusUndrainedHorizontal
                | Self::BulkModulus | Self::ShearModulusDrained | Self::ShearModulusUndrained
                | Self::PoissonsRatio => "Stiffness",
            Self::CoefficientLateralEarthPressureAtRest | Self::CoefficientLateralEarthPressureActive
                | Self::CoefficientLateralEarthPressurePassive | Self::CoefficientLateralEarthPressureStar => "Ret wall",
            Self::CBR | Self::SubgradeSurfaceModulus => "Pavement",
            Self::Permeability | Self::PermeabilityHorizontal | Self::PermeabilityVertical => "Permeability",
            Self::ACECClass | Self::ACECDSClass | Self::ACECCDCClass => "ACEC",
        }
    }
    
    /// Get the description for this parameter
    pub fn description(&self) -> &'static str {
        match self {
            Self::Depth => "Depth",
            Self::Elevation => "Elevation",
            Self::AnalysisDrainageCondition => "Drainage condition assumed for analysis: Drained or Undrained",
            Self::UnitWeightBulk => "Bulk unit weight",
            Self::AngleFriction => "Effective angle of shearing resistance",
            Self::AngleFrictionPeak => "Peak effective angle of shearing resistance",
            Self::AngleFrictionCritical => "Critical state effective angle of shearing resistance",
            Self::AngleFrictionResidual => "Residual effective angle of shearing resistance",
            Self::AngleDilation => "Angle of dilation",
            Self::Cohesion => "Effective cohesion",
            Self::UndrainedShearStrength => "Undrained shear strength",
            Self::UndrainedShearStrengthTriaxial => "Undrained shear strength from triaxial tests",
            Self::UniaxialCompressiveStrength => "Uniaxial Compressive Strength",
            Self::YoungsModulusDrained => "Drained Young's Modulus",
            Self::YoungsModulusUndrained => "Undrained Young's Modulus",
            Self::YoungsModulusDrainedVertical => "Vertical drained Young's Modulus",
            Self::YoungsModulusUndrainedVertical => "Vertical undrained Young's Modulus",
            Self::YoungsModulusDrainedHorizontal => "Horizontal drained Young's Modulus",
            Self::YoungsModulusUndrainedHorizontal => "Horizontal undrained Young's Modulus",
            Self::BulkModulus => "Bulk modulus",
            Self::ShearModulusDrained => "Drained shear Modulus",
            Self::ShearModulusUndrained => "Undrained shear Modulus",
            Self::PoissonsRatio => "Poisson's ratio",
            Self::CoefficientLateralEarthPressureAtRest => "Coefficient of earth pressure at rest",
            Self::CoefficientLateralEarthPressureActive => "Coefficient of active earth pressure",
            Self::CoefficientLateralEarthPressurePassive => "Coefficient of passive earth pressure",
            Self::CoefficientLateralEarthPressureStar => "Coefficient of earth pressure for integral bridge abutments subject to strain ratcheting",
            Self::CBR => "California bearing ratio (CBR)",
            Self::SubgradeSurfaceModulus => "Subgrade surface modulus",
            Self::Permeability => "Permeability",
            Self::PermeabilityHorizontal => "Horizontal permeability",
            Self::PermeabilityVertical => "Vertical permeability",
            Self::ACECClass => "ACEC Aggressive chemical environment class",
            Self::ACECDSClass => "ACEC Design sulphate class",
            Self::ACECCDCClass => "ACEC Design chemical class",
        }
    }
    
    /// Convert to code ID string
    pub fn as_code_id(&self) -> &'static str {
        match self {
            Self::Depth => "Depth",
            Self::Elevation => "Elevation",
            Self::AnalysisDrainageCondition => "AnalysisDrainageCondition",
            Self::UnitWeightBulk => "UnitWeightBulk",
            Self::AngleFriction => "AngleFriction",
            Self::AngleFrictionPeak => "AngleFrictionPeak",
            Self::AngleFrictionCritical => "AngleFrictionCritical",
            Self::AngleFrictionResidual => "AngleFrictionResidual",
            Self::AngleDilation => "AngleDilation",
            Self::Cohesion => "Cohesion",
            Self::UndrainedShearStrength => "UndrainedShearStrength",
            Self::UndrainedShearStrengthTriaxial => "UndrainedShearStrengthTriaxial",
            Self::UniaxialCompressiveStrength => "UniaxialCompressiveStrength",
            Self::YoungsModulusDrained => "YoungsModulusDrained",
            Self::YoungsModulusUndrained => "YoungsModulusUndrained",
            Self::YoungsModulusDrainedVertical => "YoungsModulusDrainedVertical",
            Self::YoungsModulusUndrainedVertical => "YoungsModulusUndrainedVertical",
            Self::YoungsModulusDrainedHorizontal => "YoungsModulusDrainedHorizontal",
            Self::YoungsModulusUndrainedHorizontal => "YoungsModulusUndrainedHorizontal",
            Self::BulkModulus => "BulkModulus",
            Self::ShearModulusDrained => "ShearModulusDrained",
            Self::ShearModulusUndrained => "ShearModulusUndrained",
            Self::PoissonsRatio => "PoissonsRatio",
            Self::CoefficientLateralEarthPressureAtRest => "CoefficientLateralEarthPressureAtRest",
            Self::CoefficientLateralEarthPressureActive => "CoefficientLateralEarthPressureActive",
            Self::CoefficientLateralEarthPressurePassive => "CoefficientLateralEarthPressurePassive",
            Self::CoefficientLateralEarthPressureStar => "CoefficientLateralEarthPressureStar",
            Self::CBR => "CBR",
            Self::SubgradeSurfaceModulus => "SubgradeSurfaceModulus",
            Self::Permeability => "Permeability",
            Self::PermeabilityHorizontal => "PermeabilityHorizontal",
            Self::PermeabilityVertical => "PermeabilityVertical",
            Self::ACECClass => "ACECClass",
            Self::ACECDSClass => "ACECDSClass",
            Self::ACECCDCClass => "ACECCDCClass",
        }
    }
    
    /// Try to parse from a code ID string
    pub fn from_code_id(code: &str) -> Option<Self> {
        match code {
            "Depth" => Some(Self::Depth),
            "Elevation" => Some(Self::Elevation),
            "AnalysisDrainageCondition" => Some(Self::AnalysisDrainageCondition),
            "UnitWeightBulk" => Some(Self::UnitWeightBulk),
            "AngleFriction" => Some(Self::AngleFriction),
            "AngleFrictionPeak" => Some(Self::AngleFrictionPeak),
            "AngleFrictionCritical" => Some(Self::AngleFrictionCritical),
            "AngleFrictionResidual" => Some(Self::AngleFrictionResidual),
            "AngleDilation" => Some(Self::AngleDilation),
            "Cohesion" => Some(Self::Cohesion),
            "UndrainedShearStrength" => Some(Self::UndrainedShearStrength),
            "UndrainedShearStrengthTriaxial" => Some(Self::UndrainedShearStrengthTriaxial),
            "UniaxialCompressiveStrength" => Some(Self::UniaxialCompressiveStrength),
            "YoungsModulusDrained" => Some(Self::YoungsModulusDrained),
            "YoungsModulusUndrained" => Some(Self::YoungsModulusUndrained),
            "YoungsModulusDrainedVertical" => Some(Self::YoungsModulusDrainedVertical),
            "YoungsModulusUndrainedVertical" => Some(Self::YoungsModulusUndrainedVertical),
            "YoungsModulusDrainedHorizontal" => Some(Self::YoungsModulusDrainedHorizontal),
            "YoungsModulusUndrainedHorizontal" => Some(Self::YoungsModulusUndrainedHorizontal),
            "BulkModulus" => Some(Self::BulkModulus),
            "ShearModulusDrained" => Some(Self::ShearModulusDrained),
            "ShearModulusUndrained" => Some(Self::ShearModulusUndrained),
            "PoissonsRatio" => Some(Self::PoissonsRatio),
            "CoefficientLateralEarthPressureAtRest" => Some(Self::CoefficientLateralEarthPressureAtRest),
            "CoefficientLateralEarthPressureActive" => Some(Self::CoefficientLateralEarthPressureActive),
            "CoefficientLateralEarthPressurePassive" => Some(Self::CoefficientLateralEarthPressurePassive),
            "CoefficientLateralEarthPressureStar" => Some(Self::CoefficientLateralEarthPressureStar),
            "CBR" => Some(Self::CBR),
            "SubgradeSurfaceModulus" => Some(Self::SubgradeSurfaceModulus),
            "Permeability" => Some(Self::Permeability),
            "PermeabilityHorizontal" => Some(Self::PermeabilityHorizontal),
            "PermeabilityVertical" => Some(Self::PermeabilityVertical),
            "ACECClass" => Some(Self::ACECClass),
            "ACECDSClass" => Some(Self::ACECDSClass),
            "ACECCDCClass" => Some(Self::ACECCDCClass),
            _ => None,
        }
    }
}

/// AGSi Data Property Value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertyValue {
    #[serde(rename = "dataID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "codeID")]
    pub code_id: String,
    #[serde(rename = "caseID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_numeric: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// AGSi Data Property Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiDataPropertySummary {
    #[serde(rename = "dataID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,
    #[serde(rename = "codeID")]
    pub code_id: String,
    #[serde(rename = "caseID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_mean: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

/// AGSi Model Boundary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgsiModelBoundary {
    #[serde(rename = "boundaryID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boundary_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_elevation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_elevation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

// Implementation helpers
impl AgsiRoot {
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            ags_schema: AgsSchema {
                name: "AGSi".to_string(),
                version: "1.0.1".to_string(),
                link: Some("https://ags-data-format-wg.gitlab.io/agsi/agsi_standard/1.0.1/".to_string()),
            },
            ags_file: AgsFile {
                file_uuid: None,
                title: None,
                project_name: project_name.into(),
                project_title: None,
                description: None,
                project_country: None,
                producer_project_id: None,
                client_project_id: None,
                remarks: None,
            },
            ags_project: None,
            agsi_model: Vec::new(),
        }
    }
    
    pub fn add_model(&mut self, model: AgsiModel) {
        self.agsi_model.push(model);
    }
}

impl AgsiModel {
    pub fn new() -> Self {
        Self {
            model_id: None,
            model_name: None,
            description: None,
            coord_system_id: None,
            model_type: None,
            category: None,
            domain: None,
            input: None,
            method: None,
            usage: None,
            uncertainty: None,
            document_set_id: None,
            alignment_id: None,
            agsi_model_element: Vec::new(),
            agsi_model_boundary: None,
            remarks: None,
        }
    }
    
    pub fn add_element(&mut self, element: AgsiModelElement) {
        self.agsi_model_element.push(element);
    }
}

impl AgsiModelElement {
    pub fn new() -> Self {
        Self {
            element_id: None,
            element_name: None,
            description: None,
            element_type: None,
            geometry_object: None,
            agsi_geometry: None,
            agsi_geometry_area_limit: None,
            agsi_data_parameter_value: Vec::new(),
            agsi_data_property_value: Vec::new(),
            agsi_data_property_summary: Vec::new(),
            agsi_data_property_from_file: None,
            colour_rgb: None,
            remarks: None,
        }
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.element_name = Some(name.into());
        self
    }
    
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    pub fn add_parameter(&mut self, parameter: AgsiDataParameterValue) {
        self.agsi_data_parameter_value.push(parameter);
    }
    
    pub fn with_parameter(mut self, parameter: AgsiDataParameterValue) -> Self {
        self.agsi_data_parameter_value.push(parameter);
        self
    }
}

impl AgsiDataParameterValue {
    /// Create a new parameter with numeric value
    pub fn numeric(code_id: impl Into<String>, value: f64) -> Self {
        Self {
            data_id: None,
            code_id: code_id.into(),
            case_id: None,
            value_numeric: Some(value),
            value_text: None,
            value_profile_ind_var_code_id: None,
            value_profile: None,
            remarks: None,
        }
    }
    
    /// Create a new parameter with text value
    pub fn text(code_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            data_id: None,
            code_id: code_id.into(),
            case_id: None,
            value_numeric: None,
            value_text: Some(value.into()),
            value_profile_ind_var_code_id: None,
            value_profile: None,
            remarks: None,
        }
    }
    
    /// Create from a standard parameter code
    pub fn from_standard_code(code: AgsiParameterCode, value: f64) -> Self {
        Self::numeric(code.as_code_id(), value)
    }
    
    /// Set the case ID
    pub fn with_case(mut self, case_id: impl Into<String>) -> Self {
        self.case_id = Some(case_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_agsi_root() {
        let root = AgsiRoot::new("Test Project");
        assert_eq!(root.ags_schema.name, "AGSi");
        assert_eq!(root.ags_schema.version, "1.0.1");
        assert_eq!(root.ags_file.project_name, "Test Project");
    }
    
    #[test]
    fn test_create_model_element() {
        let mut element = AgsiModelElement::new()
            .with_name("Dense Sand")
            .with_description("Dense, medium to coarse sand");
        
        element.add_parameter(AgsiDataParameterValue::numeric("UnitWeightBulk", 19.0));
        element.add_parameter(AgsiDataParameterValue::numeric("AngleFriction", 35.0));
        
        assert_eq!(element.element_name, Some("Dense Sand".to_string()));
        assert_eq!(element.agsi_data_parameter_value.len(), 2);
    }
    
    #[test]
    fn test_standard_parameter_codes() {
        let code = AgsiParameterCode::UndrainedShearStrength;
        assert_eq!(code.units(), Some("kPa"));
        assert_eq!(code.category(), "Strength");
        assert_eq!(code.as_code_id(), "UndrainedShearStrength");
    }
    
    #[test]
    fn test_free_text_parameter() {
        let param = AgsiDataParameterValue::text("CustomParameter", "Custom Value");
        assert_eq!(param.code_id, "CustomParameter");
        assert_eq!(param.value_text, Some("Custom Value".to_string()));
    }
    
    #[test]
    fn test_json_serialization() {
        let mut root = AgsiRoot::new("Test Project");
        let mut model = AgsiModel::new();
        model.model_name = Some("Test Model".to_string());
        
        let element = AgsiModelElement::new()
            .with_name("Test Material")
            .with_parameter(AgsiDataParameterValue::from_standard_code(
                AgsiParameterCode::UnitWeightBulk,
                20.0
            ));
        
        model.add_element(element);
        root.add_model(model);
        
        // Serialize to JSON
        let json = serde_json::to_string(&root).unwrap();
        assert!(json.contains("AGSi"));
        assert!(json.contains("Test Project"));
        assert!(json.contains("Test Material"));
        assert!(json.contains("UnitWeightBulk"));
        
        // Deserialize back
        let deserialized: AgsiRoot = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.ags_file.project_name, "Test Project");
        assert_eq!(deserialized.agsi_model.len(), 1);
        assert_eq!(deserialized.agsi_model[0].agsi_model_element.len(), 1);
    }
    
    #[test]
    fn test_parameter_code_lookup() {
        // Test forward lookup
        let code = AgsiParameterCode::AngleFriction;
        let code_id = code.as_code_id();
        assert_eq!(code_id, "AngleFriction");
        
        // Test reverse lookup
        let parsed = AgsiParameterCode::from_code_id(code_id);
        assert_eq!(parsed, Some(code));
    }
    
    #[test]
    fn test_all_parameter_codes_have_metadata() {
        // Ensure all parameter codes have units, category, and description
        let codes = [
            AgsiParameterCode::Depth,
            AgsiParameterCode::UnitWeightBulk,
            AgsiParameterCode::AngleFriction,
            AgsiParameterCode::UndrainedShearStrength,
            AgsiParameterCode::YoungsModulusDrained,
            AgsiParameterCode::Permeability,
            AgsiParameterCode::CBR,
        ];
        
        for code in codes {
            // All should have a description
            assert!(!code.description().is_empty());
            
            // All should have a category
            assert!(!code.category().is_empty());
            
            // All should have a code ID
            assert!(!code.as_code_id().is_empty());
        }
    }
}
