use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::geometry::Geometry;
use crate::material::Material;

/// A ground model representing geological interpretation
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GroundModel {
    /// Unique identifier for the model
    #[validate(length(min = 1))]
    pub id: String,

    /// Human-readable name
    #[validate(length(min = 1))]
    pub name: String,

    /// Optional description
    pub description: Option<String>,

    /// Model type (e.g., "STRATIGRAPHIC", "STRUCTURAL", "HYDROGEOLOGICAL")
    pub model_type: ModelType,

    /// Dimensionality of the model
    pub dimension: ModelDimension,

    /// Components that make up the model
    #[serde(default)]
    pub components: Vec<ModelComponent>,

    /// Materials used in this model
    #[serde(default)]
    pub materials: Vec<Material>,

    /// Coordinate reference system
    pub crs: Option<String>,

    /// Model extent/bounding box
    pub extent: Option<ModelExtent>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Type of ground model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelType {
    Stratigraphic,
    Structural,
    Hydrogeological,
    Geotechnical,
    Environmental,
    Composite,
}

/// Spatial dimension of the model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModelDimension {
    OneD,   // Boreholes, CPTs
    TwoD,   // Cross-sections
    ThreeD, // Full 3D models
}

/// A component within a ground model (e.g., a layer, fault, or volume)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelComponent {
    /// Unique identifier
    pub id: String,

    /// Component name
    pub name: String,

    /// Component type
    pub component_type: ComponentType,

    /// Reference to material ID
    pub material_id: String,

    /// Geometric representation
    pub geometry: Geometry,

    /// Top elevation/depth
    pub top: Option<f64>,

    /// Base elevation/depth
    pub base: Option<f64>,

    /// Thickness
    pub thickness: Option<f64>,

    /// Additional attributes
    #[serde(default)]
    pub attributes: HashMap<String, serde_json::Value>,
}

/// Type of model component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentType {
    Layer,
    Lens,
    Volume,
    Fault,
    Intrusion,
    Boundary,
}

/// Spatial extent of a model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelExtent {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: Option<f64>,
    pub max_z: Option<f64>,
}

impl GroundModel {
    /// Create a new ground model
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        model_type: ModelType,
        dimension: ModelDimension,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            model_type,
            dimension,
            components: Vec::new(),
            materials: Vec::new(),
            crs: None,
            extent: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a component to the model
    pub fn add_component(&mut self, component: ModelComponent) {
        self.components.push(component);
    }

    /// Add a material to the model
    pub fn add_material(&mut self, material: Material) {
        self.materials.push(material);
    }

    /// Get a material by ID
    pub fn get_material(&self, id: &str) -> Option<&Material> {
        self.materials.iter().find(|m| m.id == id)
    }

    /// Get a component by ID
    pub fn get_component(&self, id: &str) -> Option<&ModelComponent> {
        self.components.iter().find(|c| c.id == id)
    }

    /// Get all components using a specific material
    pub fn get_components_by_material(&self, material_id: &str) -> Vec<&ModelComponent> {
        self.components
            .iter()
            .filter(|c| c.material_id == material_id)
            .collect()
    }

    /// Set the coordinate reference system
    pub fn with_crs(mut self, crs: impl Into<String>) -> Self {
        self.crs = Some(crs.into());
        self
    }

    /// Set the extent
    pub fn with_extent(mut self, extent: ModelExtent) -> Self {
        self.extent = Some(extent);
        self
    }
}

impl ModelComponent {
    /// Create a new model component
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        component_type: ComponentType,
        material_id: impl Into<String>,
        geometry: Geometry,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            component_type,
            material_id: material_id.into(),
            geometry,
            top: None,
            base: None,
            thickness: None,
            attributes: HashMap::new(),
        }
    }

    /// Set elevations
    pub fn with_elevations(mut self, top: f64, base: f64) -> Self {
        self.top = Some(top);
        self.base = Some(base);
        self.thickness = Some((top - base).abs());
        self
    }

    /// Set thickness
    pub fn with_thickness(mut self, thickness: f64) -> Self {
        self.thickness = Some(thickness);
        self
    }

    /// Add an attribute
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

impl ModelExtent {
    /// Create a 2D extent
    pub fn new_2d(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z: None,
            max_z: None,
        }
    }

    /// Create a 3D extent
    pub fn new_3d(
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        min_z: f64,
        max_z: f64,
    ) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z: Some(min_z),
            max_z: Some(max_z),
        }
    }

    /// Check if a point is within the extent
    pub fn contains(&self, x: f64, y: f64, z: Option<f64>) -> bool {
        let xy_inside = x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y;

        if let (Some(z), Some(min_z), Some(max_z)) = (z, self.min_z, self.max_z) {
            xy_inside && z >= min_z && z <= max_z
        } else {
            xy_inside
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::MaterialType;

    #[test]
    fn test_create_ground_model() {
        let mut model = GroundModel::new(
            "MODEL001",
            "Site Investigation Model",
            ModelType::Stratigraphic,
            ModelDimension::TwoD,
        );

        let material = Material::new("MAT001", "Clay", MaterialType::Soil);
        model.add_material(material);

        assert_eq!(model.id, "MODEL001");
        assert_eq!(model.materials.len(), 1);
    }

    #[test]
    fn test_model_extent() {
        let extent = ModelExtent::new_2d(0.0, 100.0, 0.0, 100.0);
        
        assert!(extent.contains(50.0, 50.0, None));
        assert!(!extent.contains(150.0, 50.0, None));
    }

    #[test]
    fn test_model_extent_3d() {
        let extent = ModelExtent::new_3d(0.0, 100.0, 0.0, 100.0, -10.0, 10.0);
        
        assert!(extent.contains(50.0, 50.0, Some(0.0)));
        assert!(!extent.contains(50.0, 50.0, Some(20.0)));
    }
}
