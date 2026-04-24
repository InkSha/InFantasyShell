mod cmd;
mod system;
mod utils;

#[tokio::main]
async fn main() {
    cmd::Cmd::new("infs").run();
}
