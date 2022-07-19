use game_of_life::Universe;
fn main() {
    let mut universe = Universe::new();
    separate();
    universe.glider();
    universe.display();
    loop {
        separate();
        universe.tick();
        universe.display();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

fn separate() {
    for _ in 0..75 {
        print!("-");
    }
    println!("");
}
