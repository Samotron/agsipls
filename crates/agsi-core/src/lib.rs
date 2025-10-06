//! AGSi Core Library
//!
//! This library provides data structures and utilities for working with AGSi
//! (Association of Geotechnical & Geoenvironmental Specialists interchange format)
//! ground model data.
//!
//! # Features
//!
//! - Type-safe data structures for AGSi components
//! - Multiple serialization formats (JSON, Avro, Protocol Buffers)
//! - Geometry handling with WKT/WKB for 1D/2D, OBJ for 3D surfaces
//! - Schema validation
//! - Material-centric design allowing independent use of components

pub mod document;
pub mod error;
pub mod geometry;
pub mod material;
pub mod model;
pub mod project;
pub mod serialization;
pub mod validation;

pub use document::Document;
pub use error::{Error, Result};
pub use material::{Material, MaterialProperty};
pub use model::{GroundModel, ModelComponent};

/// AGSi schema version
pub const AGSI_VERSION: &str = "1.0.1";
