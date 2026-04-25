use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    Home,
    Quiz,
    Community,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Post {
    pub content: String,
    pub author: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct QuizItem {
    pub question: String,
    pub options: Vec<String>,
    pub answer_idx: usize,
}
