pub trait Graph {
    fn get_n(&self) -> usize;
    fn get_adj(&self, u: usize) -> impl Iterator<Item = usize>;
}

#[rustfmt::skip]
impl Graph for Vec<Vec<usize>> {
    fn get_n(&self) -> usize { self.len() }
    fn get_adj(&self, u: usize) -> impl Iterator<Item = usize> {
        self[u].iter().copied()
    }
}
