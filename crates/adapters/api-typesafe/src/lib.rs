//! @system api-typesafe
//! @status generated
//! @edit change the rest_client/api-typesafe row, then regenerate
//!
//! Generated Rust adapter for TypeSafe REST API.
//! Uses reqwest for HTTP, serde for serialization, anyhow for errors.

use anyhow::{Context, Result};
use serde_json::Value;

/// Typed HTTP client for TypeSafe.
#[derive(Clone)]
pub struct Client {
	http: reqwest::Client,
	base_url: String,
	auth: Option<(reqwest::header::HeaderName, reqwest::header::HeaderValue)>,
}

impl Client {
	/// Create a new client with a bearer token.
	pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
		let http = reqwest::Client::builder()
			.timeout(std::time::Duration::from_secs(30))
			.build()?;
		let auth = (
			reqwest::header::AUTHORIZATION,
			reqwest::header::HeaderValue::from_str(&format!("Bearer {api_key}"))?,
		);
		Ok(Self {
			http,
			base_url: base_url.trim_end_matches('/').to_string(),
			auth: Some(auth),
		})
	}

	/// Path for `post /v1/systemone`, exactly as the supplier spec declares it.
	/// Exported so a PASS-THROUGH caller can address the endpoint without
	/// hard-coding it and without going through the typed method.
	pub const SYSTEMONE_V1_SYSTEMONE_POST_PATH: &'static str = "/v1/systemone";

	/// The prepared request for `post /v1/systemone` — the ONE place its URL,
	/// parameters, credential and body are assembled, so the typed, raw and
	/// bytes surfaces below cannot disagree about what reaches the wire.
	fn systemone_v1_systemone_post_request(&self, body: types::SystemoneV1SystemonePostPayload) -> Result<reqwest::RequestBuilder> {
		let mut url = format!("{}/v1/systemone", self.base_url);
		let mut query: Vec<(String, String)> = Vec::new();
		let mut req = self.http.post(&url);
		if !query.is_empty() {
			req = req.query(&query);
		}
		if let Some((ref name, ref value)) = self.auth {
			req = req.header(name, value);
		}
		req = req.json(&body);
		Ok(req)
	}

	/// post /v1/systemone
	pub async fn systemone_v1_systemone_post(&self, body: types::SystemoneV1SystemonePostPayload) -> Result<Value> {
		let resp = self.systemone_v1_systemone_post_request(body)?.send().await
			.with_context(|| "post /v1/systemone failed")?;
		let status = resp.status();
		let body = resp.text().await.unwrap_or_default();
		if !status.is_success() {
			anyhow::bail!("systemone_v1_systemone_post: HTTP {} - {}", status.as_u16(), body);
		}
		let value: Value = serde_json::from_str(&body)
			.with_context(|| "systemone_v1_systemone_post: response not valid JSON")?;
		Ok(value)
	}

	/// post /v1/systemone, with the HTTP status preserved — so a caller can
	/// read 404 or 401 as an OUTCOME rather than a failure, without matching
	/// on an error message.
	pub async fn systemone_v1_systemone_post_raw(&self, body: types::SystemoneV1SystemonePostPayload) -> Result<(u16, String)> {
		let resp = self.systemone_v1_systemone_post_request(body)?.send().await
			.with_context(|| "post /v1/systemone failed")?;
		let status = resp.status();
		let body = resp.text().await.unwrap_or_default();
		Ok((status.as_u16(), body))
	}

	/// post /v1/systemone, with the response as BYTES and the status preserved —
	/// for a body that is not JSON (NDJSON, an archive) and for the same
	/// 404/401-as-outcome reads the raw form serves.
	pub async fn systemone_v1_systemone_post_bytes(&self, body: types::SystemoneV1SystemonePostPayload) -> Result<(u16, Vec<u8>)> {
		let resp = self.systemone_v1_systemone_post_request(body)?.send().await
			.with_context(|| "post /v1/systemone failed")?;
		let status = resp.status();
		let body = resp.bytes().await.unwrap_or_default();
		Ok((status.as_u16(), body.to_vec()))
	}

	/// Path for `get /v1/models`, exactly as the supplier spec declares it.
	/// Exported so a PASS-THROUGH caller can address the endpoint without
	/// hard-coding it and without going through the typed method.
	pub const MODELS_V1_V1_MODELS_GET_PATH: &'static str = "/v1/models";

	/// The prepared request for `get /v1/models` — the ONE place its URL,
	/// parameters, credential and body are assembled, so the typed, raw and
	/// bytes surfaces below cannot disagree about what reaches the wire.
	fn models_v1_v1_models_get_request(&self) -> Result<reqwest::RequestBuilder> {
		let mut url = format!("{}/v1/models", self.base_url);
		let mut query: Vec<(String, String)> = Vec::new();
		let mut req = self.http.get(&url);
		if !query.is_empty() {
			req = req.query(&query);
		}
		if let Some((ref name, ref value)) = self.auth {
			req = req.header(name, value);
		}
		Ok(req)
	}

	/// get /v1/models
	pub async fn models_v1_v1_models_get(&self) -> Result<Value> {
		let resp = self.models_v1_v1_models_get_request()?.send().await
			.with_context(|| "get /v1/models failed")?;
		let status = resp.status();
		let body = resp.text().await.unwrap_or_default();
		if !status.is_success() {
			anyhow::bail!("models_v1_v1_models_get: HTTP {} - {}", status.as_u16(), body);
		}
		let value: Value = serde_json::from_str(&body)
			.with_context(|| "models_v1_v1_models_get: response not valid JSON")?;
		Ok(value)
	}

	/// get /v1/models, with the HTTP status preserved — so a caller can
	/// read 404 or 401 as an OUTCOME rather than a failure, without matching
	/// on an error message.
	pub async fn models_v1_v1_models_get_raw(&self) -> Result<(u16, String)> {
		let resp = self.models_v1_v1_models_get_request()?.send().await
			.with_context(|| "get /v1/models failed")?;
		let status = resp.status();
		let body = resp.text().await.unwrap_or_default();
		Ok((status.as_u16(), body))
	}

	/// get /v1/models, with the response as BYTES and the status preserved —
	/// for a body that is not JSON (NDJSON, an archive) and for the same
	/// 404/401-as-outcome reads the raw form serves.
	pub async fn models_v1_v1_models_get_bytes(&self) -> Result<(u16, Vec<u8>)> {
		let resp = self.models_v1_v1_models_get_request()?.send().await
			.with_context(|| "get /v1/models failed")?;
		let status = resp.status();
		let body = resp.bytes().await.unwrap_or_default();
		Ok((status.as_u16(), body.to_vec()))
	}
}

