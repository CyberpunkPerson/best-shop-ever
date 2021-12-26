/*
 * Best shop ever API
 *
 * Test project for Rust education purpose
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: cyberpunk.perosn@gmail.com
 * Generated by: https://openapi-generator.tech
 */

/// ErrorResponse : Error response



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error identificator
    #[serde(rename = "errorId")]
    pub error_id: String,
    /// Service error code
    #[serde(rename = "errorCode")]
    pub error_code: String,
    /// service error message
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    /// Error details
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<serde_json::Value>,
}

impl ErrorResponse {
    /// Error response
    pub fn new(error_id: String, error_code: String, error_message: String) -> ErrorResponse {
        ErrorResponse {
            error_id,
            error_code,
            error_message,
            error_details: None,
        }
    }
}

