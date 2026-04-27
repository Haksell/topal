mod ast;
mod tokenizer;

use crate::tokenizer::Tokenizer;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("Σ ".into()),
        DefaultPromptSegment::Empty,
    );

    loop {
        let line = match line_editor.read_line(&prompt) {
            Ok(Signal::Success(buffer)) => buffer,
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("Aborted!");
                return;
            }
            x => {
                println!("Unexpected event: {x:?}");
                return;
            }
        };
        let tokens = Tokenizer::tokenize(&line);
        println!("{tokens:?}");
    }
}
