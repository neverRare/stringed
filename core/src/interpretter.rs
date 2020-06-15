use crate::parser::Node;

pub struct Interpretter<I, O>
where
    I: Fn() -> String,
    O: Fn(&str) -> (),
{
    input: I,
    output: O,
    queue: String,
}
impl<I, O> Interpretter<I, O>
where
    I: Fn() -> String,
    O: Fn(&str) -> (),
{
    pub fn new(input: I, output: O) -> Self {
        Self {
            input,
            output,
            queue: String::new(),
        }
    }
    fn queue_output(&mut self, string: &str) {
        self.queue.push_str(string);
        let mut lines: Vec<&str> = self.queue.lines().collect();
        let new_queue = if self.queue.ends_with('\n') || lines.is_empty() {
            ""
        } else {
            lines.pop().unwrap()
        };
        for line in lines {
            (self.output)(line);
        }
        self.queue = new_queue.to_string();
    }
    fn run_node(&mut self, var: &str, node: &Node) -> Result<(), String> {
        match node {
            Node::Group(node) => self.run_node(var, node)?,
            Node::Closure { left, right } => self.run_node(&self.eval(var, left)?, right)?,
            Node::Concat(vec) => {
                for node in vec {
                    self.run_node(var, node)?;
                }
            }
            Node::Eval(node) => self.run_node(var, &Node::parse(&self.eval(var, node)?)?)?,
            node => self.queue_output(&self.eval(var, node)?),
        }
        Ok(())
    }
    fn eval(&self, var: &str, node: &Node) -> Result<String, String> {
        match node {
            Node::Literal(content) => Ok(content.to_string()),
            Node::Group(node) => self.eval(var, node),
            Node::Prompt => Ok((self.input)()),
            Node::Var => Ok(var.to_string()),
            Node::Closure { left, right } => self.eval(&self.eval(var, left)?, right),
            Node::Concat(vec) => {
                let mut val = String::new();
                for node in vec {
                    val.push_str(&self.eval(var, node)?);
                }
                Ok(val)
            }
            Node::Slice { src, lower, upper } => {
                let src = self.eval(var, src)?;
                let lower = match lower {
                    Some(node) => parse_int(&self.eval(var, node)?)?,
                    None => 0,
                };
                let upper = match upper {
                    Some(node) => parse_int(&self.eval(var, node)?)?,
                    None => src.len(),
                };
                if upper > src.len() {
                    Err("upper bound larger than the length of string".to_string())
                } else if lower > upper {
                    Err("lower bound larger than upper bound".to_string())
                } else {
                    Ok(src[lower..upper].to_string())
                }
            }
            Node::Equal { left, right } => {
                Ok((self.eval(var, left)? == self.eval(var, right)?).to_string())
            }
            Node::Length(node) => Ok((self.eval(var, node)?).len().to_string()),
            Node::Eval(node) => Ok(self.eval(var, &Node::parse(&self.eval(var, node)?)?)?),
        }
    }
    pub fn run(&mut self, src: &str) -> Result<(), String> {
        self.run_node("", &Node::parse(src)?)?;
        (self.output)(&self.queue);
        Ok(())
    }
}
fn parse_int(a: &str) -> Result<usize, String> {
    match a.parse() {
        Ok(num) => Ok(num),
        Err(reason) => Err(reason.to_string()),
    }
}
