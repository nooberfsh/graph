//! http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
//! https://github.com/nikomatsakis/simple-graph

pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

#[derive(Clone, Copy, Debug)]
pub struct NodeIdx(usize);

#[derive(Clone, Copy, Debug)]
struct EdgeIdx(usize);

struct Node<T> {
    data: T,
    first_edge_idx: Option<EdgeIdx>,
}

struct Edge {
    target_node: NodeIdx,
    next_edge_idx: Option<EdgeIdx>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, data: T) -> NodeIdx {
        let idx = self.nodes.len();
        self.nodes.push(Node::new(data));
        NodeIdx(idx)
    }

    pub fn add_edge(&mut self, from: NodeIdx, to: NodeIdx) {
        let edge_idx = self.edges.len();
        let node = &mut self.nodes[from.0];
        let edge = Edge {
            target_node: to,
            next_edge_idx: node.first_edge_idx,
        };
        self.edges.push(edge);
        node.first_edge_idx = Some(EdgeIdx(edge_idx));
    }

    pub fn successors(&self, node_idx: NodeIdx) -> Successors<T> {
        Successors {
            graph: self,
            edge_idx: self.nodes[node_idx.0].first_edge_idx,
        }
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            first_edge_idx: None,
        }
    }
}

pub struct Successors<'a, T> {
    graph: &'a Graph<T>,
    edge_idx: Option<EdgeIdx>,
}

impl<'a, T: 'a> Iterator for Successors<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let edge_idx = self.edge_idx?;
        let edge = &self.graph.edges[edge_idx.0];
        self.edge_idx = edge.next_edge_idx;
        let node_idx = edge.target_node;
        Some(&self.graph.nodes[node_idx.0].data)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {

        // N0 ---E0---> N1 ---E1---> N2
        // |                         ^
        // E2                        |
        // |                         |
        // v                         |
        // N3 ----------E3-----------+

        let mut graph = Graph::new();

        let n0 = graph.add_node('A');
        let n1 = graph.add_node('B');
        let n2 = graph.add_node('C');
        let n3 = graph.add_node('D');

        graph.add_edge(n0, n1); // e0
        graph.add_edge(n1, n2); // e1
        graph.add_edge(n0, n3); // e2
        graph.add_edge(n3, n2); // e3

        let successors: Vec<_> = graph.successors(n0).map(|d| *d).collect();
        assert_eq!(successors, vec!['D', 'B'])
    }
}
