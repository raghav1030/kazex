pub mod redis_manager {
    use std::sync::Mutex;

    use once_cell::sync::Lazy;
    use redis::{Client, Connection};

    const REDIS_HOSTNAME: &str = "localhost:6379";
    const REDIS_PASSWORD: &str = ""; 
    
    fn connect() -> Client {
        let connection_url = if REDIS_PASSWORD.is_empty() {
            format!("redis://{}", REDIS_HOSTNAME)
        } else {
            format!("redis://:{}@{}", REDIS_PASSWORD, REDIS_HOSTNAME)
        };

        Client::open(connection_url).expect("Invalid Connection URL")
    }

    static NEW_REDIS_CLIENT: Lazy<Client> = Lazy::new(|| connect());
    static REDIS_CONNECTION: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

    pub fn get_redis_client() -> &'static Client {
        &NEW_REDIS_CLIENT
    }
}
