use math_algebra_engine::structures::groups::{Group, Z7Add, ZnAdd, ZpMult, Symmetric, GL};
use nalgebra::SMatrix;
use num_complex::Complex;

#[cfg(test)]
mod Z7tests {
    use super::*; //this grabs all above code

    #[test]
    fn eq_classes() {
        let four = Z7Add::new(4);
        let three = Z7Add::new(3);
        let eleven = Z7Add::new(11);
        let minus3 = Z7Add::new(-3);
        let minus11 = Z7Add::new(-11);
        assert_eq!(four, eleven);
        assert_eq!(four, minus3);
        assert_eq!(three, minus11);
    }

    #[test]
    fn text_axioms(){
        let four = Z7Add::new(4);
        let five = Z7Add::new(5);
        let two = Z7Add::new(2);
        let zero = Z7Add::identity();
        let zero_num = Z7Add::new(0);

        //closure
        assert_eq!(four.combine(&five), two);
        assert_eq!(four.combine(&five).value(), 2);

        //identity
        assert_eq!(four.combine(&zero), four);
        assert_eq!(zero, zero_num);

        //inverse
        assert_eq!(four.combine(&four.inverse()), zero);
        assert_eq!(four.combine(&four.inverse()).value(), 0);

        //associative
        let eq1 = four.combine(&(five.combine(&two))); // 4+(5+2)
        let eq2 = (four.combine(&five)).combine(&two); //(4+5)+2
        assert_eq!(eq1,eq2);
    }


}

mod ZNAddTests {
    use super::*; //this grabs all above code

    #[test]

    // fn ZnNeZm (){
    //     let _four6 = ZnAdd::<6>::new(4);
    //     let _four7 = ZnAdd::<7>::new(4);
    //     // assert_ne!(four6, four7); this should fail and it did
    // }
    fn eq_classes() { //pick one particular class(mod 9) and just work there
        let four = ZnAdd::<9>::new(4);
        let thirteen = ZnAdd::<9>::new(13);
        let minus5 = ZnAdd::<9>::new(-5);
        let minus14 = ZnAdd::<9>::new(-14);
        assert_eq!(four, thirteen);
        assert_eq!(four, minus5);
        assert_eq!(four, minus14);
    }

    #[test]
    fn text_axioms(){
        let four = ZnAdd::<9>::new(4);
        let six = ZnAdd::<9>::new(6);
        let one = ZnAdd::<9>::new(1);
        let zero = ZnAdd::<9>::identity();
        let zero_num = ZnAdd::<9>::new(0);

        //closure
        assert_eq!(four.combine(&six), one);
        assert_eq!(four.combine(&six).value(), 1);

        //identity
        assert_eq!(four.combine(&zero), four);
        assert_eq!(zero, zero_num);

        //inverse
        assert_eq!(four.combine(&four.inverse()), zero);
        assert_eq!(four.combine(&four.inverse()).value(), 0);

        //associative
        let eq1 = four.combine(&six.combine(&one)); // 4+(6+1)
        let eq2 = (four.combine(&six)).combine(&one); //(4+6)+1
        assert_eq!(eq1,eq2);
    }


}

mod ZPMultTests{
    use super::*; //this grabs all above code

    #[test]

    // fn ZnNeZm (){
    //     let _fourMod7 = ZpMult::<7>::new(4);
    //     let _fourMod11 = ZpMult::<11>::new(4);
    //     // assert_ne!(fourMod7, fourMod11); //this should fail and it did
    // }
    fn eq_classes() { //pick one particular class(mod 9) and just work there
        let four = ZpMult::<7>::new(4);
        let eleven = ZpMult::<7>::new(11);
        let minus3 = ZpMult::<7>::new(-3);
        let minus10 = ZpMult::<7>::new(-10);
        assert_eq!(four, eleven);
        assert_eq!(four, minus3);
        assert_eq!(four, minus10);
    }

    #[test]
    fn text_axioms(){
        let four = ZpMult::<7>::new(4);
        let six = ZpMult::<7>::new(6);
        let two = ZpMult::<7>::new(2);
        let one = ZpMult::<7>::identity();
        let one_num = ZpMult::<7>::new(1);
        let zero = ZpMult::<7>::new(0);

        // let fifteen20 = ZpMult::<20>::new(15); //bc g(15,20) != 1(and bc 20 not prime), we expect this to fail

        //0 mult
        assert_eq!(four.combine(&zero).value(), 0);
        
        //closure
        assert_eq!(four.combine(&six).value(), 3);

        //identity
        assert_eq!(four.combine(&one), four);
        assert_eq!(one_num, one);

        //inverse
        assert_eq!(four.combine(&four.inverse()), one);
        assert_eq!(four.combine(&four.inverse()).value(), 1); 

        // assert_ne!(fifteen20.combine(&fifteen20.inverse()).value(), 1);

        //associative
        let eq1 = four.combine(&(six.combine(&two))); // 4*(6*2)
        let eq2 = (four.combine(&six)).combine(&two); //(4*6)*2
        assert_eq!(eq1,eq2);
    }

}

mod SnTests{
    use super::*; //this grabs all above code

    #[test]

    // fn SnNeSm (){
    //     let S3 = Symmetric::<3>::new([0,1,2].to_vec());
    //     let S4 = Symmetric::<4>::new([0,1,2,3].to_vec());
    //     assert_ne!(S3, S4); //this should fail and it did
    // }
    
