fn main() {
    let x = vec![1, 2, 3];

    for n in &x {
        println!("{}", n);
    };

    let y = match &x {
        r => r,
    };

    let z = match &x {
        r => r,
    };

    println!("Got {}", x[1]);
    println!("Got {}", x[1]);
}
