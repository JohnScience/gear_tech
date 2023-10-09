use rayon::prelude::*;

const THRESHOLD: usize = 200_000;

fn copying_xor(inp: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(inp.len());

    let f = |i: &u8| *i ^ 0xFF;

    if inp.len() < THRESHOLD {
        out.extend(inp.iter().map(f))
    } else {
        out.par_extend(inp.par_iter().map(f));
    }

    out
}

fn main() {
    let inp: Vec<u8> = (0..255).collect();
    let v = copying_xor(&inp);
    assert!(v.windows(2).all(|w| w[0] > w[1]));
    println!("{v:?}");
}
