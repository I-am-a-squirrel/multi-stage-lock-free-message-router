type ProducersNumber = u8;

fn start(number: ProducersNumber) -> () {
    for i in 0..number {
        tokio::spawn(future)
    }
}
