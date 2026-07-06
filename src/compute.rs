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

use std::ops::{Add, Sub, Mul, Div, AddAssign};

pub struct ComplexNum {
    real: f64,
    i: f64,
}

// Impl Rem, Neg, Clone, Copy, Default, Debug, Display, Sum, Product,
// Send, Sync, Unpin, UnwindSafe, RefUnwindSafe, AddAssign, Sized, From<i32>,
// From<f32>
impl Add for ComplexNum {
    type Output = ComplexNum;

    fn add(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: self.real + other.real,
            i: self.i + other.i,
        }
    }
}

impl Sub for ComplexNum {
    type Output = ComplexNum;

    fn sub(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: self.real - other.real,
            i: self.i - other.i,
        }
    }
}

impl Mul for ComplexNum {
    type Output = ComplexNum;

    fn mul(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: (self.real * other.real) - (self.i * other.i),
            i: (self.real * other.i) + (self.i * other.real),
        }
    }
}

impl Div for ComplexNum {
    type Output = ComplexNum;

    fn div(self, other: ComplexNum) -> ComplexNum {
        ComplexNum {
            real: ((self.real * other.real) + (self.i * other.i)) / 
            ((other.real * other.real) + (other.i * other.i)),
            i: ((self.i * other.real) - (self.real * other.i)) /
            ((other.real * other.real) + (other.i * other.i)),
        }
    }
}

pub trait Number:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sized
    + Copy
    + From<i32>
    + From<f32>
    + AddAssign
    + std::fmt::Display
{}

impl<T> Number for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sized
        + Copy
        + From<i32>
        + From<f32>
        + AddAssign
        + std::fmt::Display
{}

pub struct Point<T: Number> {
    p: Vec<T>,
    dimension: usize,
}

pub struct Element {
    i: usize,
    j: usize,
}

pub struct Mtx<T: Number> {
    elem: Vec<T>,
    // shape
    m: usize,
    n: usize,
}

// Tensor struct
pub struct Tns<T: Number> {
    elem: Vec<T>,
    rank: Vec<usize>,
}

pub struct FuncBody {
    func: String,
}

// Numerical function of x-y data
pub struct FuncNum<T: Number> {
    x: Vec<T>,
    y: Vec<T>,
    len: usize,
}

pub struct FuncCont<F>
    where
        F: Fn(f64) -> f64,
    {
        closure: F,
        body: FuncBody,
    }

// impl<F: Fn(f64)> FuncCont<F> {
//     pub fn new(func: FuncBody) -> Result<Self> {
//         let closure = func_continuous(func);
//         // Test if closure return NAN from 1 to see if failed
//         Ok(Self {closure, func})
//     }
// }

// Make this form a real function closure from the string
pub fn func_continuous(func: FuncBody) -> impl Fn(f64) -> f64 {
    move |x| x * x
}

impl<T: Number> Mtx<T> {
    pub fn new(m: usize, n: usize) -> Result<Self> {
        // let a = Vec::<f64>::with_capacity(shape[0] * shape[1])
        let mut elem = Vec::<T>::new();
        elem.try_reserve(m * n)?;
        Ok(Self {elem, m, n})
    }

    fn from_vec(vector: Vec<T>, m: usize, n: usize) -> Result<Self> {
        Ok(Self {elem: vector, m, n})
    }

    // Push does not affect shape
    fn push(&mut self, e: T) {
        self.elem.push(e);
    }

    fn concatonate(&mut self, a: Mtx<T>, axis: usize) -> Result<Self> {
        todo!()
    }

    fn echelon_form(&mut self) -> Result<Self> {
        todo!()
    }

    fn derivative(&mut self) -> Result<Self> {
        todo!()
    }

    fn determinant(&self) -> Result<T> {
        todo!()
    }
}

// Determines |u><v|
fn outer_product<T: Number>(
    u: &Vec<T>,
    v: &Vec<T>) -> Result<Mtx<T>> {
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
fn outer_product_elem<T: Number>(
    u: &Vec<T>,
    v: &Vec<T>,
    e: &Element) -> Result<T> {
    if e.i > u.len() || e.j > v.len() {
        return Err(Error::MtxBounds);
    }

    Ok(u[e.i] * v[e.j])
}

// Determines <u|v>
fn inner_product<T: Number>(u: &Vec<T>, v: &Vec<T>) -> Result<T> {
    if u.len() != v.len() {
        return Err(Error::VectorSize);
    }
    let mut a: T = T::from(0);

    for x in 0..u.len() {
        a += u[x] * v[x];
    }

    Ok(a)
}

// Explicit kronecker product A{OX}B
fn knonecker_product<T: Number>(a: &Mtx<T>, b: &Mtx<T>) -> Result<Mtx<T>> {
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
fn knonecker_product_elem<T: Number>(
    a: &Mtx<T>,
    b: &Mtx<T>,
    e: &Element) -> Result<T> {

    Ok(T::from(0))
}

pub fn printable_object<T: Number>(mtx: &Mtx<T>) -> Result<Vec<String>> {
    let mut printable: Vec<String> = Vec::new();
    let mut i: usize = 0;

    for m in 0..mtx.m {
        for n in 0..mtx.n {
            printable.push(
                mtx.elem
                    .get(i)
                    .ok_or(Error::MtxBounds)?
                    .to_string());
            i += 1;
        }
        printable.push(String::from("\n"));
    }

    Ok(printable)
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
