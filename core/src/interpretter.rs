use crate::gen_interpretter::{GenInterpretter, Output};
use crate::output_queue::OutputQueue;

pub struct Interpretter<I, O>
where
    I: Fn() -> String,
    O: Fn(&str) -> (),
{
    input: I,
    output: O,
}
impl<I, O> Interpretter<I, O>
where
    I: Fn() -> String,
    O: Fn(&str) -> (),
{
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
    pub fn run(&mut self, src: &str) -> Result<(), String> {
        let mut interpretter = GenInterpretter::start(src);
        let mut queue = OutputQueue::new();
        let mut result = None;
        loop {
            let input;
            if let Some(result) = result {
                match result {
                    Output::Output(output) => {
                        for output in queue.insert(&output) {
                            (self.output)(&output);
                        }
                        input = None;
                    }
                    Output::Input => {
                        input = Some((self.input)());
                    }
                    Output::Error(reason) => break Err(reason),
                    Output::Done => {
                        (self.output)(&queue.left());
                        break Ok(());
                    }
                }
            } else {
                input = None;
            }
            result = Some(interpretter.next(match &input {
                Some(x) => Some(&x),
                None => None,
            }));
        }
    }
}
