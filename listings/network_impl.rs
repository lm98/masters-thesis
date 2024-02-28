struct SyncMQTTNetwork {
    client: rumqttc::Client,
    mb: Arc<Mutex<Vec<Bytes>>>,
}

impl SyncMQTTNetwork {
    pub fn new(
        options: MqttOptions,
        topics: Vec<i32>,
        mqtt_channel_cap: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let (mut client, mut connection) = Client::new(options, mqtt_channel_cap);
        SyncMQTTNetwork::subscribe_to_topics(&mut client, topics)?;
        let mb: Arc<Mutex<Vec<Bytes>>> = Arc::new(Mutex::new(vec![]));

        let mb_clone = Arc::clone(&mb);
        thread::spawn(move || {
            loop {
                for (_i, notification) in connection.iter().enumerate() {
                    match notification {
                        Ok(Incoming(rumqttc::Packet::Publish(msg))) => {
                            if let Ok(mut mb) = mb_clone.lock() {
                                mb.push(msg.payload);
                            }
                        }
                        _ => {}
                    }
                }
            }
        });
        Ok(Self { client, mb })
    }

    fn subscribe_to_topics(client: &mut Client, topics: Vec<i32>) -> NetworkResult<()> {
        for nbr in topics.clone() {
            if let Err(e) = client
                .subscribe(format!("hello-rufi/{nbr}/subscriptions"), QoS::AtMostOnce)
            {
                return Err(e.into());
            }
        }
        Ok(())
    }
}

impl Network for SyncMQTTNetwork {
    fn send(&mut self, msg: Message) -> Result<(), Box<dyn<Error>> {
        let source = msg.source;
        let to_send = serde_json::to_vec(&msg)?;
        self.client
            .try_publish(
                format!("hello-rufi/{source}/subscriptions"),
                QoS::AtMostOnce,
                false,
                to_send,
            )
            .map_err(|e| e.into())
    }

    fn receive(&mut self) -> Result<HashMap<i32,Message>, Box<dyn Error>> {
        let mut mailbox = MemoryLessMailbox::new();

        for u in self.mb.lock()?.iter() {
            if let Ok(mex) = serde_json::from_slice::<Message>(u) {
                mailbox.enqueue(mex)
            }
        }

        Ok(mailbox.messages())
    }
}