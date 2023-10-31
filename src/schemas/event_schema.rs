use async_graphql::Object;
use axum::Error;

#[derive(Clone)]
pub struct Event {
    pub id: String,
    pub name: String,
}

#[Object]
impl Event {
    pub async fn id(&self) -> String {
        self.id.to_string()
    }

    pub async fn name(&self) -> String {
        self.name.to_string()
    }

    async fn value_from_db(&self) -> Result<Event, Error> {
        Ok(Event {
            id: "1".to_string(),
            name: "text".to_string(),
        })
    }
}
