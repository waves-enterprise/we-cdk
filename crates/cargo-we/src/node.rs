use serde::{Deserialize, Serialize};

use response::*;

pub struct Node<'a> {
    url: &'a str,
}

impl<'a> Node<'a> {
    pub fn from_url(url: &'a str) -> Self {
        Node { url }
    }

    pub fn transactionSignAndBroadcast(&self) {
        
    }
}



