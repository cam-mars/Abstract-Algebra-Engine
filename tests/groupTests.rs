use math_algebra_engine::structures::groups::{Group, Z7Add, ZnAdd, ZpMult, Symmetric, GL};
use nalgebra::SMatrix;
use num_complex::Complex;
use math_algebra_engine::structures::helpers::{generate_cyclic_subgroup, is_subgroup, is_normal, generate_group};

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

mod cyclic_tests {
    use super::*;
    #[test]

    fn test_Zn(){
        let two = ZnAdd::<8>::new(2);
        let sub_gen_two = generate_cyclic_subgroup::<ZnAdd<8>>(&two, 100);

        // println!("The subgroup generated by {} in Z_8 is {:?}", two.value(), sub_gen_two);
    }

    #[test]
    fn test_zp(){
        let two = ZpMult::<7>::new(2);
        let sub_gen_two = generate_cyclic_subgroup::<ZpMult<7>>(&two, 100);

        println!("The subgroup generated by {} in Z_7 is {:?}", two.value(), sub_gen_two);
    }

    #[test]
    fn test_sn(){
        let tau = Symmetric::<5>::new(vec![1,2,0,3,4]).unwrap();
        let sub_gen_tau = generate_cyclic_subgroup::<Symmetric<5>>(&tau, 100);

        println!("The subgroup generated by {:?} in S_5 is {:?}", tau, sub_gen_tau);
    }
}


mod subgroup_test {
    use super::*;
    #[test]
    fn test_cyclic_subgroup(){
        let two = ZnAdd::<8>::new(2);
        let sub_gen_two = generate_cyclic_subgroup::<ZnAdd<8>>(&two, 100).unwrap();
        
        assert!(is_subgroup::<ZnAdd::<8>>(&sub_gen_two))
    }

    #[test]
    fn test_a5(){
        let a5_raw = vec![
            vec![0, 1, 2, 3, 4], vec![0, 1, 3, 4, 2], vec![0, 1, 4, 2, 3], vec![0, 2, 1, 4, 3], 
            vec![0, 2, 3, 1, 4], vec![0, 2, 4, 3, 1], vec![0, 3, 1, 2, 4], vec![0, 3, 2, 4, 1], 
            vec![0, 3, 4, 1, 2], vec![0, 4, 1, 3, 2], vec![0, 4, 2, 1, 3], vec![0, 4, 3, 2, 1], 
            vec![1, 0, 2, 4, 3], vec![1, 0, 3, 2, 4], vec![1, 0, 4, 3, 2], vec![1, 2, 0, 3, 4], 
            vec![1, 2, 3, 4, 0], vec![1, 2, 4, 0, 3], vec![1, 3, 0, 4, 2], vec![1, 3, 2, 0, 4], 
            vec![1, 3, 4, 2, 0], vec![1, 4, 0, 2, 3], vec![1, 4, 2, 3, 0], vec![1, 4, 3, 0, 2], 
            vec![2, 0, 1, 3, 4], vec![2, 0, 3, 4, 1], vec![2, 0, 4, 1, 3], vec![2, 1, 0, 4, 3], 
            vec![2, 1, 3, 0, 4], vec![2, 1, 4, 3, 0], vec![2, 3, 0, 1, 4], vec![2, 3, 1, 4, 0], 
            vec![2, 3, 4, 0, 1], vec![2, 4, 0, 3, 1], vec![2, 4, 1, 0, 3], vec![2, 4, 3, 1, 0], 
            vec![3, 0, 1, 4, 2], vec![3, 0, 2, 1, 4], vec![3, 0, 4, 2, 1], vec![3, 1, 0, 2, 4], 
            vec![3, 1, 2, 4, 0], vec![3, 1, 4, 0, 2], vec![3, 2, 0, 4, 1], vec![3, 2, 1, 0, 4], 
            vec![3, 2, 4, 1, 0], vec![3, 4, 0, 1, 2], vec![3, 4, 1, 2, 0], vec![3, 4, 2, 0, 1], 
            vec![4, 0, 1, 2, 3], vec![4, 0, 2, 3, 1], vec![4, 0, 3, 1, 2], vec![4, 1, 0, 3, 2], 
            vec![4, 1, 2, 0, 3], vec![4, 1, 3, 2, 0], vec![4, 2, 0, 1, 3], vec![4, 2, 1, 3, 0], 
            vec![4, 2, 3, 0, 1], vec![4, 3, 0, 2, 1], vec![4, 3, 1, 0, 2], vec![4, 3, 2, 1, 0]
        ];
        let mut a5 = vec![];
        for elem in a5_raw{
            a5.push(Symmetric::<5>::new(elem).unwrap())
        }

        assert!(is_subgroup::<Symmetric::<5>>(&a5))
    }

