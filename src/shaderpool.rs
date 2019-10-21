use std::thread;
use ray::{HitData};
use plumbing::Message;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use scene::Mat;

pub struct ShaderPool {
    workers: Vec<ShaderWorker>,
    sender: mpsc::Sender<Message<HitData>>,
}

struct ShaderWorker {
    id: usize,
    worker: Option<thread::JoinHandle<()>>,
}

impl ShaderWorker {
    fn new(id: usize, shader_list: Mat, receiver: Arc<Mutex<mpsc::Receiver<Message<HitData>>>>) -> Self {
        let thread = thread::Builder::new().name(format!("Shader {}", id).to_string()).spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Next(s) => {
                        s

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
            worker: Some(thread.unwrap()),
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

impl Drop for ShaderPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.worker.take() {
                thread.join().unwrap();
            }
        }
    }
}