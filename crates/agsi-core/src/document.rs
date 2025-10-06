use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use crate::model::GroundModel;
use crate::project::Project;
use crate::AGSI_VERSION;

/// Top-level AGSi document structure
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    /// AGSi schema information
    pub ags_schema: SchemaInfo,

    /// File metadata
    pub ags_file: FileInfo,

    /// Optional project information
    pub ags_project: Option<Project>,

    /// Ground models in this document
    #[serde(default)]
    pub agsi_model: Vec<GroundModel>,

    /// Additional custom fields
    #[serde(flatten)]
    pub extensions: HashMap<String, serde_json::Value>,
}

/// AGSi schema version information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SchemaInfo {
    /// Schema version (e.g., "1.0.1")
    #[validate(length(min = 1))]
    pub version: String,

    /// Optional schema variant
    pub variant: Option<String>,
}

/// File metadata information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    /// File identifier
    pub file_id: String,

    /// File name
    pub file_name: Option<String>,

    /// File creation date (ISO 8601)
    pub file_date: Option<String>,

    /// Creator/author
    pub file_author: Option<String>,

    /// Software used to create the file
    pub file_software: Option<String>,

    /// File version
    pub file_version: Option<String>,

    /// File comments
    pub file_comments: Option<String>,
}

impl Document {
    /// Create a new AGSi document
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            ags_schema: SchemaInfo {
                version: AGSI_VERSION.to_string(),
                variant: None,
            },
            ags_file: FileInfo {
                file_id: file_id.into(),
                file_name: None,
                file_date: None,
                file_author: None,
                file_software: Some("agsi-rust".to_string()),
                file_version: Some(env!("CARGO_PKG_VERSION").to_string()),
                file_comments: None,
            },
            ags_project: None,
            agsi_model: Vec::new(),
            extensions: HashMap::new(),
        }
    }

    /// Add a project to the document
    pub fn with_project(mut self, project: Project) -> Self {
        self.ags_project = Some(project);
        self
    }

    /// Add a ground model to the document
    pub fn add_model(&mut self, model: GroundModel) {
        self.agsi_model.push(model);
    }

    /// Get a model by ID
    pub fn get_model(&self, id: &str) -> Option<&GroundModel> {
        self.agsi_model.iter().find(|m| m.id == id)
    }

    /// Get a mutable reference to a model by ID
    pub fn get_model_mut(&mut self, id: &str) -> Option<&mut GroundModel> {
        self.agsi_model.iter_mut().find(|m| m.id == id)
    }

    /// Load a document from JSON file
    pub fn from_json_file(path: impl AsRef<std::path::Path>) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_json_str(&content)
    }

    /// Load a document from JSON string
    pub fn from_json_str(json: &str) -> crate::Result<Self> {
        let doc: Document = serde_json::from_str(json)?;
        doc.validate()?;
        Ok(doc)
    }

    /// Save document to JSON file
    pub fn to_json_file(&self, path: impl AsRef<std::path::Path>) -> crate::Result<()> {
        let json = self.to_json_string()?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Convert document to JSON string
    pub fn to_json_string(&self) -> crate::Result<String> {
        self.validate()?;
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    /// Set file metadata
    pub fn with_file_name(mut self, name: impl Into<String>) -> Self {
        self.ags_file.file_name = Some(name.into());
        self
    }

    /// Set file author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.ags_file.file_author = Some(author.into());
        self
    }

    /// Set file comments
    pub fn with_comments(mut self, comments: impl Into<String>) -> Self {
        self.ags_file.file_comments = Some(comments.into());
        self
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{Material, MaterialType};
    use crate::model::{ModelDimension, ModelType};

    #[test]
    fn test_create_document() {
        let doc = Document::new("DOC001")
            .with_file_name("test.agsi")
            .with_author("Test Author");

        assert_eq!(doc.ags_file.file_id, "DOC001");
        assert_eq!(doc.ags_file.file_author, Some("Test Author".to_string()));
        assert_eq!(doc.ags_schema.version, AGSI_VERSION);
    }

    #[test]
    fn test_document_with_model() {
        let mut doc = Document::new("DOC001");

        let mut model = GroundModel::new(
            "MODEL001",
            "Test Model",
            ModelType::Stratigraphic,
            ModelDimension::TwoD,
        );

        let material = Material::new("MAT001", "Clay", MaterialType::Soil);
        model.add_material(material);

        doc.add_model(model);

        assert_eq!(doc.agsi_model.len(), 1);
        assert!(doc.get_model("MODEL001").is_some());
    }

    #[test]
    fn test_serialize_deserialize() {
        let doc = Document::new("DOC001");
        let json = doc.to_json_string().unwrap();
        let deserialized = Document::from_json_str(&json).unwrap();

        assert_eq!(doc.ags_file.file_id, deserialized.ags_file.file_id);
    }
}
