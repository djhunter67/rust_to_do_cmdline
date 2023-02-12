pub trait Get {
    fn get(&self, title: &str) {
        println!("Get: {title} is being fetched",);
    }
}
