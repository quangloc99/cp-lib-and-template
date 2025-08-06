// verification https://codeforces.com/contest/628/submission/332669632
#![allow(unused)]
use std::collections::VecDeque;

#[derive(Default, Copy, Clone)]
struct Edge {
    to: usize,
    rev_id: usize,
    cap: usize,
}

struct Dinic {
    gr: Vec<Vec<Edge>>,
    dist: Vec<usize>,
    dfs_id: Vec<usize>, // this one is important, since we DFS in DAG manner instead of tree.
}

impl Dinic {
    fn new(n: usize) -> Self {
        Self { gr: vec![vec![]; n], dist: vec![0; n], dfs_id: vec![0; n] }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: usize) {
        let u_id = self.gr[v].len();
        let v_id = self.gr[u].len();
        self.gr[u].push(Edge { to: v, rev_id: u_id, cap });
        self.gr[v].push(Edge { to: u, rev_id: v_id, cap: 0 });
    }

    fn bfs(&mut self, src: usize, sink: usize) -> bool {
        self.dist.fill(usize::MAX);
        let mut qu = VecDeque::<usize>::new();
        self.dist[src] = 0;
        qu.push_back(src);
        while let Some(u) = qu.pop_front() {
            for &Edge { to: v, cap, .. } in self.gr[u].iter() {
                if cap == 0 || self.dist[v] != usize::MAX {
                    continue;
                }
                self.dist[v] = self.dist[u] + 1;
                qu.push_back(v);
            }
        }
        return self.dist[sink] != usize::MAX;
    }

    fn dfs(&mut self, u: usize, sink: usize, mut cap_left: usize) -> usize {
        if u == sink {
            return cap_left;
        }
        let mut total_inc = 0usize;
        let num_edges = self.gr[u].len();
        while self.dfs_id[u] < num_edges {
            let cur_id = self.dfs_id[u];
            self.dfs_id[u] += 1;

            let Edge { to: v, rev_id, cap: _ } = self.gr[u][cur_id];
            if self.dist[u] + 1 != self.dist[v] || self.gr[u][cur_id].cap == 0 {
                continue;
            }
            let cur_inc = self.dfs(v, sink, cap_left.min(self.gr[u][cur_id].cap));
            self.gr[u][cur_id].cap -= cur_inc;
            self.gr[v][rev_id].cap += cur_inc;
            cap_left -= cur_inc;
            total_inc += cur_inc;

            if cap_left == 0 {
                break;
            }
        }
        total_inc
    }

    fn inc_flow(&mut self, src: usize, sink: usize) -> usize {
        if !self.bfs(src, sink) {
            return 0;
        }
        self.dfs_id.fill(0);
        self.dfs(src, sink, usize::MAX)
    }
}
