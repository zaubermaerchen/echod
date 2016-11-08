mod echo;

fn main() {
    let service = echo::Service::new(12345);
    match service.start() {
        Ok(_) => return,
        Err(e) => println!("{:?}", e)
    }
}