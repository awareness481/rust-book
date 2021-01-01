use ::std::io;

// Should do a recursive-memo impl at some point
fn main() {
    let mut nth = String::new();
    
    println!("Type the nth fibonacci you want to calculate: ");
    io::stdin().read_line(&mut nth).expect("I/O error");

    let nth: i32 = match nth.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    let mut result = 0;
    let mut last = 0;
    let mut curr = 1;
    for _n in 1..nth {
        result = last + curr;
        last = curr;
        curr = result;
    }

    println!("{}nth fibonacci number result: {}", nth, result);
}
