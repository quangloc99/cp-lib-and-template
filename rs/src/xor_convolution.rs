#![allow(dead_code)]
use crate::number_trait::Number;

// TODO: make sure this works with normal integer numbers
fn xor_fst<const INV: bool, T: Number>(a: &mut [T]) {
    assert!(a.len().is_power_of_two());
    for pw in 0..a.len().trailing_zeros() {
        for ck in a.chunks_mut(2 << pw) {
            let (c0, c1) = ck.split_at_mut(1 << pw);
            for (u, v) in c0.iter_mut().zip(c1.iter_mut()) {
                (*u, *v) = (*u + *v, *u - *v);
            }
        }
    }
    if INV {
        let inv = T::one() / T::from(a.len());
        for x in a.iter_mut() {
            *x = *x * inv;
        }
    }
}

fn xor_convolution<T: Number>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> {
    let n = a.len();
    assert!(n == b.len());
    xor_fst::<false, T>(&mut a);
    xor_fst::<false, T>(&mut b);
    for i in 0..n {
        a[i] = a[i] * b[i];
    }
    xor_fst::<true, T>(&mut a);
    a
}
