use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast::Sender;

pub struct Group {
    name: Arc<String>,
    sender: Sender<Arc<String>>
}

pub struct Groups(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl Groups
{
    pub fn new() -> Groups
    {
        Groups(Mutex::new(HashMap::new()))
    }
}
