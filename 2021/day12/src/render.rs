use std::{borrow::Cow, io::Write};

use crate::node::Node;

pub fn render_to<W: Write>(entry: &Node, output: &mut W) {
    dot::render(entry, output).unwrap();
}

impl<'a> dot::Labeller<'a, Node<'a>, (Node<'a>, Node<'a>)> for Node<'a> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("passage").unwrap()
    }

    fn node_id(&'a self, n: &Node<'a>) -> dot::Id<'a> {
        dot::Id::new(n.label).unwrap()
    }

    fn node_label(&'a self, n: &Node<'a>) -> dot::LabelText<'a> {
        dot::LabelText::LabelStr(Cow::Borrowed(n.label))
    }

    fn node_style(&'a self, n: &Node<'a>) -> dot::Style {
        match n {
            Node { big: true, .. } => dot::Style::Bold,
            Node {
                label: "start" | "end",
                ..
            } => dot::Style::Solid,
            _ => dot::Style::Dotted,
        }
    }
}

impl<'a> dot::GraphWalk<'a, Node<'a>, (Node<'a>, Node<'a>)> for Node<'a> {
    fn nodes(&'a self) -> dot::Nodes<'a, Node<'a>> {
        let mut nodes: Vec<Node<'a>> = vec![];
        let mut queue = vec![self];
        while let Some(next) = queue.pop() {
            if !nodes.contains(next) {
                next.targets.iter().for_each(|n| queue.push(n));
                nodes.push(next.clone());
            }
        }

        Cow::<'a, [Node<'a>]>::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, (Node<'a>, Node<'a>)> {
        let mut edges: Vec<(Node<'a>, Node<'a>)> = vec![];
        let mut queue: Vec<(&Node<'a>, &Node<'a>)> =
            self.targets.iter().map(|b| (self, b)).collect();
        while let Some(next) = queue.pop() {
            if !edges.contains(&(next.0.clone(), next.1.clone())) {
                edges.push((next.0.clone(), next.1.clone()));
                next.1.targets.iter().for_each(|n| queue.push((next.1, n)));
            }
        }

        Cow::<'a, [(Node<'a>, Node<'a>)]>::Owned(edges)
    }

    fn source(&'a self, edge: &(Node<'a>, Node<'a>)) -> Node<'a> {
        edge.0.clone()
    }

    fn target(&'a self, edge: &(Node<'a>, Node<'a>)) -> Node<'a> {
        edge.1.clone()
    }
}
