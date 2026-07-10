extern crate nalgebra as na;
use math_algebra_engine::structures::groups::{Group, ZnAdd, ZpMult, Symmetric, GL};
use std::io::{self, Write};
use num_complex::Complex;
use na::{SMatrix, SVector};



fn main() {
    println!("--- Testing our Algebra Engine ---");
    
    loop {
        println!("\nSelect group: A for Zn(additive), B for Zp(multiplicative), C for Sn, D for GL(n) [Any other key to exit]");

        let mut group_input = String::new();
        io::stdin()
            .read_line(&mut group_input)
            .expect("Failed to read line");

        let group_choice = group_input.trim().chars().next().unwrap_or(' ');

        match group_choice {
            'A' => {
                println!("Select modulus: 3, 4, 5, 6, 7");
                
                let mut mod_input = String::new();
                io::stdin()
                    .read_line(&mut mod_input)
                    .expect("Failed to read line");

                let mod_choice: u32 = match mod_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number choice.");
                        continue;
                    }
                };

                let (val1, val2) = get_two_elements();

                match mod_choice {
                    3 => {
                        let elem1 = ZnAdd::<3>::new(val1);
                        let elem2 = ZnAdd::<3>::new(val2);
                        let result = elem1.combine(&elem2);
                        println!("Result of combination in ZnAdd<3>: {}", result.value());
                    }
                    4 => {
                        let elem1 = ZnAdd::<4>::new(val1);
                        let elem2 = ZnAdd::<4>::new(val2);
                        let result = elem1.combine(&elem2);
                        println!("Result of combination in ZnAdd<4>: {}", result.value());
                    }
                    5 => {
                        let elem1 = ZnAdd::<5>::new(val1);
                        let elem2 = ZnAdd::<5>::new(val2);
                        let result = elem1.combine(&elem2);
                        println!("Result of combination in ZnAdd<5>: {}", result.value());
                    }
                    6 => {
                        let elem1 = ZnAdd::<6>::new(val1);
                        let elem2 = ZnAdd::<6>::new(val2);
                        let result = elem1.combine(&elem2);
                        println!("Result of combination in ZnAdd<6>: {}", result.value());
                    }
                    7 => {
                        let elem1 = ZnAdd::<7>::new(val1);
                        let elem2 = ZnAdd::<7>::new(val2);
                        let result = elem1.combine(&elem2);
                        println!("Result of combination in ZnAdd<7>: {}", result.value());
                    }
                    _ => println!("This interactive engine only supports moduli 3, 4, 5, 6, or 7 right now!"),
                }
            }
            'B' => {
                println!("Select prime: 2, 5, 7");
                
                let mut prime_input = String::new();
                io::stdin()
                    .read_line(&mut prime_input)
                    .expect("Failed to read line");

                let prime_choice: u32 = match prime_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number choice.");
                        continue;
                    }
                };

                println!("Select operation: 1 to Combine 2 elements, 2 to Find Inverse of an element");
                let mut op_input = String::new();
                io::stdin().read_line(&mut op_input).expect("Failed to read line");
                let op_choice = op_input.trim();

                match op_choice {
                    "1" => {
                        let (val1, val2) = get_two_elements();

                        match prime_choice {
                            2 => {
                                let elem1 = ZpMult::<2>::new(val1);
                                let elem2 = ZpMult::<2>::new(val2);
                                let result = elem1.combine(&elem2);
                                println!("Result: {}", result.value());
                            }
                            5 => {
                                let elem1 = ZpMult::<5>::new(val1);
                                let elem2 = ZpMult::<5>::new(val2);
                                let result = elem1.combine(&elem2);
                                println!("Result: {}", result.value());
                            }
                            7 => {
                                let elem1 = ZpMult::<7>::new(val1);
                                let elem2 = ZpMult::<7>::new(val2);
                                let result = elem1.combine(&elem2);
                                println!("Result: {}", result.value());
                            }
                            _ => println!("Unsupported prime for compilation gateway."),
                        }
                    }
                    "2" => {
                        println!("Enter element to invert:");
                        let mut element_input = String::new();
                        io::stdin().read_line(&mut element_input).expect("Failed to read line");
                        let val: i32 = element_input.trim().parse().unwrap_or(0);

                        match prime_choice {
                            2 => {
                                let element = ZpMult::<2>::new(val);
                                println!("The inverse of {} in ZpMult<2> is: {}", element.value(), element.inverse().value());
                            }
                            5 => {
                                let element = ZpMult::<5>::new(val);
                                println!("The inverse of {} in ZpMult<5> is: {}", element.value(), element.inverse().value());
                            }
                            7 => {
                                let element = ZpMult::<7>::new(val);
                                println!("The inverse of {} in ZpMult<7> is: {}", element.value(), element.inverse().value());
                            }
                            _ => println!("Unsupported prime for compilation gateway."),
                        }
                    }
                    _ => println!("Invalid operation selection."),
                }
            }

            'C' => {
                println!("Select n: 3, 4, 5, 6, 7");
                
                let mut n_input = String::new();
                io::stdin()
                    .read_line(&mut n_input)
                    .expect("Failed to read line");

                let n_choice: u32 = match n_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number choice.");
                        continue;
                    }
                };

                println!("Select operation: 1 to Combine 2 elements, 2 to Find Inverse of an element");
                let mut op_input = String::new();
                io::stdin().read_line(&mut op_input).expect("Failed to read line");
                let op_choice = op_input.trim();

                match op_choice {
                    "1" => {
                        println!("Please type your two elements as vectors that contain the image of [012...n-1]");

                        match n_choice {
                            3 => {
                                let (val1, val2) = get_two_elementsSn::<3>();
                                let elem1 = Symmetric::<3>::new(val1).unwrap();
                                let elem2 = Symmetric::<3>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result: {:?}", result.value());
                            }
                            4 => {
                                let (val1, val2) = get_two_elementsSn::<4>();
                                let elem1 = Symmetric::<4>::new(val1).unwrap();
                                let elem2 = Symmetric::<4>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result: {:?}", result.value());
                            }
                            5 => {
                                let (val1, val2) = get_two_elementsSn::<5>();
                                let elem1 = Symmetric::<5>::new(val1).unwrap();
                                let elem2 = Symmetric::<5>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result: {:?}", result.value());
                            }
                            6 => {
                                let (val1, val2) = get_two_elementsSn::<6>();
                                let elem1 = Symmetric::<6>::new(val1).unwrap();
                                let elem2 = Symmetric::<6>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result: {:?}", result.value());
                            }
                            7 => {
                                let (val1, val2) = get_two_elementsSn::<7>();
                                let elem1 = Symmetric::<7>::new(val1).unwrap();
                                let elem2 = Symmetric::<7>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result: {:?}", result.value());
                            }
                            _ => println!("Unsupported prime for compilation gateway."),
                        }
                    }
                    "2" => {
                        println!("Enter element to invert:");
                        

                        match n_choice {
                            3 => {
                                let perm = get_single_permutation::<3>();
                                let element = Symmetric::<3>::new(perm).unwrap();
                                println!("The inverse of {:?} in S3 is: {:?}", element.value(), element.inverse().value());
                            }
                            4 => {
                                let perm = get_single_permutation::<4>();
                                let element = Symmetric::<4>::new(perm).unwrap();
                                println!("The inverse of {:?} in S4 is: {:?}", element.value(), element.inverse().value());
                            }
                            5 => {
                                let perm = get_single_permutation::<5>();
                                let element = Symmetric::<5>::new(perm).unwrap();
                                println!("The inverse of {:?} in S5 is: {:?}", element.value(), element.inverse().value());
                            }
                            6 => {
                                let perm = get_single_permutation::<6>();
                                let element = Symmetric::<6>::new(perm).unwrap();
                                println!("The inverse of {:?} in S6 is: {:?}", element.value(), element.inverse().value());
                            }
                            7 => {
                                let perm = get_single_permutation::<7>();
                                let element = Symmetric::<7>::new(perm).unwrap();
                                println!("The inverse of {:?} in S7 is: {:?}", element.value(), element.inverse().value());
                            }
                            _ => println!("Unsupported prime for compilation gateway."),
                        }
                    }
                    _ => println!("Invalid operation selection."),
                }

            }

            'D' => {
                println!("Select n: 2, 3, 4, 5, 6");
                
                let mut n_input = String::new();
                io::stdin()
                    .read_line(&mut n_input)
                    .expect("Failed to read line");

                let n_choice: u32 = match n_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number choice.");
                        continue;
                    }
                };

                println!("Select operation: 1 to Combine 2 elements, 2 to Find Inverse of an element");
                let mut op_input = String::new();
                io::stdin().read_line(&mut op_input).expect("Failed to read line");
                let op_choice = op_input.trim();

                match op_choice {
                    "1" => {
                        match n_choice {
                            2 => {
                                let (val1, val2) = get_two_matrices::<2>();
                                let elem1 = GL::<2>::new(val1).unwrap();
                                let elem2 = GL::<2>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result:\n{:.3}", result.matrix());
                            }
                            3 => {
                                let (val1, val2) = get_two_matrices::<3>();
                                let elem1 = GL::<3>::new(val1).unwrap();
                                let elem2 = GL::<3>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result:\n{:.3}", result.matrix());
                            }
                            4 => {
                                let (val1, val2) = get_two_matrices::<4>();
                                let elem1 = GL::<4>::new(val1).unwrap();
                                let elem2 = GL::<4>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result:\n{:.3}", result.matrix());
                            }
                            5 => {
                                let (val1, val2) = get_two_matrices::<5>();
                                let elem1 = GL::<5>::new(val1).unwrap();
                                let elem2 = GL::<5>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result:\n{:.3}", result.matrix());
                            }
                            6 => {
                                let (val1, val2) = get_two_matrices::<6>();
                                let elem1 = GL::<6>::new(val1).unwrap();
                                let elem2 = GL::<6>::new(val2).unwrap();
                                let result = elem1.combine(&elem2);
                                println!("Result:\n{:.3}", result.matrix());
                            }
                            _ => println!("Unsupported prime for compilation gateway."),
                        }
                    }
                    "2" => {
                        println!("Enter element to invert:");
                        

                        match n_choice {
                            2 => {
                                let mat = get_single_matrix::<2>();
                                let element = GL::<2>::new(mat).unwrap();
                                println!("The inverse of \n{:.3} in GL2 is: \n{:.3}", element.matrix(), element.inverse().matrix());
                            }
                            3 => {
                                let mat = get_single_matrix::<3>();
                                let element = GL::<3>::new(mat).unwrap();
                                println!("The inverse of \n{:.3} in GL3 is: \n{:.3}", element.matrix(), element.inverse().matrix());
                            }
                            4 => {
                                let mat = get_single_matrix::<4>();
                                let element = GL::<4>::new(mat).unwrap();
                                println!("The inverse of \n{:.3} in GL4 is: \n{:.3}", element.matrix(), element.inverse().matrix());
                            }
                            5 => {
                                let mat = get_single_matrix::<5>();
                                let element = GL::<5>::new(mat).unwrap();
                                println!("The inverse of \n{:.3} in GL5 is: \n{:.3}", element.matrix(), element.inverse().matrix());
                            }
                            6 => {
                                let mat = get_single_matrix::<6>();
                                let element = GL::<6>::new(mat).unwrap();
                                println!("The inverse of \n{:.3} in GL6 is: \n{:.3}", element.matrix(), element.inverse().matrix());
                            }
                            _ => println!("Unsupported prime for compilation gateway."),
                        }
                    }
                    _ => println!("Invalid operation selection."),
                }

            }

            _ => {
                println!("Exiting the Algebra Engine. Goodbye!");
                break;
            }
        }
    }
}

