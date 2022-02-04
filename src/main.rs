use std::env;
use prime_tools;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if(args.len() < 2) {
        println!("Usage: integer_factors number");
        println!("Where number is the number you want to calculate up to.");
        return;
    } 

    let target : u32 = args[1].parse().unwrap();

    let mut target_set : Vec<u32> = Vec::new();

    for i in 1..(target+1) {
        target_set.push(i);
    }

    let mut ctr : usize = 0;

    let mut test : u32 = 1;

    let mut state : usize = 0;

    let prime_list = prime_tools::get_primes_less_than_x(target+1);

    // for i in target_set {
    //     println!("{:#?}",prime_tools::get_prime_factors_with_counts(i.try_into().unwrap(),&prime_list));
    // }

    for i in prime_list {
        println!("Adding {}", i);
        test *= i;
    }

    test = test * (target - target%2)/2 * (target - target%3)/3;

    println!("Got: {}",test);

    for i in 1..(target+1) {
        if test % i != 0 {
            println!("Not divisible by {}", i);
        }

    }

}
