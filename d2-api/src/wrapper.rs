use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub response: Option<T>,
    pub error_code: i32,
    pub throttle_seconds: i32,
    pub error_status: String,
    pub message: String,
    pub message_data: HashMap<String, String>,
    pub detailed_error_trace: Option<String>,
}
