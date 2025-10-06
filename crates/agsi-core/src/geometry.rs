use geo_types::{Coord, LineString, Point, Polygon};
use serde::{Deserialize, Serialize};
use wkt::ToWkt;

use crate::error::{Error, Result};

/// Geometric representation for AGSi objects
///
/// Supports 1D (points, polylines), 2D (polygons), and 3D (surfaces via OBJ)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Geometry {
    /// Point geometry (1D)
    Point {
        coordinates: [f64; 3], // x, y, z
        #[serde(skip_serializing_if = "Option::is_none")]
        crs: Option<String>,
    },

    /// Polyline geometry (1D/2D)
    LineString {
        coordinates: Vec<[f64; 3]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        crs: Option<String>,
        /// WKT representation (embedded as text)
        #[serde(skip_serializing_if = "Option::is_none")]
        wkt: Option<String>,
        /// WKB representation (embedded as base64)
        #[serde(skip_serializing_if = "Option::is_none")]
        wkb: Option<String>,
    },

    /// Polygon geometry (2D)
    Polygon {
        /// Exterior ring followed by interior rings (holes)
        rings: Vec<Vec<[f64; 3]>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        crs: Option<String>,
        /// WKT representation (embedded as text)
        #[serde(skip_serializing_if = "Option::is_none")]
        wkt: Option<String>,
        /// WKB representation (embedded as base64)
        #[serde(skip_serializing_if = "Option::is_none")]
        wkb: Option<String>,
    },

    /// Surface geometry (3D) - stored as OBJ format
    Surface {
        /// OBJ file content (embedded as base64 binary)
        obj_data: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        crs: Option<String>,
        /// Optional metadata about the surface
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<SurfaceMetadata>,
    },

    /// Multi-geometry collection
    Collection {
        geometries: Vec<Geometry>,
        #[serde(skip_serializing_if = "Option::is_none")]
        crs: Option<String>,
    },
}

/// Metadata for surface geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceMetadata {
    pub vertex_count: usize,
    pub face_count: usize,
    pub bounds: Option<BoundingBox>,
}