    #[test]
    fn not_subgroup_sn(){
        let not_sg_raw = vec![vec![0, 1, 2, 3, 4], vec![0, 1, 3, 4, 2], vec![0, 1, 4, 2, 3], vec![0, 2, 1, 4, 3]];

        let mut not_sg = vec![];

        assert!(!is_subgroup::<Symmetric::<5>>(&not_sg)); //make sure {} is not a sg
        for elem in not_sg_raw{
            not_sg.push(Symmetric::<5>::new(elem).unwrap())
        }

        assert!(!is_subgroup::<Symmetric::<5>>(&not_sg))
    }
}

mod group_generator_zn{
    use super::*;

    #[test]
    fn test_zn_single_coprime(){
        let generator = ZnAdd::<9>::new(2);
        let zn = generate_group::<ZnAdd::<9>>(&vec![generator]);

        let zn_unwrap: Vec<_> = zn.iter().map(|n| n.value()).collect();

        println!("The group generated by {} in Z_9 is {:?}", generator.value(), zn_unwrap)
    }

    #[test]
    fn test_zn_single_not_coprime(){
        let generator = ZnAdd::<9>::new(3);
        let zn = generate_group::<ZnAdd::<9>>(&vec![generator]);

        let zn_unwrap: Vec<_> = zn.iter().map(|n| n.value()).collect();

        println!("The group generated by {} in Z_9 is {:?}", generator.value(), zn_unwrap)
    }

    #[test]
    fn test_zn_two_coprime(){
        let two = ZnAdd::<9>::new(2);
        let three = ZnAdd::<9>::new(3);
        let zn = generate_group::<ZnAdd::<9>>(&vec![two,three]);

        let zn_unwrap: Vec<_> = zn.iter().map(|n| n.value()).collect();

        println!("The group generated by {} and {} in Z_9 is {:?}", two.value(),three.value(), zn_unwrap)
    }

    #[test]
    fn test_zn_two_not_coprime(){
        let two = ZnAdd::<16>::new(2);
        let four = ZnAdd::<16>::new(4);
        let zn = generate_group::<ZnAdd::<16>>(&vec![two,four]);

        let zn_unwrap: Vec<_> = zn.iter().map(|n| n.value()).collect();


        println!("The group generated by {} and {} in Z_16 is {:?}", two.value(),four.value(), zn_unwrap)
    }
}


mod group_generator_zp{
    use super::*;

    #[test]
    fn test_zp_prim_root(){
        let generator = ZpMult::<11>::new(2);
        let zp = generate_group::<ZpMult::<11>>(&vec![generator]);
        let zp_unwrap: Vec<_> = zp.iter().map(|n| n.value()).collect();

        println!("The group generated by {} in Z_11* is {:?}", generator.value(), zp_unwrap)        
    }

    #[test]
    fn test_zp_not_prim_root(){
        let generator = ZpMult::<11>::new(3);
        let zp = generate_group::<ZpMult::<11>>(&vec![generator]);

        let zp_unwrap: Vec<_> = zp.iter().map(|n| n.value()).collect();

        println!("The group generated by {} in Z_11* is {:?}", generator.value(), zp_unwrap)        
    }
}






