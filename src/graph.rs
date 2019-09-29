use std::collections::HashMap;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>
}

impl Graph {
    pub fn new() -> Self {
        return Graph {
            nodes: vec![],
            edges: vec![],
            attrs: HashMap::default()
        }
    }
}

pub struct Node {
    pub label: String,
    attrs: HashMap<String, String>
}

impl Node {
    pub fn new() -> Self {
        return Node {
            label: String::default(),
            attrs: HashMap::default()
        }
    }
}

pub struct Edge {
    pub left_node: Node,
    pub right_node: Node,
    attrs: HashMap<String, String>
}

impl Edge {
    pub fn new(left_node: Node, right_node: Node) -> Self {
        return Edge {
            left_node: left_node,
            right_node: right_node,
            attrs: HashMap::default()
        }
    }
}