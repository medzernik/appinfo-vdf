use std::ffi::CString;

use super::{VDFAppNode, VDFAppSection, VDF};

const ASSASSINS_CREED2_APP_ID: u32 = 33362;

pub fn update(vdf: &VDF) -> VDF {
    let new_sections = vdf
        .sections
        .iter()
        .filter(|s| s.app_id == ASSASSINS_CREED2_APP_ID)
        .map(update_app_section)
        .collect();
    VDF {
        sections: new_sections,
        header: vdf.header.clone(),
    }
}

fn update_app_section(section: &VDFAppSection) -> VDFAppSection {
    if section.app_id == ASSASSINS_CREED2_APP_ID {
        let nodes = section.nodes.iter().map(update_app_node).collect();
        VDFAppSection {
            nodes: nodes,
            ..*section
        }
    } else {
        section.clone()
    }
}

fn update_app_node(node: &VDFAppNode) -> VDFAppNode {
    const APP_INFO: &[u8] = b"appinfo";

    if let VDFAppNode::Simple { name, children } = node {
        if name.as_bytes() == APP_INFO {
            let mut new_children = children.clone();
            new_children.push(steam_edit_app_node());
            VDFAppNode::Simple {
                name: name.clone(),
                children: new_children,
            }
        } else {
            node.clone()
        }
    } else {
        node.clone()
    }
}

fn steam_edit_app_node() -> VDFAppNode {
    VDFAppNode::Simple {
        name: "steam_edit".to_string(),
        children: vec![
            VDFAppNode::Int {
                name: "is_hidden".to_string(),
                value: 1,
            },
            VDFAppNode::Str {
                name: "base_name".to_string(),
                value: "Assassin's Creed 2 - Mac".to_string(),
            },
            VDFAppNode::Str {
                name: "base_type".to_string(),
                value: "DLC".to_string(),
            },
        ],
    }
}
