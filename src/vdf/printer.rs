use super::{VDFAppNode, VDFAppSection, VDF};

pub fn print(vdf: &VDF) {
    println!("# VDFHeader");
    println!("magic: {}", vdf.header.magic);
    println!("version: {}\n", vdf.header.version);

    for section in &vdf.sections {
        print_app_section(&section);
        println!("\n");
    }
}

fn print_app_section(section: &VDFAppSection) {
    println!("# VDFAppSection");
    println!("app_id: {}", section.app_id);
    println!("data_size: {}", section.data_size);
    println!("info_state: {}", section.info_state);
    println!("last_updated: {}", section.last_updated);
    println!("pics_token: {}", section.pics_token);
    println!("sha1: {:?}", section.sha1);
    println!("change_number: {}", section.change_number);
    print_app_nodes(&section.nodes, 0);
}

fn print_app_nodes(nodes: &[VDFAppNode], level: usize) {
    println!("{{");
    for (i, node) in nodes.iter().enumerate() {
        print_app_node(node, level + 1);
        let sep = if i == nodes.len() - 1 { "" } else { "," };
        println!("{}", sep);
    }
    print!("{:width$}}}", "", width = level * 2);
}

fn print_app_node(node: &VDFAppNode, level: usize) {
    match node {
        VDFAppNode::Simple { name, children } => {
            print!("{:width$}{:?}: ", "", name, width = level * 2);

            print_app_nodes(children, level);
        }
        VDFAppNode::Str { name, value } => {
            print!("{:width$}{:?}: {:?}", "", name, value, width = level * 2);
        }
        VDFAppNode::Int { name, value } => {
            print!("{:width$}{:?}: {}", "", name, value, width = level * 2);
        }
    }
}

//OUTPUT SECTION
pub fn print_output(vdf: &VDF) -> String {
    let mut output_string = String::new();
    // println!("# VDFHeader");

    output_string.push_str(format!("# VDFHeader\n").as_str());

    // println!("magic: {}", vdf.header.magic);
    output_string.push_str(format!("magic: {}\n", vdf.header.magic).as_str());

    // println!("version: {}\n", vdf.header.version);
    output_string.push_str(format!("version: {}\n\n", vdf.header.version).as_str());

    for section in &vdf.sections {
        output_string.push_str(print_app_section_output(&section).as_str());
        // println!("\n");
        output_string.push_str(format!("\n\n").as_str());
    }
    output_string
}

fn print_app_nodes_output(nodes: &[VDFAppNode], level: usize) -> String {
    let mut output_string = String::new();
    // println!("{{");
    output_string.push_str(format!("{{\n").as_str());

    for (i, node) in nodes.iter().enumerate() {
        output_string.push_str(print_app_node_output(node, level + 1).as_str());
        let sep = if i == nodes.len() - 1 { "" } else { "," };
        // println!("{}", sep);
        output_string.push_str(format!("{}\n", sep).as_str());
    }
    // print!("{:width$}}}", "", width = level * 2);
    output_string.push_str(format!("{:width$}}}", "", width = level * 2).as_str());

    output_string
}

fn print_app_section_output(section: &VDFAppSection) -> String {
    let mut output_string = String::new();

    // println!("# VDFAppSection");
    output_string.push_str("# VDFAppSection");

    // println!("app_id: {}", section.app_id);
    output_string.push_str(format!("app_id: {}\n", section.app_id).as_str());

    // println!("data_size: {}", section.data_size);
    output_string.push_str(format!("data_size: {}\n", section.data_size).as_str());

    // println!("info_state: {}", section.info_state);
    output_string.push_str(format!("data_size: {}\n", section.data_size).as_str());

    // println!("last_updated: {}", section.last_updated);
    output_string.push_str(format!("last_updated: {}\n", section.last_updated).as_str());

    // println!("pics_token: {}", section.pics_token);
    output_string.push_str(format!("pics_token: {}\n", section.pics_token).as_str());

    // println!("sha1: {:?}", section.sha1);
    output_string.push_str(format!("sha1: {:?}\n", section.sha1).as_str());

    // println!("change_number: {}", section.change_number);
    output_string.push_str(format!("change_number: {}\n", section.change_number).as_str());

    output_string.push_str(print_app_nodes_output(&section.nodes, 0).as_str());

    output_string
}

fn print_app_node_output(node: &VDFAppNode, level: usize) -> String {
    let mut output_string = String::new();
    match node {
        VDFAppNode::Simple { name, children } => {
            // print!("{:width$}{:?}: ", "", name, width = level * 2);
            output_string
                .push_str(format!("{:width$}{:?}: ", "", name, width = level * 2).as_str());
            output_string.push_str(print_app_nodes_output(children, level).as_str());
        }
        VDFAppNode::Str { name, value } => {
            // print!("{:width$}{:?}: {:?}", "", name, value, width = level * 2);
            output_string.push_str(
                format!("{:width$}{:?}: {:?}", "", name, value, width = level * 2).as_str(),
            );
        }
        VDFAppNode::Int { name, value } => {
            // print!("{:width$}{:?}: {}", "", name, value, width = level * 2);
            output_string.push_str(
                format!("{:width$}{:?}: {}", "", name, value, width = level * 2).as_str(),
            );
        }
    }

    output_string
}