fn get_two_elements() -> (i32, i32) {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter first element:");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let val1: i32 = input1.trim().parse().unwrap_or(0);

    println!("Enter second element:");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let val2: i32 = input2.trim().parse().unwrap_or(0);

   (val1, val2)
}


fn get_single_permutation<const N: usize>() -> Vec<usize> {
    loop {
        print!("Enter permutation for S_{} (space-separated integers): ", N);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse into integers
        let parsed: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.trim_matches(',').parse::<usize>().ok())
            .collect();

        // Quick structural check: must match length N before passing to the constructor
        if parsed.len() != N {
            println!("Error: Input must have exactly {} elements. Try again.", N);
            continue;
        }
        match Symmetric::<N>::new(parsed) {
            Some(elem) => return elem.value().to_vec(),
            None => {
                println!("Error: That is not a valid element of S_{}. Please try again.", N);
            }
        }

    }
}

fn get_single_matrix<const N: usize>() -> SMatrix<Complex<f32>, N, N> 
{
    loop {    
        let mut columns = Vec::with_capacity(N);
        let mut i = 0;
            while i < N{
                let col_vec = loop {
                print!("Enter column {} ({} space-separated complex numbers, e.g. 1.0+0.0i 2.0+0.0i): ", i + 1, N);
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                // Parse the input string into a flat Vec of Complex<f32>
                let parsed: Vec<Complex<f32>> = input
                    .split_whitespace()
                    .filter_map(|s| s.parse::<Complex<f32>>().ok())
                    .collect();

                // Structural check: Ensure they provided exactly N valid complex numbers
                if parsed.len() != N {
                    println!(
                        "Error: Expected exactly {} valid complex numbers, but got {}. Try again.",
                        N,
                        parsed.len()
                    );
                    continue; // Re-prompt for this specific column
                }
                //if success
                break SVector::<Complex<f32>, N>::from_vec(parsed);
                };
            i += 1;
            columns.push(col_vec); 
            }
        
        let usermatrix = SMatrix::<Complex<f32>, N, N>::from_columns(&columns);

        match GL::<N>::new(usermatrix) {
                Some(elem) => {return *elem.matrix()},
                None => {
                    println!("Error: That is not a valid element of GL_{}. Please try again.", N);
                    continue;
                }
            }
        }
}


fn get_two_elementsSn<const N: usize>() -> (Vec<usize>, Vec<usize>) {
    println!("--- Element 1 ---");
    let p1_vec = get_single_permutation::<N>();
    
    println!("--- Element 2 ---");
    let p2_vec = get_single_permutation::<N>();

    (p1_vec, p2_vec)
}

fn get_two_matrices<const N: usize>() -> (SMatrix<Complex<f32>, N, N>, SMatrix<Complex<f32>, N, N>) {
    println!("--- Element 1 ---");
    let m1 = get_single_matrix::<N>();
    
    println!("--- Element 2 ---");
    let m2 = get_single_matrix::<N>();

    (m1, m2)
}