mod normality_test {
    use super ::*;

    #[test]
    fn test_a5_in_s5(){
        let a5_raw = vec![
            vec![0, 1, 2, 3, 4], vec![0, 1, 3, 4, 2], vec![0, 1, 4, 2, 3], vec![0, 2, 1, 4, 3], 
            vec![0, 2, 3, 1, 4], vec![0, 2, 4, 3, 1], vec![0, 3, 1, 2, 4], vec![0, 3, 2, 4, 1], 
            vec![0, 3, 4, 1, 2], vec![0, 4, 1, 3, 2], vec![0, 4, 2, 1, 3], vec![0, 4, 3, 2, 1], 
            vec![1, 0, 2, 4, 3], vec![1, 0, 3, 2, 4], vec![1, 0, 4, 3, 2], vec![1, 2, 0, 3, 4], 
            vec![1, 2, 3, 4, 0], vec![1, 2, 4, 0, 3], vec![1, 3, 0, 4, 2], vec![1, 3, 2, 0, 4], 
            vec![1, 3, 4, 2, 0], vec![1, 4, 0, 2, 3], vec![1, 4, 2, 3, 0], vec![1, 4, 3, 0, 2], 
            vec![2, 0, 1, 3, 4], vec![2, 0, 3, 4, 1], vec![2, 0, 4, 1, 3], vec![2, 1, 0, 4, 3], 
            vec![2, 1, 3, 0, 4], vec![2, 1, 4, 3, 0], vec![2, 3, 0, 1, 4], vec![2, 3, 1, 4, 0], 
            vec![2, 3, 4, 0, 1], vec![2, 4, 0, 3, 1], vec![2, 4, 1, 0, 3], vec![2, 4, 3, 1, 0], 
            vec![3, 0, 1, 4, 2], vec![3, 0, 2, 1, 4], vec![3, 0, 4, 2, 1], vec![3, 1, 0, 2, 4], 
            vec![3, 1, 2, 4, 0], vec![3, 1, 4, 0, 2], vec![3, 2, 0, 4, 1], vec![3, 2, 1, 0, 4], 
            vec![3, 2, 4, 1, 0], vec![3, 4, 0, 1, 2], vec![3, 4, 1, 2, 0], vec![3, 4, 2, 0, 1], 
            vec![4, 0, 1, 2, 3], vec![4, 0, 2, 3, 1], vec![4, 0, 3, 1, 2], vec![4, 1, 0, 3, 2], 
            vec![4, 1, 2, 0, 3], vec![4, 1, 3, 2, 0], vec![4, 2, 0, 1, 3], vec![4, 2, 1, 3, 0], 
            vec![4, 2, 3, 0, 1], vec![4, 3, 0, 2, 1], vec![4, 3, 1, 0, 2], vec![4, 3, 2, 1, 0]
        ];

        let mut a5 = vec![];
        for elem in a5_raw{
            a5.push(Symmetric::<5>::new(elem).unwrap())
        }

        let transposition = Symmetric::<5>::new(vec![1,0,2,3,4]).unwrap();
        let ncycle = Symmetric::<5>::new(vec![1,2,3,4,0]).unwrap();

        let s5: Vec<_> = generate_group::<Symmetric<5>>(&[transposition, ncycle])
            .into_iter() //use into_iter to take ownership of the vectors
            .collect();

        assert!(is_normal::<Symmetric::<5>>(&s5, &a5));

        let ncycle = Symmetric::<5>::new(vec![1,2,3,4,0]).unwrap();
        let not_normal = generate_cyclic_subgroup::<Symmetric<5>>(&ncycle, 120).unwrap();

        !assert!(is_normal::<Symmetric::<5>>(&s5, &not_normal));
    }

}