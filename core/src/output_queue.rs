use crate::utils::find_newline;

#[derive(Default)]
pub struct OutputQueue(String);
impl OutputQueue {
    pub fn new() -> Self {
        Self(String::new())
    }
    pub fn insert(&mut self, string: &str) -> Vec<String> {
        let queue = &mut self.0;
        queue.push_str(string);
        let mut vec = Vec::new();
        let mut i = 0;
        while let Some(pos) = find_newline(&queue[i..]) {
            vec.push(queue[i..pos].to_string());
            i = pos;
        }
        self.0 = queue[i..].to_string();
        vec
    }
    pub fn left(&mut self) -> String {
        let string = self.0.to_string();
        self.0 = String::new();
        string
    }
}
