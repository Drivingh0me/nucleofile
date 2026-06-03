use crate::error::{Error, Result};

// Try allocating large vectors first with Vec::with_capacity
// Instead of .push(), use try_reserve() or try_reserve_exact()
// which return Reslut<T, TryReserveError> so can handle failure to alloc.
// Can then use push() and extend and the rest, as well as shrink_to_fit()
// to make the vec.

// Can also consider using Box<[T]> for unchanging length, but all elems
// must be filled. Box is likely better for saving a vec for a long life.

// pub type Result<T> = std::result::Result<T, MathErr>;
// Use From <T> for ? operator.
// Needs std::error::Error, Debug, and Display traits.

// Add a way to do linked lists, dictionaries/hashmaps, unions, units.

pub struct Mtx {
    elem: Vec<f64>,
    // shape is mxn, [m, n]
    shape: [u64; 2],
}

impl Mtx {
    fn new(shape: [u64; 2]) -> Result<Self> {
        // let a = Vec::<f64>::with_capacity(shape[0] * shape[1])
        let elem: Vec::<f64>::new();
        elem.try_reserve()?;
        Self {elem, shape}
    }

    fn from_vec(elem: Vec<f64>, shape: [u64; 2]) -> Self {
        Self {elem, size}
    }

    fn push(self, e: f64) -> Self {
        todo!()
    }

    fn determinant(self) -> Result<f64> {
        todo!()
    }
}

// Determines |u><v|
fn outer_product(u: Vec<f64>, v: Vec<f64>) -> Result<Mtx> {
    let m: u32 = u.len();
    let n: u32 = v.len();
    let mxn: u64 = m * n;
    let mut a = Mtx::new([m, n])?;

    for x in 0..mxn {
        // Maybe use .get(x)
        a.push(u[x / m] * v[x % n]);
    }

    Ok(a)
}

// Determines <u|v>
fn inner_product(u: Vec<f64>, v: Vec<f64>) -> Result<f64> {
    if u.len() != v.len() {
        return Err(Error::x{});
    }

    Ok(0.0)
}

// Kronecker product A{OX}B
fn knonecker_product(a: Mtx, b: Mtx) -> Result<Mtx> {
    let mut c = Mtx::from_vec(
        vec![0, 1, 2, 3], [2, 2],
    );

    Ok(c)
}
