use crate::{Document, Error, Result};

/// Serialization formats supported by AGSi
pub enum Format {
    Json,
    JsonCompact,
    Avro,
    Protobuf,
}

/// Serialize a document to bytes in the specified format
pub fn serialize(doc: &Document, format: Format) -> Result<Vec<u8>> {
    match format {
        Format::Json => serialize_json(doc, true),
        Format::JsonCompact => serialize_json(doc, false),
        Format::Avro => serialize_avro(doc),
        Format::Protobuf => serialize_protobuf(doc),
    }
}

/// Deserialize a document from bytes
pub fn deserialize(data: &[u8], format: Format) -> Result<Document> {
    match format {
        Format::Json | Format::JsonCompact => deserialize_json(data),
        Format::Avro => deserialize_avro(data),
        Format::Protobuf => deserialize_protobuf(data),
    }
}

/// Serialize to JSON
fn serialize_json(doc: &Document, pretty: bool) -> Result<Vec<u8>> {
    let json = if pretty {
        serde_json::to_vec_pretty(doc)?
    } else {
        serde_json::to_vec(doc)?
    };
    Ok(json)
}

/// Deserialize from JSON
fn deserialize_json(data: &[u8]) -> Result<Document> {
    let doc: Document = serde_json::from_slice(data)?;
    Ok(doc)
}

/// Serialize to Avro format
fn serialize_avro(doc: &Document) -> Result<Vec<u8>> {
    use apache_avro::{Schema, Writer};
    
    // Load schema
    let schema_str = include_str!("../../../schemas/agsi.avsc");
    let schema = Schema::parse_str(schema_str)
        .map_err(|e| Error::Serialization(format!("Failed to parse Avro schema: {}", e)))?;
    
    // Convert to Avro value
    let json_value = serde_json::to_value(doc)?;
    let avro_value = apache_avro::to_value(json_value)
        .map_err(|e| Error::Serialization(format!("Failed to convert to Avro: {}", e)))?;
    
    // Write to bytes
    let mut writer = Writer::new(&schema, Vec::new());
    writer.append(avro_value)
        .map_err(|e| Error::Serialization(format!("Failed to write Avro: {}", e)))?;
    
    writer.into_inner()
        .map_err(|e| Error::Serialization(format!("Failed to finalize Avro: {}", e)))
}

/// Deserialize from Avro format
fn deserialize_avro(data: &[u8]) -> Result<Document> {
    use apache_avro::Reader;
    
    // Read from bytes
    let reader = Reader::new(data)
        .map_err(|e| Error::Deserialization(format!("Failed to read Avro: {}", e)))?;
    
    // Get first record
    for value in reader {
        let value = value
            .map_err(|e| Error::Deserialization(format!("Failed to deserialize Avro: {}", e)))?;
        
        // Convert to JSON then to Document
        let json_value = apache_avro::from_value::<serde_json::Value>(&value)
            .map_err(|e| Error::Deserialization(format!("Failed to convert from Avro: {}", e)))?;
        
        let doc: Document = serde_json::from_value(json_value)?;
        return Ok(doc);
    }
    
    Err(Error::Deserialization("No records found in Avro data".to_string()))
}

/// Serialize to Protocol Buffers format
fn serialize_protobuf(_doc: &Document) -> Result<Vec<u8>> {
    // TODO: Implement protobuf serialization with prost
    // This requires generated code from the .proto file
    Err(Error::Serialization(
        "Protobuf serialization requires code generation from .proto files. Run: protoc --rust_out=. schemas/agsi.proto".to_string(),
    ))
}

/// Deserialize from Protocol Buffers format
fn deserialize_protobuf(_data: &[u8]) -> Result<Document> {
    Err(Error::Deserialization(
        "Protobuf deserialization requires generated code from .proto files".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_serialization() {
        let doc = Document::new("TEST001");
        let bytes = serialize(&doc, Format::Json).unwrap();
        let deserialized = deserialize(&bytes, Format::Json).unwrap();

        assert_eq!(doc.ags_file.file_id, deserialized.ags_file.file_id);
    }

    #[test]
    fn test_json_compact_serialization() {
        let doc = Document::new("TEST001");
        let pretty = serialize(&doc, Format::Json).unwrap();
        let compact = serialize(&doc, Format::JsonCompact).unwrap();

        // Compact should be smaller
        assert!(compact.len() < pretty.len());
    }
    
    #[test]
    #[ignore] // Requires proper Avro schema mapping
    fn test_avro_roundtrip() {
        let doc = Document::new("TEST001");
        let bytes = serialize(&doc, Format::Avro).unwrap();
        let deserialized = deserialize(&bytes, Format::Avro).unwrap();
        
        assert_eq!(doc.ags_file.file_id, deserialized.ags_file.file_id);
        assert_eq!(doc.ags_schema.version, deserialized.ags_schema.version);
    }
}

