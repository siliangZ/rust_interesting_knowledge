use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

struct OctBscanEvent {
    data: Vec<u8>,
    size: usize,
}

fn main() {
    // create mpsc channel
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut data_list = Vec::new();
        // prepare data_list with 100 empty Vectors
        for _ in 0..100 {
            data_list.push(Arc::new(OctBscanEvent {
                data: vec![0; 100],
                size: 0,
            }));
        }

        // move data out of data_list and create OctBscanEvent
        for event in data_list.into_iter() {
            println!("The original address of data: {:p}", event.data.as_ptr());
            //let event = Arc::new(data);
            println!("The address of data(thread): {:p}", event.data.as_ptr());
            // pass the event to another thread
            tx.send(event).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    // take 1 seconds to receive the event
    let now = std::time::Instant::now();
    while now.elapsed().as_secs() < 1 {
        let event = rx.recv().unwrap();
        println!("The address of data(main): {:p}", event.data.as_ptr());
    }
}
