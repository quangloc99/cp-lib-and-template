#![allow(dead_code)]

fn suffix_array(s: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let mut ans = (0..s.len()).collect::<Vec<_>>();
    let mut rank = s.iter().map(|&ch| ch as usize).collect::<Vec<_>>();
    let mut jr = vec![0; s.len()];
    let mut tmp = ans.clone();
    let mut cnt = vec![0; 256];

    const BITS: usize = 30;
    const MASK: usize = (1 << BITS) - 1;

    for lg_sz in 1..=s.len().ilog2() + 1 {
        let sz = 1 << lg_sz;

        for i in 0..s.len() {
            let &r = rank.get(i + sz / 2).unwrap_or(&0); // <---- change this one for cyclic array
            jr[i] = (rank[i] << BITS) | r;
        }
        cnt.resize(if sz == 2 { 256 } else { s.len() + 1 }, 0);
        for shift in [0, BITS] {
            cnt.fill(0);
            for i in 0..s.len() {
                cnt[(jr[i] >> shift) & MASK] += 1;
            }
            for i in 1..cnt.len() {
                cnt[i] += cnt[i - 1];
            }
            assert_eq!(cnt.last(), Some(&s.len()));
            std::mem::swap(&mut ans, &mut tmp);
            for &i in tmp.iter().rev() {
                let r = (jr[i] >> shift) & MASK;
                cnt[r] -= 1;
                ans[cnt[r]] = i;
            }
        }
        rank[ans[0]] = 1;
        for i in 1..s.len() {
            rank[ans[i]] = rank[ans[i - 1]] + (jr[ans[i]] != jr[ans[i - 1]]) as usize;
        }
    }

    (ans, rank)
}

#[cfg(test)]
mod test_suffix_array {
    use super::*;

    #[test]
    fn fuzz_test() {
        for testcase in 0..1000 {
            let mut prng = crate::prng::PRNG::new(testcase);
            let n = prng.rand_usize(1..=1000);
            let s = (0..n)
                .map(|_| prng.rand_usize(b'a' as usize..=b'z' as usize) as u8)
                .collect::<Vec<_>>();
            let (sa, _) = suffix_array(&s);
            for i in 1..n {
                assert!(s[sa[i - 1]..] < s[sa[i]..]);
            }
        }
    }
}
