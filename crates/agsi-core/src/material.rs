use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

/// A material representing geotechnical properties
///
/// Materials can be used independently and are the fundamental building blocks
/// of ground models in AGSi.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    /// Unique identifier for the material
    #[validate(length(min = 1))]
    pub id: String,

    /// Human-readable name for the material
    #[validate(length(min = 1))]
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Material type (e.g., "SOIL", "ROCK", "FILL", "MADE_GROUND")
    pub material_type: MaterialType,

    /// Geological classification or description
    pub geology: Option<String>,

    /// Engineering properties
    #[serde(default)]
    pub properties: Vec<MaterialProperty>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type of geotechnical material
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MaterialType {
    Soil,
    Rock,
    Fill,
    MadeGround,
    Anthropogenic,
    Water,
    Void,
    Unknown,
}

/// A property of a material with value and optional unit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialProperty {
    /// Property name (e.g., "density", "cohesion", "friction_angle")
    pub name: String,

    /// Property value
    pub value: PropertyValue,

    /// Unit of measurement
    pub unit: Option<String>,

    /// Measurement method or standard
    pub method: Option<String>,

    /// Property source (e.g., "TESTED", "ESTIMATED", "LITERATURE")
    pub source: Option<PropertySource>,
}

/// Value types for material properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Range { min: f64, max: f64 },
    Array(Vec<f64>),
}

/// Source of property value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PropertySource {
    Tested,
    Estimated,
    Literature,
    Assumed,
    Calculated,
}

impl Material {
    /// Create a new material with basic information
    pub fn new(id: impl Into<String>, name: impl Into<String>, material_type: MaterialType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            material_type,
            geology: None,
            properties: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a property to the material
    pub fn with_property(mut self, property: MaterialProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Add a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get a property by name
    pub fn get_property(&self, name: &str) -> Option<&MaterialProperty> {
        self.properties.iter().find(|p| p.name == name)
    }

    /// Get all properties of a specific type
    pub fn get_properties_by_name(&self, name: &str) -> Vec<&MaterialProperty> {
        self.properties.iter().filter(|p| p.name == name).collect()
    }
}

impl MaterialProperty {
    /// Create a new numeric property
    pub fn numeric(name: impl Into<String>, value: f64, unit: Option<String>) -> Self {
        Self {
            name: name.into(),
            value: PropertyValue::Number(value),
            unit,
            method: None,
            source: None,
        }
    }

    /// Create a new text property
    pub fn text(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: PropertyValue::Text(value.into()),
            unit: None,
            method: None,
            source: None,
        }
    }

    /// Create a range property
    pub fn range(name: impl Into<String>, min: f64, max: f64, unit: Option<String>) -> Self {
        Self {
            name: name.into(),
            value: PropertyValue::Range { min, max },
            unit,
            method: None,
            source: None,
        }
    }

    /// Set the property source
    pub fn with_source(mut self, source: PropertySource) -> Self {
        self.source = Some(source);
        self
    }

    /// Set the measurement method
    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_material() {
        let material = Material::new("MAT001", "Dense Sand", MaterialType::Soil)
            .with_description("Dense, medium to coarse sand")
            .with_property(
                MaterialProperty::numeric("density", 1900.0, Some("kg/m3".to_string()))
                    .with_source(PropertySource::Tested),
            )
            .with_property(
                MaterialProperty::numeric("friction_angle", 35.0, Some("degrees".to_string()))
                    .with_source(PropertySource::Tested),
            );

        assert_eq!(material.id, "MAT001");
        assert_eq!(material.name, "Dense Sand");
        assert_eq!(material.properties.len(), 2);
        assert!(material.get_property("density").is_some());
    }

    #[test]
    fn test_property_range() {
        let prop = MaterialProperty::range("cohesion", 10.0, 25.0, Some("kPa".to_string()));
        
        match prop.value {
            PropertyValue::Range { min, max } => {
                assert_eq!(min, 10.0);
                assert_eq!(max, 25.0);
            }
            _ => panic!("Expected range value"),
        }
    }
}
