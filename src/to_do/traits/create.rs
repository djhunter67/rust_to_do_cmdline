pub trait Create {
    fn create(&self, title: &str) {
        println!("Create: {title} is being created",);
    }
}