use crate::error::{Error, Result};

// Try allocating large vectors first with Vec::with_capacity
// Use try_reserve() or try_reserve_exact()
// which return Reslut<T, TryReserveError> so can handle failure to alloc.
// Can then use push() and extend and the rest, as well as shrink_to_fit()
// to make the vec.

// Can also consider using Box<[T]> for unchanging length, but all elems
// must be filled. Box is likely better for saving a vec for a long life.

// pub type Result<T> = std::result::Result<T, MathErr>;

// Add a way to do linked lists, dictionaries/hashmaps, unions, units.

// Consider Vec<T> so that any type that has multiplication
// will work. (Mtx<T> {elem: Vec<T>, dim: [u64; 2],}
// Can also restrict type with trait bound and impl block.
// Also consider using uom for units.
// Consider implementing Mul for Mtx (fn mul) and Vec to make easy.

// Maybe this is a bad layout for point. Make more generic?
pub struct Pnt {
    p: [f64; 2],
}

pub struct Elm {
    // i j
    elem: [usize; 2]
}

pub struct Mtx {
    elem: Vec<f64>,
    // shape is mxn, [m, n]
    shape: [usize; 2],
}

// Tensor struct
pub struct Tns<T> {
    elem: Vec<T>,
    rank: Vec<usize>,
}

pub struct Func {
    func_body: String,
}

impl Mtx {
    fn new(shape: [usize; 2]) -> Result<Self> {
        // let a = Vec::<f64>::with_capacity(shape[0] * shape[1])
        let mut elem = Vec::<f64>::new();
        elem.try_reserve(shape[0] * shape[1])?;
        Ok(Self {elem, shape})
    }

    fn from_vec(vector: Vec<f64>, shape: [usize; 2]) -> Result<Self> {
        Ok(Self {elem: vector, shape})
    }

    // Push does not affect shape
    fn push(&mut self, e: f64) {
        self.elem.push(e);
    }

    fn echelon_form(&mut self) -> Result<Self> {
        todo!()
    }

    fn derivative(&mut self) -> Result<Self> {
        todo!()
    }
}

fn determinant(a: Mtx) -> Result<f64> {
    todo!()
}

// Determines |u><v|
fn outer_product(u: Vec<f64>, v: Vec<f64>) -> Result<Mtx> {
    let m: usize = u.len();
    let n: usize = v.len();
    let mxn: usize = m * n;
    let mut a = Mtx::new([m, n])?;

    // Make this a nested for loop to be faster. This can be used for lazy
    // deterination of one elem of outer product.
    for x in 0..mxn {
        // Maybe use .get(x)
        a.push(u[x / m] * v[x % n]);
    }

    Ok(a)
}

// Determines <u|v>
fn inner_product(u: Vec<f64>, v: Vec<f64>) -> Result<f64> {
    if u.len() != v.len() {
        return Err(Error::VectorSize);
    }
    let mut a: f64 = 0.0;
    for x in 0..u.len() {
        a += u[x] * v[x];
    }

    Ok(a)
}

// Explicit kronecker product A{OX}B
fn knonecker_product(a: Mtx, b: Mtx) -> Result<Mtx> {
    let c_shape: [usize; 2] =
        [a.shape[0] * b.shape[0],
        a.shape[1] * b.shape[1]];
    let m: usize = c_shape[0];
    let n: usize = c_shape[1];

    let mut c = Mtx::new(c_shape)?;

    for i in 0..m {
        for j in 0..n {
            c.push(a.elem[i] * b.elem[j]);
        }
    }

    Ok(c)
}

// Lazy calculates (i, j) elem of kronecker product
fn knonecker_product_elem(a: Mtx, b: Mtx, p: Pnt) -> Result<f64> {

    Ok(0.0)
}

// TESTING
#[test]
fn test_inner_product() {
    let a: Vec<f64> = vec![0.0, 3.0, 1.45, 4.0];
    let b: Vec<f64> = vec![0.2, 7.0, 4.0, 5.0];
    let c: f64 = 46.8;
    let tolerance: f64 = c * 0.0001;

    let error = inner_product(a, b).unwrap() - c;
    assert!((error).abs() <= tolerance);
}
