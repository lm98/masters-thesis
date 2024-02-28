pub trait Network {
    fn send(&mut self, msg: Message) -> Result<(), Box<dyn Error>>;
    fn receive(&mut self) -> Result<HashMap<i32, Message>, Box<dyn Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    source: i32,
    msg: Export,
    timestamp: SystemTime,
}