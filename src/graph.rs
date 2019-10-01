use std::collections::HashMap;

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub struct Node {
    pub label: String,
    pub attrs: HashMap<String, String>
}

impl Node {
    pub fn new() -> Self {
        return Node {
            label: String::default(),
            attrs: HashMap::default()
        }
    }

    pub fn with_label(label: String) -> Self {
        return Node {
            label: label,
            attrs: HashMap::default(),
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    pub left_node: Node,
    pub right_node: Node,
    pub attrs: HashMap<String, String>
}

impl Edge {
    pub fn default() -> Self {
        return Edge {
            left_node: Node::new(),
            right_node: Node::new(),
            attrs: HashMap::default()
        }
    }

    pub fn with_nodes(left_node: Node, right_node: Node) -> Self {
        return Edge {
            left_node: left_node,
            right_node: right_node,
            attrs: HashMap::default()
        }
    }
}