use axum::Error;
use std::sync::Mutex;

use crate::schemas::event_schema::Event;

static mut GLOBAL_EVENTS: Option<Mutex<Vec<Event>>> = None;

pub async fn get_event() -> Result<Vec<Event>, Error> {
    unsafe {
        if let Some(event) = &GLOBAL_EVENTS {
            let locked_events = event.lock().unwrap();
            Ok(locked_events.clone().to_vec())
        } else {
            Ok(Vec::new())
        }
    }
}

pub async fn add_event_todb(
    // id: String,
    name: String,
    description: String,
    date: String,
    price: f64,
) -> Result<Event, Error> {
    let event = Event {
        id: "id".to_string(),
        name,
        price,
        description,
        date,
    };
    unsafe {
        if let Some(events) = &GLOBAL_EVENTS {
            let mut locked_events = events.lock().unwrap();
            locked_events.push(event.clone());
            Ok(event)
        } else {
            GLOBAL_EVENTS = Some(Mutex::new(vec![event.clone()]));
            Ok(event)
        }
    }
}
