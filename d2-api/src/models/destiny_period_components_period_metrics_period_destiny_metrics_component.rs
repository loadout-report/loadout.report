/*
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * The version of the OpenAPI document: 2.16.0
 * Contact: support@bungie.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DestinyPeriodComponentsPeriodMetricsPeriodDestinyMetricsComponent {
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<::std::collections::HashMap<String, crate::models::DestinyPeriodComponentsPeriodMetricsPeriodDestinyMetricComponent>>,
    #[serde(rename = "metricsRootNodeHash", skip_serializing_if = "Option::is_none")]
    pub metrics_root_node_hash: Option<i32>,
}

impl DestinyPeriodComponentsPeriodMetricsPeriodDestinyMetricsComponent {
    pub fn new() -> DestinyPeriodComponentsPeriodMetricsPeriodDestinyMetricsComponent {
        DestinyPeriodComponentsPeriodMetricsPeriodDestinyMetricsComponent {
            metrics: None,
            metrics_root_node_hash: None,
        }
    }
}


