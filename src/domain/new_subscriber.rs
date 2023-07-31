use serde::Deserialize;

use super::{subscriber_name::SubscriberName, SubscriberEmail};

#[derive(Debug, Deserialize)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
