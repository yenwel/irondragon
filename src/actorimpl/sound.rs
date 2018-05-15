extern crate robots;
extern crate rodio;

use robots::actors::{Actor, ActorCell};
use std::any::Any;
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SoundCommands {
    PlayRoar,
}


pub struct SoundActor;

impl Actor for SoundActor {
    fn receive(&self, message: Box<Any>, context: ActorCell) {
        if let Ok(message) = Box::<Any>::downcast::<SoundCommands>(message) {
             match *message {
                SoundCommands::PlayRoar => {
                    let device = rodio::default_output_device().unwrap();
                    let file = File::open("static/roar.wav").unwrap();
                    let mut roar = rodio::play_once(&device, BufReader::new(file)).unwrap();
                    roar.set_volume(0.2);
                    println!("Started roar");
                }
             }
        }
    }
}

impl SoundActor {
    pub fn new(_dummy: ()) -> SoundActor {
        SoundActor
    }
}
