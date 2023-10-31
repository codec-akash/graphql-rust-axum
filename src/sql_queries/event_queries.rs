use axum::Error;
use std::sync::Mutex;

use crate::schemas::event_schema::Event;

static mut GLOBAL_EVENTS: Option<Mutex<Vec<Event>>> = None;

pub async fn get_event() -> Result<Vec<Event>, Error> {
    // Ok(vec![
    //     Event {
    //         id: String::from("1"),
    //         name: "signup event".to_string(),
    //     },
    //     Event {
    //         id: "2".to_string(),
    //         name: "Akash".to_string(),
    //     },
    // ])
    unsafe {
        if let Some(event) = &GLOBAL_EVENTS {
            let locked_events = event.lock().unwrap();
            Ok(locked_events.clone().to_vec())
        } else {
            Ok(Vec::new())
        }
    }
}

pub async fn add_event_todb(id: String, name: String) -> Result<Vec<Event>, Error> {
    let event = Event { id: id, name: name };
    unsafe {
        if let Some(events) = &GLOBAL_EVENTS {
            let mut locked_events = events.lock().unwrap();
            locked_events.push(event);
            Ok(locked_events.clone().to_vec())
        } else {
            GLOBAL_EVENTS = Some(Mutex::new(vec![event]));
            if let Some(event) = &GLOBAL_EVENTS {
                let locked_events = event.lock().unwrap();
                Ok(locked_events.clone().to_vec())
            } else {
                Ok(Vec::new())
            }
        }
    }
}
