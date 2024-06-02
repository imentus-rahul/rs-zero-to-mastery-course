use tokio::runtime;

use rand::prelude::*;
use std::thread;
use std::time::{Duration};

use std::cell::RefCell;
use std::sync::Arc;
use parking_lot::Mutex;

use std::sync::atomic::{AtomicUsize, Ordering};


#[tokio::main(worker_threads = 4)]  // Error: this macro here and custom runtime created in line 21 are not used together
async fn main() {

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

    let mut tasks = Vec::new();
    for _ in 0..=100 {
        tasks.push(rt.spawn(async {
            let delay = rand::thread_rng().gen_range(1000..2000);
            tokio::time::sleep(Duration::from_millis(delay)).await;
            let thread_current = thread::current();
            let thread_name = thread_current.name().unwrap();
            println!("Delay {delay} ms done! Thread name: {thread_name}");
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    // Error: thread 'main' panicked at 'Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context
}



/*

//  //  //  //  //  //  //  //  BACKUP
use rand::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
// use std::runtime;
use tokio::task;
use tokio::*;
// use tokio::timer::Delay;
// use tokio::time::{DelayQueue};
use futures::future::*;

// #[tokio::main(worker_threads = 4)]
// async fn main() {
//     // create a threadpool, EXECUTOR
//     // limit to 4 threads

//     // let mut rt = runtime::Builder::new()
//     // .core_threads(4)
//     // .build()
//     // .unwrap();

            // let rt = runtime::Builder::new_multi_thread()
            //     .thread_name("my-pool")
            //     .build();

//     let mut handles = vec![];

//     // create 100 tasks
//     for _ in 0..=100 {
//         // 1-2 second delay
//         let delay = rand::thread_rng().gen_range(1000..2000);

//         // setup a task
//         // does not run untill spawned
//         // Delay
//         // Print a result

//         let task = Delay::new(Instant::now() + Duration::from_millis(delay)).then(move |_| {
//             println!(
//                 "Delay {} ms done! Thread name: {}",
//                 delay,
//                 thread::current().name().unwrap()
//             );
//             Ok(())
//         });

//         // Add this task to tokio
//         // rt.spawn(task);
//         handles.push(task::spawn(async {
//             let ms = rand::thread_rng().gen_range(1000..2000);
//             thread::sleep(Duration::from_millis(ms));
//             println!(
//                 "ms: {}, thread name: {}",
//                 ms,
//                 thread::current().name().unwrap()
//             );
//         }));

//         // // Spawn two tasks, one gets a key, the other sets a key
//         // let t1 = tokio::spawn(async {
//         //     let res = client.get("hello").await;
//         // });

//         // let t2 = tokio::spawn(async {
//         //     client.set("foo", "bar".into()).await;
//         // });

//         // t1.await.unwrap();
//         // t2.await.unwrap();
//     }

//     // wait for all of tasks to finish
//     // rt.shutdown_on_idle().wait().unwrap();
//     for handle in handles {
//         handle.await.unwrap();
//     }
//     println!("job done");
// }

// // async fn main() {
// //     let mut handles = vec![];
// //     for _ in 0..100 {
// //         // We must pass a Future to task::spawn.
// //         // Using an async block does this.
// //         handles.push(task::spawn(async {
// //             let ms = rand::thread_rng().gen_range(1000..2000);
// //             thread::sleep(Duration::from_millis(ms));
// //             println!(
// //                 "ms: {}, thread name: {}",
// //                 ms,
// //                 thread::current().name().unwrap()
// //             );
// //         }));
// //     }

// //     for handle in handles {
// //         handle.await.unwrap();
// //     }

// //     println!("done");
// // }

// #[tokio::main]
#[tokio::main(worker_threads = 4)]
async fn main() {
    let mut tasks = Vec::new();
    for _ in 0..=100 {
        tasks.push(tokio::spawn(async {
            let delay = rand::thread_rng().gen_range(1000..2000);
            tokio::time::sleep(Duration::from_millis(delay)).await;
            let thread_current = thread::current();
            let thread_name = thread_current.name().unwrap();
            println!("Delay {delay} ms done! Thread name: {thread_name}");
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
}



*/