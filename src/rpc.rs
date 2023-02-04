use std::collections::HashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use crate::g_result::GResult;

pub mod api_point;
pub mod context_handler;
pub mod conversions;

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
