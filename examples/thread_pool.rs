extern crate fpool;

use fpool::RoundRobinPool;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Use static variable to label the threads as they're spawned
    static mut THREAD_INDEX : usize = 0;

    let thread_spawner = || -> io::Result<_> {
        let index = unsafe {
            let old_index = THREAD_INDEX;
            THREAD_INDEX += 1;
            old_index
        };

        let (tx, rx) = mpsc::channel();
        let join_handle = thread::Builder::new()
            .name(index.to_string())
            .spawn(move || {
                // Loop continuously, reading messages as they are received
                loop {
                    match rx.recv() {
                        Ok(msg) => println!("Thread({}): {}", index, msg),
                        // disconnected, let's shutdown this thread
                        Err(_err) => break,
                    }
                }
            })?;

        Ok((tx, Some(join_handle)))
    };

    let mut pool = RoundRobinPool::builder(5, thread_spawner)
        .build()
        // We can handle initial thread spawn failures here
        .expect("Thread spawns");

    let messages = vec!["Good day.", "How do you do.", "Hola.", "Top of the morning to ya."];
    for message in messages {
        // We can handle thread spawn failures here if they occur
        let handle = pool.get().expect("Thread spawns");
        let should_invalidate = {
            let (ref mut tx, ref mut join_handle) = handle.as_item_mut();
            if let Err(_err) = tx.send(message) {
                // failed, discard the message
                // and join with the thread, as it will be restarted
                if let Some(handle) = join_handle.take() {
                    let _ = handle.join();
                }
                true
            } else {
                false
            }
        };

        if should_invalidate {
            handle.invalidate();
        }
    }

    // Let the threads have time to receive the messages
    thread::sleep(Duration::from_secs(2));

    // Should print out (in random order, due to thread scheduling):
    //
    // Thread(3): Top of the morning to ya.
    // Thread(2): Hola.
    // Thread(1): How do you do.
    // Thread(0): Good day.
}
