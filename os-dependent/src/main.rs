#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test() {
    println!("Hello, world!");
}

fn main() {
    test();
}
