pub trait Get {
    fn get(&self, title: &str) {
        println!("Getting the task: {}", title);
    }
}
