use crate::{Document, Error, Result};
use validator::Validate;

/// Validation result containing errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

/// A validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub path: String,
    pub message: String,
    pub error_type: ValidationErrorType,
}

/// A validation warning
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub path: String,
    pub message: String,
}

/// Type of validation error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorType {
    Schema,
    Required,
    Type,
    Range,
    Format,
    Reference,
}

/// Validate a document against AGSi rules
pub fn validate_document(doc: &Document) -> Result<ValidationResult> {
    let mut result = ValidationResult {
        is_valid: true,
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    // Validate structure using validator crate
    if let Err(e) = doc.validate() {
        for (field, errors) in e.field_errors() {
            for error in errors {
                result.errors.push(ValidationError {
                    path: field.to_string(),
                    message: error.to_string(),
                    error_type: ValidationErrorType::Schema,
                });
            }
        }
        result.is_valid = false;
    }

    // Validate schema version
    if doc.ags_schema.version != crate::AGSI_VERSION {
        result.warnings.push(ValidationWarning {
            path: "agsSchema.version".to_string(),
            message: format!(
                "Schema version {} differs from library version {}",
                doc.ags_schema.version,
                crate::AGSI_VERSION
            ),
        });
    }

    // Validate model references
    for (model_idx, model) in doc.agsi_model.iter().enumerate() {
        // Check for duplicate model IDs
        let duplicate_count = doc.agsi_model.iter().filter(|m| m.id == model.id).count();
        if duplicate_count > 1 {
            result.errors.push(ValidationError {
                path: format!("agsiModel[{}].id", model_idx),
                message: format!("Duplicate model ID: {}", model.id),
                error_type: ValidationErrorType::Reference,
            });
            result.is_valid = false;
        }

        // Check material references in components
        for (comp_idx, component) in model.components.iter().enumerate() {
            if model.get_material(&component.material_id).is_none() {
                result.errors.push(ValidationError {
                    path: format!("agsiModel[{}].components[{}].materialId", model_idx, comp_idx),
                    message: format!(
                        "Material ID '{}' not found in model",
                        component.material_id
                    ),
                    error_type: ValidationErrorType::Reference,
                });
                result.is_valid = false;
            }
        }

        // Check for duplicate material IDs within a model
        for (mat_idx, material) in model.materials.iter().enumerate() {
            let duplicate_count = model
                .materials
                .iter()
                .filter(|m| m.id == material.id)
                .count();
            if duplicate_count > 1 {
                result.errors.push(ValidationError {
                    path: format!("agsiModel[{}].materials[{}].id", model_idx, mat_idx),
                    message: format!("Duplicate material ID: {}", material.id),
                    error_type: ValidationErrorType::Reference,
                });
                result.is_valid = false;
            }
        }

        // Validate model extent if present
        if let Some(ref extent) = model.extent {
            if extent.min_x > extent.max_x {
                result.errors.push(ValidationError {
                    path: format!("agsiModel[{}].extent", model_idx),
                    message: "min_x must be less than or equal to max_x".to_string(),
                    error_type: ValidationErrorType::Range,
                });
                result.is_valid = false;
            }
            if extent.min_y > extent.max_y {
                result.errors.push(ValidationError {
                    path: format!("agsiModel[{}].extent", model_idx),
                    message: "min_y must be less than or equal to max_y".to_string(),
                    error_type: ValidationErrorType::Range,
                });
                result.is_valid = false;
            }
            if let (Some(min_z), Some(max_z)) = (extent.min_z, extent.max_z) {
                if min_z > max_z {
                    result.errors.push(ValidationError {
                        path: format!("agsiModel[{}].extent", model_idx),
                        message: "min_z must be less than or equal to max_z".to_string(),
                        error_type: ValidationErrorType::Range,
                    });
                    result.is_valid = false;
                }
            }
        }
    }

    Ok(result)
}

/// Quick validation - returns error if invalid
pub fn validate_quick(doc: &Document) -> Result<()> {
    let result = validate_document(doc)?;
    if !result.is_valid {
        return Err(Error::Validation(format!(
            "Validation failed with {} errors",
            result.errors.len()
        )));
    }
    Ok(())
}

impl ValidationResult {
    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Get all errors
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Get all warnings
    pub fn warnings(&self) -> &[ValidationWarning] {
        &self.warnings
    }

    /// Format as human-readable string
    pub fn to_string(&self) -> String {
        let mut output = String::new();

        if self.is_valid {
            output.push_str("✓ Validation passed\n");
        } else {
            output.push_str("✗ Validation failed\n");
        }

        if !self.errors.is_empty() {
            output.push_str(&format!("\n{} Errors:\n", self.errors.len()));
            for error in &self.errors {
                output.push_str(&format!("  • {} - {}\n", error.path, error.message));
            }
        }

        if !self.warnings.is_empty() {
            output.push_str(&format!("\n{} Warnings:\n", self.warnings.len()));
            for warning in &self.warnings {
                output.push_str(&format!("  • {} - {}\n", warning.path, warning.message));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{Material, MaterialType};
    use crate::model::{ComponentType, GroundModel, ModelComponent, ModelDimension, ModelType};
    use crate::geometry::Geometry;

    #[test]
    fn test_valid_document() {
        let doc = Document::new("TEST001");
        let result = validate_document(&doc).unwrap();
        assert!(result.is_valid());
    }

    #[test]
    fn test_missing_material_reference() {
        let mut doc = Document::new("TEST001");
        let mut model = GroundModel::new(
            "MODEL001",
            "Test",
            ModelType::Stratigraphic,
            ModelDimension::TwoD,
        );

        // Add component without adding the material
        let component = ModelComponent::new(
            "COMP001",
            "Layer 1",
            ComponentType::Layer,
            "MAT_MISSING",
            Geometry::point(0.0, 0.0, 0.0),
        );
        model.add_component(component);
        doc.add_model(model);

        let result = validate_document(&doc).unwrap();
        assert!(!result.is_valid());
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_duplicate_material_id() {
        let mut doc = Document::new("TEST001");
        let mut model = GroundModel::new(
            "MODEL001",
            "Test",
            ModelType::Stratigraphic,
            ModelDimension::TwoD,
        );

        // Add two materials with the same ID
        model.add_material(Material::new("MAT001", "Clay", MaterialType::Soil));
        model.add_material(Material::new("MAT001", "Sand", MaterialType::Soil));

        doc.add_model(model);

        let result = validate_document(&doc).unwrap();
        assert!(!result.is_valid());
    }
}
