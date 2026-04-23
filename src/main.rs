mod cmd;
mod system;

#[tokio::main]
async fn main() {
    cmd::Cmd::new("infs").run();
}
