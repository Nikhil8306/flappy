
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use device_query::{DeviceEvents, DeviceEventsHandler};
pub use device_query::Keycode;

use std::collections::HashSet;

pub struct Input {

    eventReceiver: Option<Receiver<KeyEvent>>,

    keyPress: HashSet<Keycode>,
    keyDown: HashSet<Keycode>,
    keyUp: HashSet<Keycode>,
    
}

enum KeyEvent {
    Down(Keycode),
    Up(Keycode)
}

impl Input {
    pub(super) fn new() -> Self {
        return Self {
            eventReceiver: None,

            keyPress: HashSet::new(), // Keys which are pressed

            keyDown: HashSet::new(), // Keys which got down
            keyUp: HashSet::new(), // Keys which got upzz
        };
    }

    pub(super) fn init(&mut self) { 
        
        let (sender, receiver) = mpsc::channel();
        self.eventReceiver = Some(receiver);

        thread::spawn( move || {
            
            let device_query = DeviceEventsHandler::new(Duration::from_millis(0)).expect("Something went wrong");
            let sender1 = sender.clone();
            let sender2 = sender.clone();


            let _guard = device_query.on_key_down(move|key: &Keycode| {
                sender1.send(KeyEvent::Down(key.clone()));
            });

            let _guard = device_query.on_key_up(move |key: &Keycode| {
                sender2.send(KeyEvent::Up(key.clone()));
            });

            loop {
                thread::sleep(Duration::from_secs(1));
            }

        });

    }
    
    pub(super) fn update(&mut self) {
        
        self.keyDown.clear();
        self.keyUp.clear();
        
        let eventReceiver = self.eventReceiver.as_mut().unwrap();

        for event in eventReceiver.try_iter() {

            match event {

                KeyEvent::Down(key) => {
                    self.keyDown.insert(key);
                    self.keyPress.insert(key);
                },

                KeyEvent::Up(key ) => {
                    self.keyUp.insert(key);
                    self.keyPress.remove(&key);
                }

            };
        }
    }

    pub fn onKeyDown(&self, key: Keycode) -> bool {
        
        if self.keyDown.contains(&key) {
            return true;
        }

        false

    }

    pub fn onKeyUp(&self, key: Keycode) -> bool {
        
        if self.keyUp.contains(&key) {
            return true;
        }

        false

    }

    pub fn onKeyPress(&self, key: Keycode) -> bool {
        
        if self.keyPress.contains(&key) {
            return true;
        }

        false

    }

    pub(super) fn close(&mut self) {
        todo!("Close the Input System");
    }
}       