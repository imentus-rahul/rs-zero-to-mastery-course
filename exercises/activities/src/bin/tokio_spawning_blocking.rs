use tokio::task;

fn fib_cpu_intensive(n: u32) -> u32 {
    println!("fib_cpu_intensive got hit");
    match n {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n - 1) + fib_cpu_intensive(n - 2),
    }
}

#[tokio::main]
async fn main() {
    let threadpool_future = task::spawn_blocking(||fib_cpu_intensive(2));
    todo!()
}