/// Types derived from the supplier spec: every shape the document
/// declares, named as the document names it. Open tails flatten, so
/// extension keys survive a round-trip.
pub mod types {
	use serde_json::Value;

	/// `ModelMetadata`, as the supplier spec declares it. Open tail
	/// flattened, so extension keys survive a round-trip.
	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct ModelMetadata {
		
		pub name: String,
		
		pub description: String,
		
		pub release_date: String,
		/// Everything the spec leaves open, preserved on round-trip.
		#[serde(flatten)]
		pub extra: Value,
	}

	/// `SystemoneV1SystemonePostPayloadState` — the supplier declares this as a choice of shapes.
	/// Untagged: every declared alternative parses, and a consumer
	/// matches on the variant instead of inspecting raw JSON.
	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	#[serde(untagged)]
	pub enum SystemoneV1SystemonePostPayloadState {
		Text(String),
		ValueList(Vec<Value>),
		Open(Value),
	}

	/// `Usage`, as the supplier spec declares it. Open tail
	/// flattened, so extension keys survive a round-trip.
	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct Usage {
		
		pub input_tokens: i64,
		
		pub output_tokens: i64,
		/// Everything the spec leaves open, preserved on round-trip.
		#[serde(flatten)]
		pub extra: Value,
	}

	/// POST /v1/systemone, as the supplier spec declares it.
	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct SystemoneV1SystemonePostPayload {
		
		pub state: SystemoneV1SystemonePostPayloadState,
		
		pub model: String,
		
		pub questions: Value,
		/// Everything the spec leaves open, preserved on round-trip.
		#[serde(flatten)]
		pub extra: Value,
	}

}

/// Response views derived from the supplier spec's declared 200
/// properties. Open tail flattened; unset optional fields omit.
pub mod response_types {
	use serde_json::Value;
	#[allow(unused_imports)]
	use super::types::*;

