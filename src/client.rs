use crate::error::{map_reqwest_error, CliError, Result};
use crate::{debug, debug_json};
use reqwest::{Method, RequestBuilder};
use serde_json::Value;

#[derive(Clone)]
pub struct CraftClient {
    base_url: String,
    secret_key: Option<String>,
    http: reqwest::Client,
}

impl CraftClient {
    pub fn new(base_url: String, secret_key: Option<String>) -> Self {
        Self {
            base_url,
            secret_key,
            http: reqwest::Client::new(),
        }
    }

    fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        debug!("Request: {} {}", method, url);

        let mut req = self.http.request(method, &url);

        // Add Authorization header if secret key exists
        if let Some(key) = &self.secret_key {
            debug!("Using Authorization header");
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        req
    }

    // ==================== Blocks ====================

    pub async fn get_block(
        &self,
        id: Option<&str>,
        date: Option<&str>,
        depth: i32,
        metadata: bool,
    ) -> Result<Value> {
        let mut query: Vec<(&str, String)> = vec![
            ("maxDepth", depth.to_string()),
            ("fetchMetadata", metadata.to_string()),
        ];

        if let Some(id) = id {
            query.push(("id", id.to_string()));
        }

        if let Some(date) = date {
            query.push(("date", date.to_string()));
        }

        let req = self.build_request(Method::GET, "/blocks").query(&query);
        debug!("Query params: {:?}", query);

        let resp = req.send().await.map_err(map_reqwest_error)?;
        let status = resp.status();
        debug!("Response status: {}", status);

        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            debug!("Error response: {}", text);
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        let json: Value = resp.json().await.map_err(map_reqwest_error)?;
        debug_json!("Response body", &json);
        Ok(json)
    }

