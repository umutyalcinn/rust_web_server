pub mod router {

    use std::collections::HashMap;
    pub struct Router {
        routes: HashMap<String, String>,
        path_not_found: String
    }
    
    impl Router {
        pub fn new(path_not_found: &str) -> Router{
            Router {
                routes: HashMap::new(),
                path_not_found: String::from(path_not_found),
            }
        }

        pub fn add_route(&mut self, route: &str, path: &str){
            self.routes.insert(route.to_string(), path.to_string());
        }

        pub fn get_route(&self, route: &str) -> String{
            match self.routes.get(route){
                Some(v) => v.to_string(),
                None => self.path_not_found.to_string(),
            }
        }
    }

}
