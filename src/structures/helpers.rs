use std::collections::HashSet;

pub fn extended_euclidean(a: &i32, b: &i32) -> (i32, i32, i32) {
    extended_euclidean_helper(a,b)
}

pub fn extended_euclidean_helper(a: &i32, b: &i32) -> (i32, i32, i32){ 
    //want to return (g, x, y) where g = xa+yb
    if *b == 0 { //base case where a = g
        (*a, 1, 0) 
    }
        
    else {
        let rem = *a % *b; 
        let (g, x1, y1) = extended_euclidean_helper(b, &rem); //go down
        (g, y1, x1 - ((*a)/(*b)) * y1) //come back up(calc in notes)
    }
}

pub fn gcd(n: i32, m: i32) -> i32 {
    if m == 0{
        n
    }
    else {
        gcd(m, n % m)
    }
}

pub fn perm_valid(v: &Vec<usize>, n: usize) -> bool {
    if v.len() != n {
        false
    } 
    else{
        if !v.iter().all(|&x| x < n) {
            return false;
        }
        //use copied to take the refs in v and turn them into actual integers
        let unique_elems: HashSet<usize> = v.iter().copied().collect();

        //the hashset won't add duplicate elems to itself
        unique_elems.len() == n
    }
}