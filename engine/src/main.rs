use engine::redis_manager::redis_manager;

fn main() {

    
        let redis_manager = redis_manager::RedisManager::instance().lock().unwrap();
        let client = redis_manager.get_client();
        let mut con = client.get_connection().expect("Failed to get Redis connection");

        
        
    
    
}
