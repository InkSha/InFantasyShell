mod cmd;
mod system;
mod vfs;

#[tokio::main]
async fn main() {
    cmd::Cmd::new("infs").run();
}