    pub async fn insert_blocks(
        &self,
        blocks: Vec<Value>,
        position: Value,
    ) -> Result<Value> {
        let body = json!({
            "blocks": blocks,
            "position": position
        });

        let resp = self
            .build_request(Method::POST, "/blocks")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn update_blocks(&self, blocks: Vec<Value>) -> Result<Value> {
        let body = json!({ "blocks": blocks });

        let resp = self
            .build_request(Method::PUT, "/blocks")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn delete_blocks(&self, block_ids: Vec<String>) -> Result<Value> {
        let body = json!({ "blockIds": block_ids });

        let resp = self
            .build_request(Method::DELETE, "/blocks")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn move_blocks(&self, block_ids: Vec<String>, position: Value) -> Result<Value> {
        let body = json!({
            "blockIds": block_ids,
            "position": position
        });

        let resp = self
            .build_request(Method::PUT, "/blocks/move")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn search_blocks(
        &self,
        date: Option<&str>,
        pattern: &str,
        case_sensitive: bool,
        before: Option<i32>,
        after: Option<i32>,
    ) -> Result<Value> {
        let mut query: Vec<(&str, String)> = vec![
            ("pattern", pattern.to_string()),
            ("caseSensitive", case_sensitive.to_string()),
        ];

        if let Some(date) = date {
            query.push(("date", date.to_string()));
        }

        if let Some(before) = before {
            query.push(("beforeBlockCount", before.to_string()));
        }

        if let Some(after) = after {
            query.push(("afterBlockCount", after.to_string()));
        }

        let resp = self
            .build_request(Method::GET, "/blocks/search")
            .query(&query)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    // ==================== Tasks ====================

    pub async fn list_tasks(&self, scope: &str) -> Result<Value> {
        let resp = self
            .build_request(Method::GET, "/tasks")
            .query(&[("scope", scope)])
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn add_tasks(&self, tasks: Vec<Value>) -> Result<Value> {
        let body = json!({ "tasks": tasks });

        let resp = self
            .build_request(Method::POST, "/tasks")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn update_tasks(&self, tasks: Vec<Value>) -> Result<Value> {
        let body = json!({ "tasksToUpdate": tasks });

        let resp = self
            .build_request(Method::PUT, "/tasks")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn delete_tasks(&self, ids: Vec<String>) -> Result<Value> {
        let body = json!({ "idsToDelete": ids });

        let resp = self
            .build_request(Method::DELETE, "/tasks")
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    // ==================== Search ====================

    pub async fn search(
        &self,
        query: &str,
        from: Option<&str>,
        to: Option<&str>,
        use_regex: bool,
    ) -> Result<Value> {
        let mut query_params: Vec<(&str, String)> = vec![("include", query.to_string())];

        if let Some(from) = from {
            query_params.push(("startDate", from.to_string()));
        }

        if let Some(to) = to {
            query_params.push(("endDate", to.to_string()));
        }

        if use_regex {
            query_params.push(("regexps", query.to_string()));
        }

        let resp = self
            .build_request(Method::GET, "/daily-notes/search")
            .query(&query_params)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    // ==================== Collections ====================

    pub async fn list_collections(&self, from: Option<&str>, to: Option<&str>) -> Result<Value> {
        let mut query: Vec<(&str, String)> = Vec::new();

        if let Some(from) = from {
            query.push(("startDate", from.to_string()));
        }

        if let Some(to) = to {
            query.push(("endDate", to.to_string()));
        }

        let resp = self
            .build_request(Method::GET, "/collections")
            .query(&query)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn get_collection_schema(&self, id: &str, format: &str) -> Result<Value> {
        let resp = self
            .build_request(Method::GET, &format!("/collections/{}/schema", id))
            .query(&[("format", format)])
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn get_collection_items(&self, id: &str, depth: i32) -> Result<Value> {
        let resp = self
            .build_request(Method::GET, &format!("/collections/{}/items", id))
            .query(&[("maxDepth", depth.to_string())])
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn add_collection_items(&self, id: &str, items: Vec<Value>) -> Result<Value> {
        let body = json!({ "items": items });

        let resp = self
            .build_request(Method::POST, &format!("/collections/{}/items", id))
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn update_collection_items(&self, id: &str, items: Vec<Value>) -> Result<Value> {
        let body = json!({ "itemsToUpdate": items });

        let resp = self
            .build_request(Method::PUT, &format!("/collections/{}/items", id))
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    pub async fn delete_collection_items(&self, id: &str, item_ids: Vec<String>) -> Result<Value> {
        let body = json!({ "idsToDelete": item_ids });

        let resp = self
            .build_request(Method::DELETE, &format!("/collections/{}/items", id))
            .json(&body)
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    // ==================== Connection ====================

    pub async fn get_connection_info(&self) -> Result<Value> {
        let resp = self
            .build_request(Method::GET, "/connection")
            .send()
            .await
            .map_err(map_reqwest_error)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        resp.json().await.map_err(map_reqwest_error)
    }

    // ==================== Upload ====================

    pub async fn upload_file(
        &self,
        file_path: &std::path::Path,
        page_id: &str,
        position: &str,
    ) -> Result<Value> {
        use reqwest::multipart::{Form, Part};

        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file");

        let file_content = tokio::fs::read(file_path).await.map_err(|e| {
            CliError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read file: {}", e),
            ))
        })?;

        let part = Part::bytes(file_content).file_name(file_name.to_string());

        let form = Form::new().part("file", part);

        let url = format!(
            "{}/upload?position={}&pageId={}",
            self.base_url.trim_end_matches('/'),
            position,
            page_id
        );

        debug!("Uploading file: {} to {}", file_name, url);

        let mut req = self.http.post(&url).multipart(form);

        if let Some(key) = &self.secret_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let resp = req.send().await.map_err(map_reqwest_error)?;
        let status = resp.status();
        debug!("Upload response status: {}", status);

        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            debug!("Upload error: {}", text);
            return Err(CliError::Api {
                code: status.as_u16(),
                message: text,
            });
        }

        let json: Value = resp.json().await.map_err(map_reqwest_error)?;
        debug_json!("Upload response", &json);
        Ok(json)
    }
}

// Need to import json! macro
use serde_json::json;
