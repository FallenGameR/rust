use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::group::Group;

pub struct Groups(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl Groups
{
    pub fn new() -> Groups
    {
        Groups(Mutex::new(HashMap::new()))
    }
}