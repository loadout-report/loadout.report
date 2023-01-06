use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ApiResponse<T>
    where T: Serialize
{
    response: T,
    error_code: i32,
    throttle_seconds: i32,
    error_status: String,
    message: String,
    message_data: HashMap<String, String>,
    detailed_error_trace: String,
}
