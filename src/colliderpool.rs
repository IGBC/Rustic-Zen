use std::thread;
use plumbing::{Message, NewRay};
use object::Object;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

pub struct ColliderPool {
    workers: Vec<ColliderWorker>,
    sender: mpsc::Sender<Message<NewRay>>,
}

struct ColliderWorker {
    id: usize,
    worker: Option<thread::JoinHandle<()>>,
}

impl ColliderWorker {
    fn new(id: usize, scene: &Vec<Object>, receiver: Arc<Mutex<mpsc::Receiver<Message<NewRay>>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Next(r) => {
                        println!("Collider {} Working on ray", id);
                    }
                    Message::Terminate => {
                        println!("Collier {} Terminating", id);
                        break;
                    }
                }
            }
        });
        ColliderWorker {
            id,
            worker: Some(thread),
        }
    }
}

impl ColliderPool {
    fn new(size: usize, scene: &Vec<Object>) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(ColliderWorker::new(id, scene, Arc::clone(&receiver)));
        }

        ColliderPool {
            workers,
            sender,
        }
    }
}