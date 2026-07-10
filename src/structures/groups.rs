extern crate nalgebra as na;
use num_complex::Complex;
use na::{DMatrix, SMatrix, Const, base::dimension::DimMin};

use super::helpers::{extended_euclidean, perm_valid};
// use primes::is_prime;

pub trait Group: Sized { // we want all group elems to have a known size at compile time
    fn combine(&self, rhs: &Self) -> Self; //capitalized Self bc its a type, not the thing itself
    fn identity() -> Self; //identity is defined by the group, no param necessary
    fn inverse(&self) -> Self; //inverse depends on element
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Z7Add {
    value: u32 
}

impl Z7Add {
    pub fn new(val: i32) -> Self {
        let rem = ((val % 7) + 7) % 7; //for negative nums
        Self { value: rem as u32}
    }

    pub fn value(&self) -> u32 { //need self in param to say this is for an instance of Z7, not the group itself
        self.value
    }
}


impl Group for Z7Add{
    fn combine(&self, rhs: &Self) -> Self{
        Self::new(((self.value + rhs.value) % 7).try_into().unwrap()) //last bit is for conversion from u32 to i32
    }

    fn identity() -> Self {
        Self{value: 0}
    }

    fn inverse(&self) -> Self {
        Self::new(((7 - self.value) % 7).try_into().unwrap())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZnAdd<const N: u32> { // N is determined at compile time to ensure that Zn != Zm for n != m
    value: i32
}

impl <const N: u32> ZnAdd<N> {
    pub fn new(val: i32) -> Self {
        let n_i32 = N as i32;

        let rem = ((val % n_i32) + n_i32) % n_i32; //for negative nums
        Self { value: rem as i32 }
    }

    pub fn value(&self) -> i32 { //need self in param to say this is for an instance of Z7, not the group itself
        self.value
    }
}

impl <const N: u32> Group for ZnAdd<N>{
    fn combine(&self, rhs: &Self) -> Self{
        let n_i32 = N as i32;
        Self::new((self.value + rhs.value) % n_i32)
    }

    fn identity() -> Self {
        Self{value: 0}
    }

    fn inverse(&self) -> Self {
        let n_i32 = N as i32;
        Self::new(n_i32 - self.value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZpMult<const P: u32> { // must make sure p prime
    value: i32
}

impl <const P: u32> ZpMult<P> {
    pub fn new(val: i32) -> Self {
        if !primes::is_prime(P as u64) { 
            //eventually, I wanna make it so you can make the mult group out of Zn where n not nec prime
            panic!("Compile-time modulus P={} is not a valid prime number for ZpMult!", P);
        }

        let p_i32 = P as i32;
        let rem = ((val % p_i32) + p_i32) % p_i32; //for negative nums
        Self { value: rem as i32 }
    }

    pub fn value(&self) -> i32 { 
        self.value
    }
}

impl <const P: u32> Group for ZpMult<P>{
    fn combine(&self, rhs: &Self) -> Self{
        let p_i32 = P as i32;
        Self::new((self.value * rhs.value) % p_i32)
    }

    fn identity() -> Self {
        Self{value: 1}
    }

    fn inverse(&self) -> Self {
        let p_i32 = P as i32;
        let (_g, _r, s) = extended_euclidean(&p_i32, &self.value);
        Self::new(s % p_i32)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symmetric<const N: usize> {
    mapping: Vec<usize>, //index is input, value is output
}

impl <const N: usize> Symmetric<N> {
    pub fn new(v: Vec<usize>) -> Option<Self> { //we're ok losing the og vec
        if perm_valid(&v, N){
            Some(Self{mapping: v}) //v will be the entire permutation
        }
        else{
            None
        }
    }
    
    pub fn value(&self) -> &Vec<usize> { //return just the vector slice in memory
        &self.mapping
    }
}

impl <const N: usize> Group for Symmetric<N>{
    fn combine(&self, rhs: &Self) -> Self{
        let v1 = &rhs.mapping;
        let v2 = &self.mapping;

        //go thru [0,1,..,N] map i to v2(v1(i))
        let v = (0..N).map(|i| v2[v1[i]]).collect(); 
        Self{mapping : v}
    }

    fn identity() -> Self {
        let e: Vec<usize> = (0..N).collect();
        Self{mapping: e}
    }

    fn inverse(&self) -> Self {
        let v = &self.mapping;
        let mut v_inverse = [0; N];

        //v.enumerate returns a tuple where first elem is index, second is reference to element
        for (i, &x) in v.iter().enumerate(){ 
            v_inverse[x] = i
        }

        Self{mapping: v_inverse.to_vec()}
    }
}

#[derive(Debug, Clone, PartialEq)] //C doesnt use Eq, so we dropped it
pub struct GL<const N: usize> 
{
    matrix: SMatrix<Complex<f32>, N, N>, //doing it over C
}

impl <const N: usize> GL<N> 
    {
    pub fn new(m: SMatrix<Complex<f32>, N, N>) -> Option<Self>{
        let check_matrix = DMatrix::from_column_slice(N, N, m.as_slice()); //make dynamic matrix to check invertibility

        if check_matrix.lu().is_invertible() {
            Some(GL { matrix: m })
        } else {
            None
        }
    }

    pub fn matrix(&self) -> &SMatrix<Complex<f32>, N, N> {
        &self.matrix
    }
}

impl <const N: usize> Group for GL<N>{
    fn combine(&self, rhs: &Self) -> Self{
        Self{matrix: self.matrix * rhs.matrix,}
    }

    fn identity() -> Self{ //just make a new matrix and call the ident constructor from nalgebra
        Self{matrix: SMatrix::<Complex<f32>, N, N>::identity(),}
    }

    fn inverse(&self) -> Self{
        Self{matrix: self.matrix.try_inverse().unwrap()} //should be the inverse from nalg(?) 
    }
}