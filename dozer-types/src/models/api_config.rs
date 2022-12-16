use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, prost::Message)]
#[serde(default = "default_api_config")]
pub struct ApiConfig {
    #[prost(message, tag = "1")]
    #[serde(default = "default_api_rest")]
    pub rest: Option<ApiRest>,
    #[prost(message, tag = "2")]
    #[serde(default = "default_api_grpc")]
    pub grpc: Option<ApiGrpc>,
    #[prost(bool, tag = "3")]
    pub auth: bool,
    #[prost(message, tag = "4")]
    #[serde(default = "default_api_internal")]
    pub api_internal: Option<ApiInternal>,
    #[prost(message, tag = "5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pipeline_internal")]
    pub pipeline_internal: Option<ApiInternal>,
    #[prost(string, optional, tag = "6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "7")]
    pub id: Option<String>,
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, prost::Message)]
pub struct ApiRest {
    #[prost(uint32, tag = "1")]
    pub port: u32,
    #[prost(string, tag = "2")]
    pub url: String,
    #[prost(bool, tag = "3")]
    pub cors: bool,
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, prost::Message)]
pub struct ApiGrpc {
    #[prost(uint32, tag = "1")]
    pub port: u32,
    #[prost(string, tag = "2")]
    pub url: String,
    #[prost(bool, tag = "3")]
    pub cors: bool,
    #[prost(bool, tag = "4")]
    pub web: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, prost::Message)]
pub struct ApiInternal {
    #[prost(uint32, tag = "1")]
    pub port: u32,
    #[prost(string, tag = "2")]
    pub host: String,
}

fn default_api_internal() -> Option<ApiInternal> {
    Some(ApiInternal {
        port: 50052,
        host: "[::1]".to_owned(),
    })
}
fn default_pipeline_internal() -> Option<ApiInternal> {
    Some(ApiInternal {
        port: 50053,
        host: "[::1]".to_owned(),
    })
}
fn default_api_rest() -> Option<ApiRest> {
    Some(ApiRest {
        port: 8080,
        url: "[::0]".to_owned(),
        cors: true,
    })
}
fn default_api_grpc() -> Option<ApiGrpc> {
    Some(ApiGrpc {
        port: 50051,
        url: "[::0]".to_owned(),
        cors: true,
        web: true,
    })
}
pub fn default_api_config() -> ApiConfig {
    ApiConfig {
        rest: default_api_rest(),
        grpc: default_api_grpc(),
        auth: false,
        api_internal: default_api_internal(),
        pipeline_internal: default_pipeline_internal(),
        ..Default::default()
    }
}