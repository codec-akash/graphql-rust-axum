use async_graphql::Object;

#[derive(Clone)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub date: String,
}

#[Object]
impl Event {
    pub async fn id(&self) -> String {
        self.id.to_string()
    }

    pub async fn name(&self) -> String {
        self.name.to_string()
    }

    pub async fn price(&self) -> f64 {
        self.price
    }

    pub async fn description(&self) -> String {
        self.description.to_string()
    }

    pub async fn date(&self) -> String {
        self.date.to_string()
    }
}
