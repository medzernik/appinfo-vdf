// use std::ffi::CString;

use std::collections::HashMap;

pub mod printer;
pub mod reader;
// pub mod updater;
// pub mod writer;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VDF {
    pub header: VDFHeader,
    pub sections: Vec<VDFAppSection>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VDFHeader {
    pub magic: u32,
    pub version: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VDFAppSection {
    pub app_id: u32,
    pub data_size: u32,
    pub info_state: u32,
    pub last_updated: u32,
    pub pics_token: u64,
    pub sha1: [u8; 20],
    pub change_number: u32,
    pub binary_hash: [u8; 20],
    pub nodes: VDFAppNode,
}

// #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
// pub enum VDFAppNode {
//     Simple {
//         name: String,
//         children: Vec<VDFAppNode>,
//     },
//     Str {
//         name: String,
//         value: String,
//     },
//     Int {
//         name: String,
//         value: u32,
//     },
// }

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum VDFValue{
    Object(VDFAppNode),
    Str(String),
    Int(u32)
}

pub type VDFAppNode = HashMap<String, VDFValue>;

pub enum VDFAppNodeKind {
    Simple = 0,
    Str = 1,
    Int = 2,
    End = 8,
}


