use async_std::task;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast::{self, Sender, Receiver};

use crate::Outbound;

pub struct Group {
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
        let receiver = self.sender.subscribe();
        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>)
    {
        // Ignoring error here for unclear reasons.
        // If there are no subscribers this call will return error.
        // In this case Outbound TCP stream can be terminated.
        let _ = self.sender.send(message);
    }

}

async fn handle_subscriber(group: Arc<String>, mut receiver: Receiver<Arc<String>>, outbound: Arc<Outbound>)
{

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
