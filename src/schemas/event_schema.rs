use async_graphql::Object;

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
}
