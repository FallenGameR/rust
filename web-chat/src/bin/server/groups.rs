use async_std::task;
use web_chat::ServerPacket;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast::{self, Sender, Receiver, error::RecvError};

use crate::Outbound;

pub struct Group
{
    name: Arc<String>,
    sender: Sender<Arc<String>>
}

const MESSAGE_QUEUE_CAPACITY: usize = 1000;

impl Group
{
    pub fn new(name: Arc<String>) -> Group
    {
        let (sender, _) = broadcast::channel(MESSAGE_QUEUE_CAPACITY);
        Group { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>)
    {
        // Who is monitoring the tasks that we create here?
        // Looks like the vars are moved here and when the tasks exists all the cleanup is done automatically
        let receiver = self.sender.subscribe();
        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>)
    {
        // Ignoring error here for unclear reasons.
        // If there are no subscribers (all tasks did exit) this call will return error.
        // But if there are no subscribers the counter on Outbound is zero and it is cleaned up automatically.
        let _ = self.sender.send(message);
    }
}

async fn handle_subscriber(group: Arc<String>, mut receiver: Receiver<Arc<String>>, outbound: Arc<Outbound>)
{
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => ServerPacket::Message { group: group.clone(), message: message.clone() },
            Err(RecvError::Lagged(n)) => ServerPacket::Error(format!("Dropped {} messages from {}", n, group)),
            Err(RecvError::Closed) => break,
        };

        let reply_result = outbound.send(packet).await;
        if reply_result.is_err() {
            break;
        }
    }
}

// Std mutex is used here. In case there is no need
// to await anything it is faster compared to async Mutex
pub struct Groups(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl Groups
{
    pub fn new() -> Groups
    {
        Groups(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &String) -> Option<Arc<Group>>
    {
        self.0
            .lock()
            .unwrap()
            .get(name)
            .cloned() // Cloned returns an option instead of just doing Clone
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group>
    {
        self.0
            .lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Group::new(name)))
            .clone() // Clone just increments reference count
    }
}
