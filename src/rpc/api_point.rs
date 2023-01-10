use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::rpc::Response;


#[derive(Serialize, Deserialize, Debug)]
pub struct PointRequest {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PointResponse {
    pub sum: i32,
}

impl Response<PointRequest> for PointResponse {}