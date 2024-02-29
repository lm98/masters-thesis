fn main() -> Result<(), Box<dyn std::error::Error>> {
    // CLI logic to get the self_id and is_source omitted

    let discovery = MockDiscovery(self_id);
    let nbrs = discovery.discover_neighbors();
    let setup = MockSetup {};
    // Setup Context
    let local_sensor: HashMap<SensorId, Rc<Box<dyn Any>>> = vec![(
        sensor("source"),
        Rc::new(Box::new(is_source) as Box<dyn Any>),
    )]
    .into_iter()
    .collect();
    let context = Context::new(
        self_id,
        local_sensor.clone(),
        Default::default(),
        Default::default(),
    );
    // Setup the MQTT client network
    let mut mqttoptions =
        MqttOptions::new(format!("device#{}", self_id), "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let network = SyncMQTTNetwork::new(mqttoptions, nbrs.clone(), 10);
    let time = TimeImpl::new();
    // Setup the platform and run the program
    RuFiPlatform::new(network?, context, discovery, setup, time)
        .run_forever(gradient)
}
