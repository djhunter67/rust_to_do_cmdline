pub trait Delete {
    fn delete(&self, title: &str) {
        println!("Delete: {title} is being deleted",);
    }
}
