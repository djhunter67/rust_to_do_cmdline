pub trait Edit {
    fn change_to_completed(&self, title: &str) {
        println!("Edit: {title} is being changed to completed",);
    }

    fn change_to_delayed(&self, title: &str) {
        println!("Edit: {title} is being changed to delayed",);
    }

    fn change_to_abandoned(&self, title: &str) {
        println!("Edit: {title} is being changed to abandoned",);
    }

    fn change_to_pending(&self, title: &str) {
        println!("Edit: {title} is being changed to pending",);
    }
}
