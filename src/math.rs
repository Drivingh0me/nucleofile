use anyhow::bail

// pub type Result<T> = std::result::Result<T, MathErr>;
// Use From <T> for ? operator.
// Needs std::error::Error, Debug, and Display traits.

pub struct Mtx {
    elem: Vec<f64>,
    size: [u64; 2],
}

impl Mtx {
    fn new(elem: Vec<f64>, size: [u64; 2]) -> Self {
        Self {elem, size}
    }

    fn determinant(self) -> f64 {
        todo!
    }
}

fn outer_product(u: Vec<f64>, v: vec<f64>) -> anyhow::Result<Mtx> {
    if u.len != v.len {
        bail!("Vectors are not same length u:{} v:{}", u.len, v.len))
    }
    for x in e.iter() {
        todo!
    }
}