	/// POST /v1/systemone, as the supplier spec declares it.
	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct SystemoneV1SystemonePostResponse {
		
		pub model: String,
		
		pub answers: Value,
		
		pub usage: Usage,
		/// Everything the spec leaves open, preserved on round-trip.
		#[serde(flatten)]
		pub extra: Value,
	}

	/// GET /v1/models, as the supplier spec declares it.
	#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
	pub struct ModelsV1V1ModelsGetResponse {
		
		pub models: Vec<ModelMetadata>,
		/// Everything the spec leaves open, preserved on round-trip.
		#[serde(flatten)]
		pub extra: Value,
	}

}

#[cfg(test)]
mod declared_shapes {
	//! Round-trip cases DERIVED from the supplier document — its own
	//! published examples where it gives them, and an instance built from
	//! the declared required properties where it does not. A type that
	//! cannot hold what its own document declares is wrong.
	#[allow(unused_imports)]
	use crate::types::*;
	#[allow(unused_imports)]
	use crate::response_types::*;

	/// Input: the instance the supplier publishes for this shape.
	#[test]
	fn systemone_v1_systemone_post_payload_state_holds_the_published_example_1() {
		let declared: serde_json::Value = serde_json::from_str(r#""I was charged twice. Please help.""#)
			.expect("the derived instance is JSON");
		let parsed: SystemoneV1SystemonePostPayloadState = serde_json::from_value(declared.clone())
			.expect("the generated type holds what the document declares");
		let round_tripped = serde_json::to_value(&parsed)
			.expect("the parsed value re-serializes");
		assert_eq!(round_tripped, declared, "the round trip loses nothing");
	}

	/// Input: the instance the supplier publishes for this shape.
	#[test]
	fn systemone_v1_systemone_post_payload_state_holds_the_published_example_2() {
		let declared: serde_json::Value = serde_json::from_str(r#"{"message":"Please help.","subject":"Duplicate charge"}"#)
			.expect("the derived instance is JSON");
		let parsed: SystemoneV1SystemonePostPayloadState = serde_json::from_value(declared.clone())
			.expect("the generated type holds what the document declares");
		let round_tripped = serde_json::to_value(&parsed)
			.expect("the parsed value re-serializes");
		assert_eq!(round_tripped, declared, "the round trip loses nothing");
	}

	/// Input: every property the document declares required.
	#[test]
	fn systemone_v1_systemone_post_payload_holds_what_the_document_declares() {
		let declared: serde_json::Value = serde_json::from_str(r#"{"state":"x","model":"x","questions":{}}"#)
			.expect("the derived instance is JSON");
		let parsed: SystemoneV1SystemonePostPayload = serde_json::from_value(declared.clone())
			.expect("the generated type holds what the document declares");
		let round_tripped = serde_json::to_value(&parsed)
			.expect("the parsed value re-serializes");
		assert_eq!(round_tripped, declared, "the round trip loses nothing");
	}

	/// Input: every property the document declares required.
	#[test]
	fn systemone_v1_systemone_post_response_holds_what_the_document_declares() {
		let declared: serde_json::Value = serde_json::from_str(r#"{"model":"x","answers":{},"usage":{"input_tokens":1,"output_tokens":1}}"#)
			.expect("the derived instance is JSON");
		let parsed: SystemoneV1SystemonePostResponse = serde_json::from_value(declared.clone())
			.expect("the generated type holds what the document declares");
		let round_tripped = serde_json::to_value(&parsed)
			.expect("the parsed value re-serializes");
		assert_eq!(round_tripped, declared, "the round trip loses nothing");
	}

	/// Input: every property the document declares required.
	#[test]
	fn models_v1_v1_models_get_response_holds_what_the_document_declares() {
		let declared: serde_json::Value = serde_json::from_str(r#"{"models":[{"description":"x","name":"x","release_date":"x"}]}"#)
			.expect("the derived instance is JSON");
		let parsed: ModelsV1V1ModelsGetResponse = serde_json::from_value(declared.clone())
			.expect("the generated type holds what the document declares");
		let round_tripped = serde_json::to_value(&parsed)
			.expect("the parsed value re-serializes");
		assert_eq!(round_tripped, declared, "the round trip loses nothing");
	}

}
