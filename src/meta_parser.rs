use std::collections::HashMap;
use std::fs;
use std::io;
use serde::{Deserialize, Serialize};

pub struct NodeInfo {
    parent: Option<u32>,
    incoming: Vec<Vec<u32>>,
    outgoing: Vec<Vec<u32>>
}

pub(crate) fn parse_meta(path: &str) -> Result<(HashMap<u32, Vec<u32>>, HashMap<u32, NodeInfo>), io::Error> {
    let file_str = fs::read_to_string(path)?;
    let file_data: MetaFile = serde_json::from_str(&file_str)?;

    let mut supernodes: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut nodes: HashMap<u32, NodeInfo> = HashMap::new();

    for snode in file_data.s {
        supernodes.insert(snode.i, snode.g);
    }

    for node in file_data.q {
        nodes.insert(node.i, NodeInfo { parent: node.p, incoming: node.n, outgoing: node.o });
    }

    Ok((supernodes, nodes))
}

#[derive(Serialize, Deserialize)]
struct MetaFile {
    s: Vec<Supernode>,
    q: Vec<Node>
}

#[derive(Serialize, Deserialize)]
struct Node {
    i: u32,
    p: Option<u32>,
    n: Vec<Vec<u32>>,
    o: Vec<Vec<u32>>
}

#[derive(Serialize, Deserialize)]
struct Supernode {
    i: u32,
    g: Vec<u32>
}