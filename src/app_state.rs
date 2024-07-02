use tokio_postgres::Client;

pub struct AppState {
    pub client: Client,
}
