use crate::graph_traits::Graph;

struct KuhnMatcher {
    vis: Vec<usize>,
    timer: usize,
    matched: Vec<Option<usize>>,
}

impl KuhnMatcher {
    fn new(n: usize, m: usize) -> Self {
        Self { vis: vec![0; n], timer: 0, matched: vec![None; m] }
    }

    fn dfs(&mut self, gr: &impl Graph, u: usize) -> bool {
        if self.vis[u] == self.timer {
            return false;
        }
        self.vis[u] = self.timer;
        for v in gr.get_adj(u) {
            if let Some(w) = self.matched[v] {
                if !self.dfs(gr, w) {
                    continue;
                }
            }
            self.matched[v] = Some(u);
            return true;
        }
        return false;
    }

    pub fn do_match(&mut self, adj: &impl Graph, u: usize) -> bool {
        self.timer += 1;
        return self.dfs(adj, u);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kuhn() {
        let mut matcher = KuhnMatcher::new(4, 3);
        let adj = vec![vec![0, 1], vec![2], vec![2, 0], vec![]];
        assert_eq!(matcher.do_match(&adj, 0), true);
        assert_eq!(matcher.do_match(&adj, 1), true);
        assert_eq!(matcher.do_match(&adj, 2), true);
        assert_eq!(matcher.matched, vec![Some(2), Some(0), Some(1)]);
        assert_eq!(matcher.do_match(&adj, 3), false);
    }
}
