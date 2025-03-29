#[derive(Debug)]
enum DSUNode {
    Root { size: usize },
    Child { parent: usize },
}
struct DSURollBack {
    nodes: Vec<DSUNode>,
    history: Vec<(usize, DSUNode)>,
}

impl DSURollBack {
    fn new(n: usize) -> Self {
        Self { nodes: (0..n).map(|_| DSUNode::Root { size: 1 }).collect(), history: vec![] }
    }

    fn find(&mut self, x: usize) -> (usize, usize) {
        match self.nodes[x] {
            DSUNode::Root { size } => (x, size),
            DSUNode::Child { parent } => {
                let (parent, size) = self.find(parent);
                let old_node = std::mem::replace(&mut self.nodes[x], DSUNode::Child { parent });
                self.history.push((x, old_node));
                (parent, size)
            }
        }
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut x, mut x_size) = self.find(x);
        let (mut y, mut y_size) = self.find(y);
        if x == y {
            return false;
        }
        if x_size < y_size {
            (x, y, x_size, y_size) = (y, x, y_size, x_size);
        }
        let old_y = std::mem::replace(&mut self.nodes[y], DSUNode::Child { parent: x });
        let old_x = std::mem::replace(&mut self.nodes[x], DSUNode::Root { size: x_size + y_size });
        self.history.push((x, old_x));
        self.history.push((y, old_y));
        true
    }

    fn stamp(&self) -> usize {
        self.history.len()
    }

    fn rollback(&mut self, stamp: usize) {
        for (x, node) in self.history.drain(stamp..).rev() {
            self.nodes[x] = node;
        }
    }
}
