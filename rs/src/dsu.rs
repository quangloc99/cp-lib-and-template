#![allow(dead_code)]
enum DSUNode {
    Root { size: usize },
    Child { parent: usize },
}
struct DSU {
    nodes: Vec<DSUNode>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            nodes: (0..n).map(|_| DSUNode::Root { size: 1 }).collect(),
        }
    }

    fn find(&mut self, x: usize) -> (usize, usize) {
        match self.nodes[x] {
            DSUNode::Root { size } => (x, size),
            DSUNode::Child { parent } => {
                let (parent, size) = self.find(parent);
                self.nodes[x] = DSUNode::Child { parent };
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
        self.nodes[y] = DSUNode::Child { parent: x };
        self.nodes[x] = DSUNode::Root {
            size: x_size + y_size,
        };
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut dsu = DSU::new(5);
        assert_eq!(dsu.union(0, 1), true);
        assert_eq!(dsu.union(1, 2), true);
        assert_eq!(dsu.union(3, 4), true);
        assert_eq!(dsu.find(0).1, 3);
        assert_eq!(dsu.find(4).1, 2);

        assert_eq!(dsu.union(0, 2), false);
        assert_eq!(dsu.union(0, 4), true);
        assert_eq!(dsu.find(0).1, 5);
    }
}
