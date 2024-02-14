enum State {
    Draft,
    PendingReview,
    Published,
}

pub struct Post {
    content: String,
    state: State,
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            state: State::Draft,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        match self.state {
            State::Published => &self.content,
            _ => "",
        }
    }
    pub fn request_review(&mut self) {
        match self.state {
            State::Draft => self.state = State::PendingReview,
            _ => (),
        };
    }
    pub fn approve(&mut self) {
        match self.state {
            State::PendingReview => self.state = State::Published,
            _ => (),
        };
    }
}
