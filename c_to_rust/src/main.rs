extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    let x = 2;
    let y = 4;
    let z = unsafe {
        let z = add(x, y);
        z
    };
    println!("According to C, {} + {} = {:?}", x, y, z);
}
