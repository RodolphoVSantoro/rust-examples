use ArcMutex::ArcMutex;

fn main() {
    let a = ArcMutex::new(1);
    let b = a.clone();
    let c = a.clone();
    match a.lock() {
        Ok(mut x) => *x = 2,
        Err(_) => panic!("lock failed"),
    }
    let d = b.lock();
    dbg!(c.try_lock());
}
