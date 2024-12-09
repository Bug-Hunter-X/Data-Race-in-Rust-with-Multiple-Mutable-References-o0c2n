fn main() {
    let mut x = 5;

    // Use a mutable reference with a scope to prevent data races
    {
        let y = &mut x;
        *y = 10;
    }

    {
        let z = &mut x;
        *z = 15; 
    }
    println!("x: {}", x); // x will be 15
}