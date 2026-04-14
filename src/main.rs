mod context;

#[tokio::main]
async fn main() {
    context::execute(|context| {
        context.read();

        if context.is_enter() {
            let line = context.read_line();
            let input = line.trim();
            if input == "exit" {
                return context::ContextSignal::EXIT;
            }
            context.new_line();
        } else {
            context.write_with_prompt(context.read_line());
        }

        context::ContextSignal::NONE
    });
}
