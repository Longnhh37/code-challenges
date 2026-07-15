pub mod graph {
    use std::collections::HashMap;

    // ========= Attribute =========

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Attribute(HashMap<String, String>);

    impl Attribute {
        pub fn get(&self, key: &str) -> Option<&str> {
            self.0.get(key).map(|s| s.as_str())
        }

        pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
            self.0.insert(key.into(), value.into());
        }

        pub fn extend(&mut self, attrs: &[(&str, &str)]) {
            for (k, v) in attrs {
                self.0.insert((*k).into(), (*v).into());
            }
        }
    }

    // ========= Node =========

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Node {
        id: String,
        attrs: Attribute,
    }

    impl Node {
        pub fn new(id: impl Into<String>) -> Self {
            Self {
                id: id.into(),
                attrs: Attribute::default(),
            }
        }

        pub fn id(&self) -> &str {
            &self.id
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key)
        }

        pub fn update_attr(&mut self, key: &str, value: &str) {
            self.attrs.insert(key, value);
        }

        pub fn extend_attrs(&mut self, attrs: &[(&str, &str)]) {
            self.attrs.extend(attrs);
        }
    }

    // ========= Edge =========

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Edge {
        from: String,
        to: String,
        attrs: Attribute,
    }

    impl Edge {
        pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
            Self {
                from: from.into(),
                to: to.into(),
                attrs: Attribute::default(),
            }
        }

        pub fn endpoints(&self) -> (&str, &str) {
            (&self.from, &self.to)
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key)
        }

        pub fn update_attr(&mut self, key: &str, value: &str) {
            self.attrs.insert(key, value);
        }

        pub fn extend_attrs(&mut self, attrs: &[(&str, &str)]) {
            self.attrs.extend(attrs);
        }
    }

    // ========= Graph =========

    #[derive(Debug, Default)]
    pub struct Graph {
        nodes: HashMap<String, Node>,
        edges: Vec<Edge>,
        attrs: Attribute,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        // ---------- READ ----------

        pub fn get_node(&self, id: &str) -> Option<&Node> {
            self.nodes.get(id)
        }

        pub fn get_node_mut(&mut self, id: &str) -> Option<&mut Node> {
            self.nodes.get_mut(id)
        }

        pub fn edges(&self) -> &[Edge] {
            &self.edges
        }

        pub fn attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key)
        }

        // ---------- WRITE ----------

        pub fn add_node(&mut self, node: Node) {
            self.nodes.insert(node.id.clone(), node);
        }

        pub fn add_edge(&mut self, edge: Edge) -> Result<(), String> {
            if !self.nodes.contains_key(&edge.from) {
                return Err(format!("Missing node: {}", edge.from));
            }
            if !self.nodes.contains_key(&edge.to) {
                return Err(format!("Missing node: {}", edge.to));
            }
            self.edges.push(edge);
            Ok(())
        }

        pub fn update_graph_attr(&mut self, key: &str, value: &str) {
            self.attrs.insert(key, value);
        }

        pub fn extend_graph_attrs(&mut self, attrs: &[(&str, &str)]) {
            self.attrs.extend(attrs);
        }

        // ---------- BULK ----------

        pub fn add_nodes(&mut self, nodes: impl IntoIterator<Item = Node>) {
            for n in nodes {
                self.nodes.insert(n.id.clone(), n);
            }
        }

        pub fn add_edges(&mut self, edges: impl IntoIterator<Item = Edge>) {
            for e in edges {
                let _ = self.add_edge(e);
            }
        }
    }
}
fn main() {}
