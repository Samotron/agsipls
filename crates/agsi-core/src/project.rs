use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

/// Project information for AGSi documents
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    /// Project identifier
    #[validate(length(min = 1))]
    pub id: String,

    /// Project name
    #[validate(length(min = 1))]
    pub name: String,

    /// Project description
    pub description: Option<String>,

    /// Client name
    pub client: Option<String>,

    /// Contractor name
    pub contractor: Option<String>,

    /// Project location
    pub location: Option<Location>,

    /// Project dates
    pub dates: Option<ProjectDates>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Geographic location information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Location name/address
    pub name: String,

    /// Country
    pub country: Option<String>,

    /// Optional coordinates (longitude, latitude)
    pub coordinates: Option<[f64; 2]>,

    /// Coordinate reference system
    pub crs: Option<String>,
}

/// Project date information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDates {
    /// Project start date (ISO 8601)
    pub start: Option<String>,

    /// Project end date (ISO 8601)
    pub end: Option<String>,

    /// Date of data collection
    pub data_collection: Option<String>,
}

impl Project {
    /// Create a new project
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            client: None,
            contractor: None,
            location: None,
            dates: None,
            metadata: HashMap::new(),
        }
    }

    /// Set the client
    pub fn with_client(mut self, client: impl Into<String>) -> Self {
        self.client = Some(client.into());
        self
    }

    /// Set the contractor
    pub fn with_contractor(mut self, contractor: impl Into<String>) -> Self {
        self.contractor = Some(contractor.into());
        self
    }

    /// Set the location
    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// Set the dates
    pub fn with_dates(mut self, dates: ProjectDates) -> Self {
        self.dates = Some(dates);
        self
    }
}

impl Location {
    /// Create a new location
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            country: None,
            coordinates: None,
            crs: None,
        }
    }

    /// Set coordinates
    pub fn with_coordinates(mut self, lon: f64, lat: f64, crs: Option<String>) -> Self {
        self.coordinates = Some([lon, lat]);
        self.crs = crs;
        self
    }
}
