mod cmd;
mod vfs;

#[tokio::main]
async fn main() {
    cmd::Cmd::new("infs> ").run();
}
