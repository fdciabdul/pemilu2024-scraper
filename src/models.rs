use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    pub nama: String,
    pub id: u64,
    pub kode: String,
    pub tingkat: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElectionData {
    pub table: Option<HashMap<String, HashMap<String, u32>>>,
}