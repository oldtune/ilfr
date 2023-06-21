use std::{sync::Arc, time::Duration};

use rdev::{listen, simulate, Event, EventType, Key};
use tokio::{
    sync::mpsc,
    task,
    time::{self, interval},
};

#[tokio::main]
async fn main() {
    let (mut sender, mut receiver) = mpsc::channel::<bool>(1);
    let forever = tokio::task::spawn_blocking(|| async move {
        let mut interval = time::interval(Duration::from_secs(3));
        loop {
            tokio::select! {
                _ = receiver.recv() =>{
                    interval.reset();
                },
                _ = interval.tick() => {
                    simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
                }
            }
        }
    });

    let listener = tokio::task::spawn_blocking(|| async move{
        let callback = move |x: Event| {
            sender.send(true);
        };

        listen(callback);
    });

    forever.await;
    listener.await;
}
