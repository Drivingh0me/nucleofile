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
    size: [u64; 2],
}

impl Mtx {
    // Remove elem from constructor
    fn new(elem: Vec<f64>, size: [u64; 2]) -> Self {
        Self {elem, size}
    }

    fn push(e: f64) -> Self {
        todo!()
    }

    fn determinant(self) -> Result<f64> {
        todo!()
    }
}

// Determines |u><v|
fn outer_product(u: Vec<f64>, v: Vec<f64>) -> Result<Mtx> {
    // Initialize the returned Mtx a
    let a = Mtx::new([u.len(), v.len()]);
    for x in u.iter() {
        a.push(u.x * v.x);
    }
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
    Ok(Mtx{
        elem: vec![0, 1, 2, 3],
        size: [2, 2],
    })
}
