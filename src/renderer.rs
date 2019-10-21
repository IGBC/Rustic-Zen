use image::Image;
use std::thread;
use plumbing::Message;
use std::sync::mpsc;
use plumbing::CompleteRay;

pub struct Renderer {
    sender:  mpsc::Sender<Message<CompleteRay>>,
    worker: thread::JoinHandle<Image>,
}

impl Renderer {
    pub fn new(width: usize, height: usize, lightpower: f64) -> Self {
        
        let (sender, receiver) = mpsc::channel();

        let worker = thread::spawn(move || {
            let mut image = Image::new(width, height, lightpower);
            loop {
                let message: Message<CompleteRay> = receiver.recv().unwrap();
                match message {
                    Message::Next(r) => {
                        image.draw_line(r.wavelength, r.start.x, r.start.y, r.end.x, r.end.y);
                    }
                    
                    Message::Terminate => {
                        println!("Renderer Terminating");
                        return image;
                    }
                }
            }
        });

        Renderer {
            sender,
            worker,
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<Message<CompleteRay>> {
        self.sender.clone()
    }

    pub fn get_image(self) -> Image {
        self.sender.send(Message::Terminate);
        self.worker.join().expect("Renderer Panicked Before Join")
    }
}
