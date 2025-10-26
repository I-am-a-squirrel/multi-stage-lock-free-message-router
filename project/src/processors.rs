type ProcessorsNumber = u8;

fn start(number: ProcessorsNumber) -> () {
    for i in 0..number {
        tokio::spawn(future)
    }
}
