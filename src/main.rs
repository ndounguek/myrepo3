fn main() {
    let mut count_down = 5;

    println!("Preparing to launch...");

    println!("Launching in...");

    while count_down >= 0 {
        println!("{}...", count_down);
	count_down -= 1;
    }

    println!("Liftoff!");
}
