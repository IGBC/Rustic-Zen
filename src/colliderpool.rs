use std::thread;
use ray::{Ray, HitData};
use plumbing::Message;
use object::Object;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use geom::Rect;
use plumbing::CompleteRay;

pub struct ColliderPool {
    workers: Vec<ColliderWorker>,
    sender: mpsc::Sender<Message<Ray>>,
}

struct ColliderWorker {
    id: usize,
    worker: Option<thread::JoinHandle<()>>,
}

impl ColliderWorker {
    fn new(id: usize, scene: &Vec<Object>, viewport: &Rect, receiver: Arc<Mutex<mpsc::Receiver<Message<Ray>>>>, drawing: mpsc::Sender<Message<CompleteRay>>, bounce: mpsc::Sender<Message<HitData>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Next(r) => {
                        println!("Collider {} Working on ray", id);
                        match r.collision_list(scene, viewport) {
                            None => (),
                            Some((hit, hitdata)) => {
                                let complete = CompleteRay {
                                    start: r.get_origin().clone(),
                                    end: hit,
                                    wavelength: r.get_wavelength(),
                                };

                                // Send the complete ray to drawing subsystem;
                                drawing.send(Message::Next(complete));
                                
                                // Send the hit data to shader Processing:
                                match hitdata {
                                    Some(d) => {bounce.send(Message::Next(d));},
                                    None => {},
                                };
                            }
                        }
                    }
                    Message::Terminate => {
                        println!("Collider {} Terminating", id);
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
    fn new(size: usize, scene: &Vec<Object>, viewport: &Rect, drawing_sender: mpsc::Sender<Message<CompleteRay>>, shader_sender: mpsc::Sender<Message<HitData>>) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(ColliderWorker::new(id, scene, viewport, Arc::clone(&receiver), drawing_sender.clone(), shader_sender.clone()));
        }

        ColliderPool {
            workers,
            sender,
        }
    }

    fn get_sender(&self) -> &mpsc::Sender<Message<Ray>> {
        &(self.sender)
    }
}