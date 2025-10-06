use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use agsi_core::{
    Document, Material, GroundModel,
    material::MaterialType,
    model::{ModelType, ModelDimension},
    validation,
};

/// Python wrapper for AGSi Document
#[pyclass]
struct PyDocument {
    inner: Document,
}

#[pymethods]
impl PyDocument {
    /// Create a new document
    #[new]
    fn new(file_id: String) -> Self {
        PyDocument {
            inner: Document::new(file_id),
        }
    }

    /// Load document from JSON file
    #[staticmethod]
    fn from_json_file(path: String) -> PyResult<Self> {
        let doc = Document::from_json_file(&path)
            .map_err(|e| PyValueError::new_err(format!("Failed to load: {}", e)))?;
        Ok(PyDocument { inner: doc })
    }

    /// Load document from JSON string
    #[staticmethod]
    fn from_json_str(json: String) -> PyResult<Self> {
        let doc = Document::from_json_str(&json)
            .map_err(|e| PyValueError::new_err(format!("Failed to parse: {}", e)))?;
        Ok(PyDocument { inner: doc })
    }

    /// Save document to JSON file
    fn to_json_file(&self, path: String) -> PyResult<()> {
        self.inner.to_json_file(&path)
            .map_err(|e| PyValueError::new_err(format!("Failed to save: {}", e)))
    }

    /// Convert document to JSON string
    fn to_json_string(&self) -> PyResult<String> {
        self.inner.to_json_string()
            .map_err(|e| PyValueError::new_err(format!("Failed to serialize: {}", e)))
    }

    /// Get file ID
    #[getter]
    fn file_id(&self) -> String {
        self.inner.ags_file.file_id.clone()
    }

    /// Set author
    fn set_author(&mut self, author: String) {
        self.inner.ags_file.file_author = Some(author);
    }

    /// Get number of models
    fn model_count(&self) -> usize {
        self.inner.agsi_model.len()
    }

    /// Validate the document
    fn validate(&self) -> PyResult<PyValidationResult> {
        let result = validation::validate_document(&self.inner)
            .map_err(|e| PyValueError::new_err(format!("Validation error: {}", e)))?;
        Ok(PyValidationResult { inner: result })
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Document(file_id='{}', models={})", 
            self.inner.ags_file.file_id, 
            self.inner.agsi_model.len())
    }
}

/// Python wrapper for validation result
#[pyclass]
struct PyValidationResult {
    inner: validation::ValidationResult,
}

#[pymethods]
impl PyValidationResult {
    /// Check if document is valid
    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    /// Get error count
    fn error_count(&self) -> usize {
        self.inner.errors().len()
    }

    /// Get warning count
    fn warning_count(&self) -> usize {
        self.inner.warnings().len()
    }

    /// Get errors as list of strings
    fn errors(&self) -> Vec<String> {
        self.inner.errors()
            .iter()
            .map(|e| format!("{}: {}", e.path, e.message))
            .collect()
    }

    /// Get warnings as list of strings
    fn warnings(&self) -> Vec<String> {
        self.inner.warnings()
            .iter()
            .map(|w| format!("{}: {}", w.path, w.message))
            .collect()
    }

    /// String representation
    fn __repr__(&self) -> String {
        if self.inner.is_valid() {
            format!("ValidationResult(valid=True, warnings={})", self.inner.warnings().len())
        } else {
            format!("ValidationResult(valid=False, errors={}, warnings={})", 
                self.inner.errors().len(), 
                self.inner.warnings().len())
        }
    }
}

/// Python wrapper for Material
#[pyclass]
struct PyMaterial {
    inner: Material,
}

#[pymethods]
impl PyMaterial {
    /// Create a new material
    #[new]
    fn new(id: String, name: String, material_type: String) -> PyResult<Self> {
        let mat_type = match material_type.to_uppercase().as_str() {
            "SOIL" => MaterialType::Soil,
            "ROCK" => MaterialType::Rock,
            "FILL" => MaterialType::Fill,
            "MADE_GROUND" => MaterialType::MadeGround,
            "ANTHROPOGENIC" => MaterialType::Anthropogenic,
            "WATER" => MaterialType::Water,
            "VOID" => MaterialType::Void,
            _ => MaterialType::Unknown,
        };

        Ok(PyMaterial {
            inner: Material::new(id, name, mat_type),
        })
    }

    /// Get material ID
    #[getter]
    fn id(&self) -> String {
        self.inner.id.clone()
    }

    /// Get material name
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    /// Set description
    fn set_description(&mut self, description: String) {
        self.inner.description = Some(description);
    }

    /// Get property count
    fn property_count(&self) -> usize {
        self.inner.properties.len()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Material(id='{}', name='{}', type='{:?}')", 
            self.inner.id, 
            self.inner.name, 
            self.inner.material_type)
    }
}

/// Python module
#[pymodule]
fn agsi_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDocument>()?;
    m.add_class::<PyValidationResult>()?;
    m.add_class::<PyMaterial>()?;
    
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__doc__", "Python bindings for AGSi ground model library")?;
    
    Ok(())
}
