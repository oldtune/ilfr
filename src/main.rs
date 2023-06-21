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
                    simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
                    simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
                    println!("Sent ctrl");
                }
            }
        }
    });

    let _ = tokio::task::spawn(async move {
        let callback = move |x: Event| {
            let arc_sender_clone = arc_sender.clone();
            match x.name {
                Some(name) => println!("{}", name),
                _ => (),
            }

            tokio::task::spawn(async move {
                println!("Reset timer");
                arc_sender_clone.send(true).await.unwrap();
            });
        };
        listen(callback).unwrap();
    });

    forever.await.unwrap();
}