    fn test_constructor(){
        let too_short = Symmetric::<5>::new(vec![0, 1, 4, 3]);
        let not_injective = Symmetric::<5>::new(vec![0, 1, 4, 3, 3]);
        let valid_fn = Symmetric::<5>::new(vec![0, 1, 4, 3, 2]);
        
        assert!(too_short.is_none());
        assert!(not_injective.is_none());
        assert!(valid_fn.is_some());
    }

    #[test]
    fn text_axioms(){ //work in S5 for now
        let f2 = Symmetric::<5>::new(vec![0,1,4,3,2]).unwrap();
        let f1 = Symmetric::<5>::new(vec![3,4,1,0,2]).unwrap();
        let f2of1 = Symmetric::<5>::new(vec![3,2,1,0,4]).unwrap();
        let f3 = Symmetric::<5>::new(vec![1,0,2,4,3]).unwrap();
        let ident = Symmetric::<5>::identity();
        let id_fn = Symmetric::<5>::new(vec![0,1,2,3,4]).unwrap();

        
        //composition
        assert_eq!(f2.combine(&f1), f2of1);

        //identity
        assert_eq!(f1.combine(&ident), f1);
        assert_eq!(ident.combine(&f1), f1);
        assert_eq!(ident, id_fn);

        //inverse
        assert_eq!(f1.combine(&f1.inverse()), ident);

        //associative
        let eq1 = f1.combine(&(f2.combine(&f3))); // 4*(6*2)
        let eq2 = (f1.combine(&f2)).combine(&f3); //(4*6)*2
        assert_eq!(eq1,eq2);
        assert_ne!(f2of1, f3);
    }
}

mod GLnTests {
    use super::*;


    // A quick shorthand helper to keep matrix definitions readable
    fn c(re: f32, im: f32) -> Complex<f32> {
        Complex::new(re, im)
    }

    #[test]
    fn test_gl_constructor() {
        // 1. A valid, invertible 2x2 complex matrix
        let valid_matrix = SMatrix::<Complex<f32>, 2, 2>::from_vec(vec![
            c(1.0, 0.0), c(0.0, 1.0),
            c(0.0, 0.0), c(1.0, 0.0),
        ]);
        let gl_elem = GL::<2>::new(valid_matrix);
        assert!(gl_elem.is_some());

        // 2. An invalid, non-invertible 2x2 complex matrix (all zeros)
        let zero_matrix = SMatrix::<Complex<f32>, 2, 2>::zeros();
        let bad_elem = GL::<2>::new(zero_matrix);
        assert!(bad_elem.is_none());
    }

    #[test]
    fn test_gl_identity_axiom() {
        let ident = GL::<2>::identity();
        
        let a = GL::<2>::new(SMatrix::<Complex<f32>, 2, 2>::from_vec(vec![
            c(2.0, 1.0), c(0.0, -1.0),
            c(1.0, 3.0), c(4.0, 0.0),
        ])).unwrap();

        // A * I = A and I * A = A
        assert_eq!(a.combine(&ident), a);
        assert_eq!(ident.combine(&a), a);
    }

    #[test]
    fn test_gl_inverse_axiom() {
        let a = GL::<2>::new(SMatrix::<Complex<f32>, 2, 2>::from_vec(vec![
            c(1.0, 1.0), c(0.0, 2.0),
            c(-1.0, 0.0), c(3.0, 1.0),
        ])).unwrap();

        let a_inv = a.inverse();
        let ident = GL::<2>::identity();

        // A * A^-1 = I
        let product = a.combine(&a_inv);

        // Floats can have tiny rounding errors, so we check element-wise closeness
        for i in 0..2 {
            for j in 0..2 {
                let diff = (product.matrix()[(i, j)] - ident.matrix()[(i, j)]).norm();
                assert!(diff < 1e-5, "Elements at ({}, {}) are not close enough to identity", i, j);
            }
        }
    }

    #[test]
    fn test_gl_associativity() {
        let m1 = GL::<2>::new(SMatrix::<Complex<f32>, 2, 2>::from_vec(vec![
            c(1.0, 0.0), c(2.0, 0.0),
            c(0.0, 1.0), c(1.0, 1.0)
        ])).unwrap();
        
        let m2 = GL::<2>::new(SMatrix::<Complex<f32>, 2, 2>::from_vec(vec![
            c(0.0, 1.0), c(1.0, 0.0),
            c(1.0, 1.0), c(0.0, 0.0)
        ])).unwrap();
        
        let m3 = GL::<2>::new(SMatrix::<Complex<f32>, 2, 2>::from_vec(vec![
            c(2.0, -1.0), c(0.0, 0.0),
            c(1.0, 0.0),  c(3.0, 2.0)
        ])).unwrap();

        // (M1 * M2) * M3
        let lhs = m1.combine(&m2).combine(&m3);
        // M1 * (M2 * M3)
        let rhs = m1.combine(&m2.combine(&m3));

        // Structural check for equality with a tolerance bound
        for i in 0..2 {
            for j in 0..2 {
                let diff = (lhs.matrix()[(i, j)] - rhs.matrix()[(i, j)]).norm();
                assert!(diff < 1e-5);
            }
        }
    }
}