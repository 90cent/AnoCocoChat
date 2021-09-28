pub struct Log {
    pub message: String,
}

impl Log {
    pub fn print(&self) {
        println!("{}",self.message);
    }
    
}
