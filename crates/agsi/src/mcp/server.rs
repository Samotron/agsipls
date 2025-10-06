use agsi_core::Document;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// MCP Server for AGSi ground model operations
///
/// Implements the Model Context Protocol to enable LLM interaction with AGSi data

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
enum McpRequest {
    #[serde(rename = "tools/list")]
    ListTools {},
    
    #[serde(rename = "tools/call")]
    CallTool {
        name: String,
        arguments: HashMap<String, serde_json::Value>,
    },
    
    #[serde(rename = "resources/list")]
    ListResources {},
    
    #[serde(rename = "resources/read")]
    ReadResource {
        uri: String,
    },
}

#[derive(Debug, Serialize)]
struct McpResponse {
    result: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct Tool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

struct AgsiMcpServer {
    // Cache loaded documents
    documents: HashMap<String, Document>,
}

impl AgsiMcpServer {
    fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "agsi_validate".to_string(),
                description: "Validate an AGSi file".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the AGSi file"
                        }
                    },
                    "required": ["file_path"]
                }),
            },
            Tool {
                name: "agsi_extract_materials".to_string(),
                description: "Extract materials from an AGSi ground model".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the AGSi file"
                        },
                        "model_id": {
                            "type": "string",
                            "description": "Model ID to extract from (optional)"
                        }
                    },
                    "required": ["file_path"]
                }),
            },
            Tool {
                name: "agsi_get_info".to_string(),
                description: "Get information about an AGSi document".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the AGSi file"
                        }
                    },
                    "required": ["file_path"]
                }),
            },
            Tool {
                name: "agsi_query_materials".to_string(),
                description: "Query materials by type or properties".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the AGSi file"
                        },
                        "material_type": {
                            "type": "string",
                            "description": "Filter by material type (SOIL, ROCK, etc.)"
                        },
                        "property_name": {
                            "type": "string",
                            "description": "Filter by property name"
                        }
                    },
                    "required": ["file_path"]
                }),
            },
        ]
    }

    async fn handle_tool_call(
        &mut self,
        name: &str,
        arguments: HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        match name {
            "agsi_validate" => self.validate_file(arguments).await,
            "agsi_extract_materials" => self.extract_materials(arguments).await,
            "agsi_get_info" => self.get_info(arguments).await,
            "agsi_query_materials" => self.query_materials(arguments).await,
            _ => Ok(json!({
                "error": format!("Unknown tool: {}", name)
            })),
        }
    }

    async fn validate_file(&mut self, args: HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;

        let doc = Document::from_json_file(file_path)?;
        let result = agsi_core::validation::validate_document(&doc)?;

        Ok(json!({
            "valid": result.is_valid(),
            "errors": result.errors().iter().map(|e| {
                json!({
                    "path": e.path,
                    "message": e.message,
                    "type": format!("{:?}", e.error_type)
                })
            }).collect::<Vec<_>>(),
            "warnings": result.warnings().iter().map(|w| {
                json!({
                    "path": w.path,
                    "message": w.message
                })
            }).collect::<Vec<_>>()
        }))
    }

    async fn extract_materials(&mut self, args: HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;

        let doc = Document::from_json_file(file_path)?;
        let model_id = args.get("model_id").and_then(|v| v.as_str());

        let model = if let Some(id) = model_id {
            doc.get_model(id)
                .ok_or_else(|| anyhow::anyhow!("Model not found: {}", id))?
        } else if doc.agsi_model.len() == 1 {
            &doc.agsi_model[0]
        } else {
            return Ok(json!({
                "error": "Multiple models found. Please specify model_id",
                "available_models": doc.agsi_model.iter().map(|m| &m.id).collect::<Vec<_>>()
            }));
        };

        Ok(json!({
            "model_id": model.id,
            "model_name": model.name,
            "materials": model.materials.iter().map(|m| {
                json!({
                    "id": m.id,
                    "name": m.name,
                    "type": format!("{:?}", m.material_type),
                    "description": m.description,
                    "properties": m.properties.iter().map(|p| {
                        json!({
                            "name": p.name,
                            "value": p.value,
                            "unit": p.unit,
                            "source": p.source
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>()
        }))
    }

    async fn get_info(&mut self, args: HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;

        let doc = Document::from_json_file(file_path)?;

        Ok(json!({
            "file_id": doc.ags_file.file_id,
            "file_name": doc.ags_file.file_name,
            "author": doc.ags_file.file_author,
            "schema_version": doc.ags_schema.version,
            "project": doc.ags_project.as_ref().map(|p| {
                json!({
                    "id": p.id,
                    "name": p.name,
                    "client": p.client
                })
            }),
            "models": doc.agsi_model.iter().map(|m| {
                json!({
                    "id": m.id,
                    "name": m.name,
                    "type": format!("{:?}", m.model_type),
                    "dimension": format!("{:?}", m.dimension),
                    "material_count": m.materials.len(),
                    "component_count": m.components.len()
                })
            }).collect::<Vec<_>>()
        }))
    }

    async fn query_materials(&mut self, args: HashMap<String, serde_json::Value>) -> Result<serde_json::Value> {
        let file_path = args.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing file_path"))?;

        let doc = Document::from_json_file(file_path)?;
        let material_type = args.get("material_type").and_then(|v| v.as_str());
        let property_name = args.get("property_name").and_then(|v| v.as_str());

        let mut results = Vec::new();

        for model in &doc.agsi_model {
            for material in &model.materials {
                let mut matches = true;

                if let Some(mat_type) = material_type {
                    if format!("{:?}", material.material_type).to_uppercase() != mat_type.to_uppercase() {
                        matches = false;
                    }
                }

                if let Some(prop_name) = property_name {
                    if !material.properties.iter().any(|p| p.name == prop_name) {
                        matches = false;
                    }
                }

                if matches {
                    results.push(json!({
                        "model_id": model.id,
                        "material_id": material.id,
                        "name": material.name,
                        "type": format!("{:?}", material.material_type),
                        "properties": material.properties.len()
                    }));
                }
            }
        }

        Ok(json!({
            "matches": results.len(),
            "materials": results
        }))
    }
}

pub async fn run_mcp_server() -> Result<()> {
    eprintln!("AGSi MCP Server starting...");
    eprintln!("Listening on stdin/stdout");

    let mut server = AgsiMcpServer::new();
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();
        let n = reader.read_line(&mut line).await?;
        
        if n == 0 {
            break; // EOF
        }

        let request: Result<serde_json::Value, _> = serde_json::from_str(&line);
        
        let response = match request {
            Ok(req) => {
                eprintln!("Received request: {:?}", req);
                
                if let Some(method) = req.get("method").and_then(|m| m.as_str()) {
                    match method {
                        "tools/list" => {
                            json!({
                                "jsonrpc": "2.0",
                                "id": req.get("id"),
                                "result": {
                                    "tools": server.list_tools()
                                }
                            })
                        }
                        "tools/call" => {
                            let params = req.get("params").cloned().unwrap_or(json!({}));
                            let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            let arguments: HashMap<String, serde_json::Value> = 
                                serde_json::from_value(params.get("arguments").cloned().unwrap_or(json!({})))?;
                            
                            match server.handle_tool_call(name, arguments).await {
                                Ok(result) => {
                                    json!({
                                        "jsonrpc": "2.0",
                                        "id": req.get("id"),
                                        "result": result
                                    })
                                }
                                Err(e) => {
                                    json!({
                                        "jsonrpc": "2.0",
                                        "id": req.get("id"),
                                        "error": {
                                            "code": -32000,
                                            "message": e.to_string()
                                        }
                                    })
                                }
                            }
                        }
                        _ => {
                            json!({
                                "jsonrpc": "2.0",
                                "id": req.get("id"),
                                "error": {
                                    "code": -32601,
                                    "message": format!("Method not found: {}", method)
                                }
                            })
                        }
                    }
                } else {
                    json!({
                        "jsonrpc": "2.0",
                        "id": req.get("id"),
                        "error": {
                            "code": -32600,
                            "message": "Invalid request"
                        }
                    })
                }
            }
            Err(e) => {
                json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    }
                })
            }
        };

        let response_str = serde_json::to_string(&response)?;
        stdout.write_all(response_str.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }

    Ok(())
}
