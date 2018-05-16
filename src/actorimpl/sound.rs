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
                    let sink = rodio::Sink::new(&device);
                    let file = File::open("static/roar.wav").unwrap();
                    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                    sink.sleep_until_end();
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
