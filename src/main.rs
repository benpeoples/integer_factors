use std::env;
use prime_tools;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if(args.len() < 2) {
        println!("Usage: integer_factors number");
        println!("Where number is the number you want to calculate up to.");
        return;
    } 

    let target : u32 = args[1].parse().unwrap();

    for target in 3..target {

    let mut target_set : Vec<u32> = Vec::new();

    for i in 1..(target+1) {
        target_set.push(i);
    }

    // for target in 3..target {
    
    let mut the_set = HashSet::new();

    let mut ctr : usize = 0;

    let mut test : u32 = 1;

    let mut state : usize = 0;

    let prime_list = prime_tools::get_primes_less_than_x(target+1);

    for i in &target_set {
        let primes_with_counts = prime_tools::get_prime_factors_with_counts(*i,&prime_list);
        // println!("{:#?}",primes_with_counts);
        for p in primes_with_counts {
            for i in 0..p.1 {
                the_set.insert(format!("{}_{}",p.0,i));
            }
        }
    }
    // println!("{:#?}",the_set);

    for i in the_set {
        let parts : Vec<&str> = i.split("_").collect();
        let factor : u32 = parts[0].parse().unwrap();
        test *= factor;
    }

    println!("Target: {} Value: {}",target,test);

    // for i in prime_list {
    //     test *= i;
    // }

    // let n = target;

    // test = test * (n - (n-1)%2)/2 * (n - (n-1)%3)/3;

    // println!("Target {} Got: {}",target,test);

    for i in 1..(target+1) {
        if test % i != 0 {
            println!("Not divisible by {}", i);
        }
    }

}

}
