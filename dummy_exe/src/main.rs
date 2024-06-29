use std::time::Duration;
fn main() {
    loop {
        // 20 minutes
        std::thread::sleep(Duration::from_secs(1200));
    }
}
