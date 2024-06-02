use tokio::runtime;

use rand::prelude::*;
use std::thread;
use std::time::{Duration};

use std::cell::RefCell;
use std::sync::Arc;
use parking_lot::Mutex;

use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {

    // let mut id = RefCell::new(0);
    // let arc_id = Arc::new(Mutex::new(id));

    // custom runtime
    let rt = runtime::Builder::new_multi_thread()
        // .thread_name("custom-tokio-runtime-worker")
        .enable_all() // it is required for removing error: "A Tokio 1.x context was found, but timers are disabled. Call `enable_time` on the runtime builder to enable timers."
        .worker_threads(4)
        // .thread_name("my-custom-tokio-runtime-worker")
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-runtime-worker-{}", id)

            // Implement Arc Mutex Refcell - to change the id here
            // id =  id + 1;
            // format!("tokio-runtime-worker-{:?}", id)
        })
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();

    rt.block_on(async_main()); // we need to add an executor to async main
    // runtime should block on future processing
}

async fn async_main() {
    let mut tasks = Vec::new();
    for _ in 0..=100 {
        tasks.push(tokio::task::spawn(async {
            let delay = rand::thread_rng().gen_range(1000..2000);
            tokio::time::sleep(Duration::from_millis(delay)).await; // awaiting delay on all threads simultaneously
            let thread_current = thread::current();
            let thread_name = thread_current.name().unwrap();
            println!("Delay {delay} ms done! Thread name: {thread_name}");
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
