use crate::vdf::{MAGIC, MAGIC28, MAGIC29};

use super::{VDFAppNode, VDFAppNodeKind, VDFAppSection, VDFHeader, VDFValue, VDF};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::CString;
use std::io::{self};

pub struct ParseError<'a>(&'a str);

impl<'a> From<&'a str> for ParseError<'a> {
    fn from(s: &'a str) -> ParseError<'a> {
        ParseError(s)
    }
}

impl From<ParseError<'_>> for io::Error {
    fn from(error: ParseError) -> io::Error {
        let ParseError(msg) = error;
        io::Error::new(io::ErrorKind::InvalidInput, msg)
    }
}

type ParseResult<'a, 'b, T> = Result<(&'a [u8], T), ParseError<'b>>;

pub fn read(input: &[u8]) -> ParseResult<VDF> {
    let (input, header) = parse_vdf_header(input)?;
    let (input, sections) = parse_vdf_app_sections(input, header.offset)?;
    Ok((input, VDF { header, sections }))
}

fn parse_vdf_header(input: &[u8]) -> ParseResult<VDFHeader> {
    let (input, magic) = parse_u32le(input)?;
    let (input, version) = parse_u32le(input)?;

    match magic {
        MAGIC29 => {
            println!("Using Magic 29");
        }
        MAGIC28 => {
            println!("Using Magic 28");
        }
        MAGIC => {
            println!("Using Magic OG");
        }
        _ => return Err(ParseError("Invalid magic number: {magic}")),
    }
    println!("DEBUG: version {version}");

    let (input, (offset, string_table)) = parse_offset(input)?;
    println!("TABLLES:\n {offset}\n {string_table:?}");

    Ok((
        input,
        VDFHeader {
            magic,
            version,
            offset,
        },
    ))
}

fn parse_vdf_app_sections(input: &[u8], offset: i64) -> ParseResult<Vec<VDFAppSection>> {
    let mut sections = Vec::new();
    let mut vdf_string_table: Vec<String> = Vec::new();
    let mut input2 = input;
    let mut result = Err(ParseError("Couldn't parse VDF app section"));

    loop {
        if matches_bytes(input2, &[0, 0, 0, 0]) {
            result = Ok((input2, sections));
            println!("DEBUG: OK, 0000, EMPTY");
            break;
        } else if let Ok((input, section)) = parse_vdf_app_section(input2) {
            sections.push(section);
            println!("DEBUG: SECTION PUSHED!");
            input2 = input;
        } else {
            println!("DEBUG: NO SECTION FOUND");
            break;
        }
    }

    result
}

fn parse_vdf_app_section(input: &[u8]) -> ParseResult<VDFAppSection> {
    let (input, app_id) = parse_u32le(input)?;
    let (input, data_size) = parse_u32le(input)?;
    let data_size2 = data_size
        .try_into()
        .map_err(|_| "Couldn't convert to usize")?;
    let (input, data) = parse_take_n(input, data_size2)?;
    let (data, info_state) = parse_u32le(data)?;
    let (data, last_updated) = parse_u32le(data)?;
    let (data, pics_token) = parse_u64le(data)?;
    let (data, sha1) = parse_take_n(data, 20)?;
    let (data, change_number) = parse_u32le(data)?;
    let (data, binary_hash) = parse_take_n(data, 20)?;
    let (_data, nodes) = parse_vdf_app_nodes(data)?;
    Ok((
        input,
        VDFAppSection {
            app_id,
            data_size,
            info_state,
            last_updated,
            pics_token,
            sha1: sha1.try_into().unwrap(),
            change_number,
            binary_hash: binary_hash.try_into().unwrap(),
            nodes,
        },
    ))
}

fn parse_vdf_app_nodes(input: &[u8]) -> ParseResult<VDFAppNode> {
    let mut input2 = input;
    let mut children: VDFAppNode = HashMap::new();
    let mut result = Err(ParseError("Couldn't parse VDF app nodes"));

    loop {
        if matches_bytes(input2, &[VDFAppNodeKind::End as u8]) {
            result = Ok((&input2[1..], children));
            break;
        } else if let Ok((input, node)) = parse_vdf_app_node(input2) {
            //TODO: hashmap.push
            children.insert(node.0, node.1);

            input2 = input;
        } else {
            break;
        }
    }

    result
}

fn parse_vdf_app_node(input: &[u8]) -> ParseResult<(String, VDFValue)> {
    let (input, kind) = parse_take_n(input, 1)?;

    match kind[0] {
        k if k == VDFAppNodeKind::Simple as u8 => parse_vdf_app_node_simple(input),
        k if k == VDFAppNodeKind::Str as u8 => parse_vdf_app_node_str(input),
        k if k == VDFAppNodeKind::Int as u8 => parse_vdf_app_node_int(input),
        _ => Err(ParseError("Unrecognized VDF app node kind")),
    }
}

fn parse_vdf_app_node_simple(input: &[u8]) -> ParseResult<(String, VDFValue)> {
    let (input, name) = parse_vdf_str(input, 0)?;
    let (input, children) = parse_vdf_app_nodes(input)?;

    Ok((
        input,
        (
            name.to_string_lossy().to_string(),
            VDFValue::Object(children),
        ),
    ))
}

fn parse_vdf_app_node_str(input: &[u8]) -> ParseResult<(String, VDFValue)> {
    let (input, name) = parse_vdf_str(input, 0)?;
    let (input, value) = parse_vdf_str(input, 0)?;
    Ok((
        input,
        (
            name.to_string_lossy().to_string(),
            VDFValue::Str(value.to_string_lossy().to_string()),
        ),
    ))
}

fn parse_vdf_app_node_int(input: &[u8]) -> ParseResult<(String, VDFValue)> {
    let (input, name) = parse_vdf_str(input, 0)?;
    let (input, value) = parse_u32le(input)?;

    Ok((
        input,
        (name.to_string_lossy().to_string(), VDFValue::Int(value)),
    ))
}

fn parse_vdf_str(input: &[u8], offset: i64) -> ParseResult<CString> {
    let err_msg = "Couldn't parse VDF string";
    let pos = input
        .iter()
        .skip(offset as usize)
        .position(|b| *b == b'\0')
        .ok_or(err_msg)?;
    let (input, bytes) = parse_take_n(input, pos)?;
    let string = unsafe { CString::from_vec_unchecked(bytes.to_vec()) };
    Ok((&input[1..], string))
}

fn parse_magic(input: &[u8]) -> ParseResult<u32> {
    let (input, value) = parse_u32le(input)?;
    Ok((input, value))
}

fn parse_offset(input: &[u8]) -> ParseResult<(i64, Vec<String>)> {
    let (input, string_offset) = parse_i64le(input)?;
    let mut offset = 0;
    let mut string_table: Vec<String> = Vec::new();

    let prev_offset = offset;
    offset = string_offset;
    let (input, string_count) = parse_i32le(input)?;
    for _ in 0..string_count {
        let (input, str) = parse_vdf_str(input, offset)?;
        string_table.push(str.to_string_lossy().to_string());
    }
    offset = prev_offset;
    Ok((input, (offset, string_table)))
}

fn parse_u32le(input: &[u8]) -> ParseResult<u32> {
    let err_msg = "Couldn't parse u32le";
    let size = std::mem::size_of::<u32>();
    let (input, int_bytes) = parse_take_n(input, size).map_err(|_| err_msg)?;
    match int_bytes.try_into() {
        Ok(bytes) => Ok((input, u32::from_le_bytes(bytes))),
        Err(_) => Err(ParseError(err_msg)),
    }
}
fn parse_i32le(input: &[u8]) -> ParseResult<i32> {
    let err_msg = "Couldn't parse i32le";
    let size = std::mem::size_of::<i32>();
    let (input, int_bytes) = parse_take_n(input, size).map_err(|_| err_msg)?;
    match int_bytes.try_into() {
        Ok(bytes) => Ok((input, i32::from_le_bytes(bytes))),
        Err(_) => Err(ParseError(err_msg)),
    }
}

fn parse_u64le(input: &[u8]) -> ParseResult<u64> {
    let err_msg = "Couldn't parse u64le";
    let size = std::mem::size_of::<u64>();
    let (input, int_bytes) = parse_take_n(input, size).map_err(|_| err_msg)?;
    match int_bytes.try_into() {
        Ok(bytes) => Ok((input, u64::from_le_bytes(bytes))),
        Err(_) => Err(ParseError(err_msg)),
    }
}

fn parse_i64le(input: &[u8]) -> ParseResult<i64> {
    let err_msg = "Couldn't parse i64le";
    let size = std::mem::size_of::<i64>();
    let (input, int_bytes) = parse_take_n(input, size).map_err(|_| err_msg)?;
    match int_bytes.try_into() {
        Ok(bytes) => Ok((input, i64::from_le_bytes(bytes))),
        Err(_) => Err(ParseError(err_msg)),
    }
}

fn parse_take_n(input: &[u8], n: usize) -> ParseResult<&[u8]> {
    if input.len() >= n {
        let (bytes, input) = input.split_at(n);
        Ok((input, bytes))
    } else {
        Err(ParseError("Couldn't parse take_n"))
    }
}

fn matches_bytes(input: &[u8], value: &[u8]) -> bool {
    if let Ok((_, bytes)) = parse_take_n(input, value.len()) {
        bytes == value
    } else {
        false
    }
}
