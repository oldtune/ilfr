use std::{sync::Arc, time::Duration};

use rdev::{listen, simulate, Event, EventType, Key};
use tokio::{
    sync::mpsc,
    task,
    time::{self, interval},
};

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel::<bool>(1);
    let arc_sender = Arc::new(sender);
    let forever = tokio::task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        loop {
            tokio::select! {
                _ = receiver.recv() =>{
                    interval.reset();
                },
                _ = interval.tick() => {
                    // simulate(&EventType::MouseMove{x: 1.0, y: 1.0}).unwrap();
                    simulate(&EventType::Wheel{delta_x: 50, delta_y :0}).unwrap();
                    simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
                    simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
                }
            }
        }
    });

    let listener = tokio::task::spawn(async move {
        let callback = move |x: Event| {
            let arc_sender_clone = arc_sender.clone();
            match x.name {
                Some(name) => println!("{}", name),
                _ => (),
            }

            let callback = || {
                tokio::task::spawn(async move {
                    println!("Reset timer");
                    arc_sender_clone.send(true).await.unwrap();
                });
            };

            match x.event_type {
                EventType::Wheel { delta_x, delta_y } => {
                    println!("wheel {}, {}", delta_x, delta_y);
                    callback();
                }
                EventType::ButtonRelease(button) => {
                    println!("Button {:?} released", button);
                    callback();
                }
                EventType::KeyRelease(button) => {
                    println!("Key {:?} released", button);
                    callback();
                }
                EventType::MouseMove { x, y } => {
                    println!("Mouse moved {} {}", x, y);
                    callback();
                }
                _ => (),
            };
        };

        listen(callback).unwrap();
    });

    forever.await.unwrap();
    listener.await.unwrap();
}
