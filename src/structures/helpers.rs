use std::collections::{HashSet, VecDeque};
 use std::hash::Hash;
use std::cmp::PartialEq;
use crate::structures::groups::Group;
// use contains::{Container, In};

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

pub fn generate_cyclic_subgroup<T: Group + Clone + PartialEq>(generator: &T, max_order: usize) -> Option<Vec<T>>{
    let ident = T::identity();
    let mut subgroup = vec![ident.clone()];

    if *generator == ident {
        Some(subgroup)
    }
    else{
        let mut current = generator.clone();
        for _ in 0..max_order{
            subgroup.push(current.clone());
            current = generator.combine(&current);
            if current == ident{
                return Some(subgroup);
            }
        }
        None
    }
}

//note that we have to have the group def itself, this will only give us the elements
//eventually, I want to replace this with the Schreier-Sims algorithm
pub fn generate_group<T: Group  + Clone + Eq + Hash>(generators: &[T]) -> HashSet<T> {
    let mut master_record = HashSet::new(); //this will contain all unique elements
    let mut queue = VecDeque::new(); //elements we still have to check

    master_record.insert(T::identity());
    queue.push_back(T::identity());

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap(); //use a deque here to improve complexity with popping
        for element in generators { //combine the queue with every generator
            let combined = current.combine(element);

            if !master_record.contains(&combined){
                master_record.insert(combined.clone());
                queue.push_back(combined);
            }
        }
    }
    master_record
}




pub fn is_subgroup<T: Group + Clone + PartialEq>(subset: &[T]) -> bool { 
    //since subset is of type T, we already know each elem is in the group by default
    if subset.is_empty() {
        return false;
    }

    for i in subset { //NOTE: THIS IS ONLY FOR FINITE SUBSETS
        for j in subset {
            if !(subset.contains(&(i.combine(j)))){ //only need to check closure
                return false;
            }
        }
    }
    true
}

pub fn is_normal<T: Group + Clone + PartialEq>(G: &[T], N: &[T]) -> bool { 
    if !is_subgroup::<T>(N){
        return false;
    }

    for n in N {
        for g in G{
            let gng_inv = g.combine(&n.combine(&g.inverse()));
            if !(N.contains(&gng_inv)){
                return false;
            }
        }
    }
    true
}