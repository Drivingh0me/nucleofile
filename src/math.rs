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

// Determines |u><v|
fn outer_product(u: vec<f64>, v: vec<f64>) -> anyhow::Result<Mtx> {
    for x in e.iter() {
        todo!
    }
}

// Determines <u|v>
fn inner_product(u: vec<f64>, v: vec<f64>) -> Mtx {
    if u.len != v.len{
        bail!("Vector lengths do not match: u:{} v:{}", u.len, v.len)
    }
    todo!
}
