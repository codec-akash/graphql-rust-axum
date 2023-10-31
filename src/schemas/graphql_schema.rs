use async_graphql::Object;
use axum::Error;

use crate::sql_queries::event_queries::{add_event_todb, get_event};

use super::event_schema::Event;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn event(&self) -> Result<Vec<Event>, Error> {
        let event = get_event().await?;
        Ok(event)
    }
}

pub struct MutationRoot {}

#[Object]
impl MutationRoot {
    async fn add_event(
        &self,
        name: String,
        description: String,
        date: String,
        price: f64,
    ) -> Result<Event, Error> {
        let event = add_event_todb(name, description, date, price).await?;
        Ok(event)
    }
}