/// 3D Bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl Geometry {
    /// Create a point geometry
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::Point {
            coordinates: [x, y, z],
            crs: None,
        }
    }

    /// Create a linestring from coordinates
    pub fn linestring(coords: Vec<[f64; 3]>) -> Result<Self> {
        if coords.len() < 2 {
            return Err(Error::Geometry(
                "LineString must have at least 2 points".to_string(),
            ));
        }

        Ok(Self::LineString {
            coordinates: coords,
            crs: None,
            wkt: None,
            wkb: None,
        })
    }

    /// Create a polygon from rings
    pub fn polygon(exterior: Vec<[f64; 3]>, interiors: Vec<Vec<[f64; 3]>>) -> Result<Self> {
        if exterior.len() < 3 {
            return Err(Error::Geometry(
                "Polygon exterior ring must have at least 3 points".to_string(),
            ));
        }

        let mut rings = vec![exterior];
        rings.extend(interiors);

        Ok(Self::Polygon {
            rings,
            crs: None,
            wkt: None,
            wkb: None,
        })
    }

    /// Create a surface from OBJ data
    pub fn surface(obj_data: Vec<u8>, metadata: Option<SurfaceMetadata>) -> Self {
        use base64::{engine::general_purpose, Engine as _};
        Self::Surface {
            obj_data: general_purpose::STANDARD.encode(&obj_data),
            crs: None,
            metadata,
        }
    }

    /// Convert to WKT representation (for 1D/2D geometries)
    pub fn to_wkt(&self) -> Result<String> {
        match self {
            Self::Point { coordinates, .. } => {
                let point = Point::new(coordinates[0], coordinates[1]);
                Ok(point.to_wkt().to_string())
            }
            Self::LineString { coordinates, .. } => {
                let coords: Vec<Coord<f64>> = coordinates
                    .iter()
                    .map(|c| Coord { x: c[0], y: c[1] })
                    .collect();
                let line = LineString::from(coords);
                Ok(line.to_wkt().to_string())
            }
            Self::Polygon { rings, .. } => {
                if rings.is_empty() {
                    return Err(Error::Geometry("Empty polygon".to_string()));
                }

                let exterior: Vec<Coord<f64>> = rings[0]
                    .iter()
                    .map(|c| Coord { x: c[0], y: c[1] })
                    .collect();
                let exterior_ring = LineString::from(exterior);

                let interior_rings: Vec<LineString<f64>> = rings[1..]
                    .iter()
                    .map(|ring| {
                        let coords: Vec<Coord<f64>> =
                            ring.iter().map(|c| Coord { x: c[0], y: c[1] }).collect();
                        LineString::from(coords)
                    })
                    .collect();

                let polygon = Polygon::new(exterior_ring, interior_rings);
                Ok(polygon.to_wkt().to_string())
            }
            Self::Surface { .. } => {
                Err(Error::Geometry("Surface geometry cannot be converted to WKT".to_string()))
            }
            Self::Collection { .. } => {
                Err(Error::Geometry("Collection geometry not yet supported for WKT".to_string()))
            }
        }
    }

    /// Get the coordinate reference system
    pub fn crs(&self) -> Option<&str> {
        match self {
            Self::Point { crs, .. }
            | Self::LineString { crs, .. }
            | Self::Polygon { crs, .. }
            | Self::Surface { crs, .. }
            | Self::Collection { crs, .. } => crs.as_deref(),
        }
    }

    /// Set the coordinate reference system
    pub fn with_crs(mut self, crs: impl Into<String>) -> Self {
        let crs_value = Some(crs.into());
        match &mut self {
            Self::Point { crs, .. }
            | Self::LineString { crs, .. }
            | Self::Polygon { crs, .. }
            | Self::Surface { crs, .. }
            | Self::Collection { crs, .. } => *crs = crs_value,
        }
        self
    }

    /// Compute WKT and WKB representations (for 1D/2D geometries)
    pub fn compute_wkt_wkb(&mut self) -> Result<()> {
        // First compute WKT from self
        let wkt_str = match self {
            Self::LineString { .. } | Self::Polygon { .. } => Some(self.to_wkt()?),
            _ => None,
        };

        // Then update the fields
        match self {
            Self::LineString { wkt, wkb, .. } | Self::Polygon { wkt, wkb, .. } => {
                *wkt = wkt_str;
                // WKB computation would go here using geozero
                // For now, we'll leave it as None or implement later
                *wkb = None;
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_geometry() {
        let geom = Geometry::point(100.0, 200.0, 10.0);
        
        match geom {
            Geometry::Point { coordinates, .. } => {
                assert_eq!(coordinates, [100.0, 200.0, 10.0]);
            }
            _ => panic!("Expected Point geometry"),
        }
    }

    #[test]
    fn test_linestring_to_wkt() {
        let geom = Geometry::linestring(vec![
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [2.0, 0.0, 0.0],
        ])
        .unwrap();

        let wkt = geom.to_wkt().unwrap();
        assert!(wkt.contains("LINESTRING"));
    }

    #[test]
    fn test_polygon_to_wkt() {
        let exterior = vec![
            [0.0, 0.0, 0.0],
            [10.0, 0.0, 0.0],
            [10.0, 10.0, 0.0],
            [0.0, 10.0, 0.0],
            [0.0, 0.0, 0.0],
        ];

        let geom = Geometry::polygon(exterior, vec![]).unwrap();
        let wkt = geom.to_wkt().unwrap();
        assert!(wkt.contains("POLYGON"));
    }

    #[test]
    fn test_crs() {
        let geom = Geometry::point(0.0, 0.0, 0.0).with_crs("EPSG:27700");
        assert_eq!(geom.crs(), Some("EPSG:27700"));
    }
}
