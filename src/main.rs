use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    if(args.len() < 2) {
        println!("Usage: integer_factors number");
        println!("Where number is the number you want to calculate up to.");
        return;
    } 

    let target : u64 = args[1].parse().unwrap();

    let mut target_set : Vec<u64> = Vec::new();

    for i in 1..(target+1) {
        target_set.push(i);
    }

    let mut test : u64 = target;

    let mut state : usize = 0;

    loop {
        // println!("Testing {}",test);
        for i in &target_set {
            // println!("Testing against {}",i);
            if(test % i != 0) {
                // println!("Failed, moving on");
                state = 1;
                break;
            }
        }
        if(state == 0) {
            // println!("Success! Next");
            break;    
        } else {
            state = 0;
            test += 1;
        }
    }

    println!("Found: {}",test);

}
