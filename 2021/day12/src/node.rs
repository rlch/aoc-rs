#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Node<'a> {
    pub targets: Vec<Node<'a>>,
    pub label: &'a str,
    pub big: bool,
}

impl<'a> Node<'a> {
    pub fn leaf(label: &'a str) -> Self {
        Self {
            targets: vec![],
            label,
            big: label.chars().any(|c| c.is_uppercase()),
        }
    }

    pub fn parse(label: &'a str, input: &'a str) -> Self {
        let mut node = Node::<'a>::leaf(label);
        for line in input.lines() {
            if let Some(b) = match line.split_once('-') {
                Some((a, b)) if a == label => Some(b),
                _ => None,
            } {
                node.targets.push(Node::parse(b, input));
            }
        }

        node
    }

    pub fn is_leaf(&self) -> bool {
        self.targets.is_empty()
    }

    pub fn dfs(&self, callback: &impl Fn(&Node<'a>)) {
        callback(self);
        for n in &self.targets {
            n.dfs(callback);
        }
    }
}
