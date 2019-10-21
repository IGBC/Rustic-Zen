use std::thread;
use ray::{HitData};
use plumbing::Message;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

pub struct ShaderPool {
    workers: Vec<ShaderWorker>,
    sender: mpsc::Sender<Message<HitData>>,
}

struct ShaderWorker {
    id: usize,
    worker: Option<thread::JoinHandle<()>>,
}

impl ShaderWorker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message<HitData>>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Next(r) => {
                        println!("Shader {} Working", id);

                    }
                    
                    Message::Terminate => {
                        println!("Shader {} Terminating", id);
                        break;
                    }
                }
            }
        });
        ShaderWorker {
            id,
            worker: Some(thread),
        }
    }
}

impl ShaderPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(ShaderWorker::new(id, Arc::clone(&receiver)));
        }

        ShaderPool {
            workers,
            sender,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Message<HitData>> {
        self.sender.clone()
    }
}