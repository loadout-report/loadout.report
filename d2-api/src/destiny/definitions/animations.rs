use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AnimationReference {
    pub anim_name: String,
    pub anim_identifier: String,
    pub path: String,
}

