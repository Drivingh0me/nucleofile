use crate::error::{Error, Result};

// Try allocating large vectors first with Vec::with_capacity
// Use try_reserve() or try_reserve_exact()
// which return Reslut<T, TryReserveError> so can handle failure to alloc.
// Can then use push() and extend and the rest, as well as shrink_to_fit()
// to make the vec.

// Can also consider using Box<[T]> for unchanging length, but all elems
// must be filled. Box is likely better for saving a vec for a long life.

// Add a way to do linked lists, dictionaries/hashmaps, unions, units.

// Consider Vec<T> so that any type that has multiplication
// will work. (Mtx<T> {elem: Vec<T>, dim: [u64; 2],}
// Can also restrict type with trait bound and impl block.
// Also consider using uom for units.
// Consider implementing Mul for Mtx (fn mul) and Vec to make easy.

use std::ops::{Add, Sub, Mul, Div}

trait NumberType {}

impl NumberType for i32 {}
impl NumberType for i64 {}
impl NumberType for f32 {}
impl NumberType for f64 {}
impl NumberType for ComplexNum {}

pub struct ComplexNum {
    real: f64,
    i: f64,
}

// Do Not need these to work with all NumberTypes, just cast into complex
// before operation
impl Add for ComplexNum {
    type Output = ComplexNum;

    fn add(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: self.real - other.real,
            i: self.i - other.i,
        }
    }
}

impl Sub for ComplexNum {
    type Output = ComplexNum;

    fn add(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: self.real + other.real,
            i: self.i + other.i,
        }
    }
}

impl Mul for ComplexNum {
    type Output = ComplexNum;

    fn add(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: self.real + other.real,
            i: self.i + other.i,
        }
    }
}

impl Div for ComplexNum {
    type Output = ComplexNum;

    fn add(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: self.real + other.real,
            i: self.i + other.i,
        }
    }
}
// -------------------------------------------

pub struct Point<T: NumberType> {
    p: Vec<T>,
    dimension: usize,
}

pub struct Element {
    i: usize,
    j: usize,
}

pub struct Mtx<T: NumberType> {
    elem: Vec<T>,
    // shape
    m: usize,
    n: usize,
}

// Tensor struct
pub struct Tns<T: NumberType> {
    elem: Vec<T>,
    rank: Vec<usize>,
}

pub struct FuncBody {
    func: String,
}

// Numerical function of x-y data
pub struct FuncNum {
    x: Vec<f64>,
    y: Vec<f64>,
    len: i32,
}

// Make this form a real function closure from the string
pub fn Func_continuous(func: FuncBody) -> impl Fn(f64) -> f64 {
    move |x| x * x
}

impl Mtx {
    fn new(m: usize, n: usize) -> Result<Self> {
        // let a = Vec::<f64>::with_capacity(shape[0] * shape[1])
        let mut elem = Vec::<f64>::new();
        elem.try_reserve(m * n)?;
        Ok(Self {elem, m, n})
    }

    fn from_vec(vector: Vec<f64>, m: usize, n: usize) -> Result<Self> {
        Ok(Self {elem: vector, m, n})
    }

    // Push does not affect shape
    fn push(&mut self, e: f64) {
        self.elem.push(e);
    }

    fn concatonate(&mut self, a: Mtx, axis: usize) -> Result<Self> {
        todo!()
    }

    fn echelon_form(&mut self) -> Result<Self> {
        todo!()
    }

    fn derivative(&mut self) -> Result<Self> {
        todo!()
    }

    fn determinant(&self) -> Result<f64> {
        todo!()
    }
}

// Determines |u><v|
fn outer_product(u: Vec<f64>, v: Vec<f64>) -> Result<Mtx> {
    let m: usize = u.len();
    let n: usize = v.len();
    let mut a = Mtx::new(m, n)?;

    for x in 0..m {
        for y in 0..n {
            a.push(u[x] * v[y]);
        }
    }

    Ok(a)
}

// Finds single element of |u><v|
fn outer_product_elem(u: Vec<f64>, v: Vec<f64>, e: Element) -> Result<f64> {
    if e.i > u.len() || e.j > v.len() {
        return Err(Error::MtxBounds);
    }

    Ok(u[e.i] * v[e.j])
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
    // let c_shape: [usize; 2] =
    //     [a.shape[0] * b.shape[0],
    //     a.shape[1] * b.shape[1]];
    let m: usize = a.m * b.m;
    let n: usize = a.n * b.n;

    let mut c = Mtx::new(m, n)?;

    for i in 0..m {
        for j in 0..n {
            c.push(a.elem[i] * b.elem[j]);
        }
    }

    Ok(c)
}

// Lazy calculates (i, j) elem of kronecker product
fn knonecker_product_elem(a: Mtx, b: Mtx, e: Element) -> Result<f64> {

    Ok(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_product() {
        let a: Vec<f64> = vec![0.0, 3.0, 1.45, 4.0];
        let b: Vec<f64> = vec![0.2, 7.0, 4.0, 5.0];
        let c: f64 = 46.8;
        let tolerance: f64 = c * 0.0001;

        let error = inner_product(a, b).unwrap() - c;
        assert!((error).abs() <= tolerance);
    }
}
