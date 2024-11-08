pub struct Post {
    state: Option<Box<dyn State>>,
    pub content: String,
}

impl Post {

    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // self.content.push_str(text);

        // https://stackoverflow.com/questions/22282117/how-do-i-borrow-a-reference-to-what-is-inside-an-optiont
        
        if let Some(ref s) = self.state {
            if s.is_draft() {
                self.content.push_str(text);
            }
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // pub fn request_review(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.request_review())
    //     }
    // }

    pub fn request_review_pre(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review_pre())
        }
    }
    
    pub fn request_review_final(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review_final())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

}

trait State {
    // fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn request_review_pre(self: Box<Self>) -> Box<dyn State>;
    fn request_review_final(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn is_draft(&self) -> bool;

}

struct Draft {}

impl State for Draft {
    
    // fn request_review(self: Box<Self>) -> Box<dyn State> {
    //     println!("req for draft");
    //     Box::new(PendingReview {})
    // }

    fn request_review_pre(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview_Pre {})
    }
    fn request_review_final(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn is_draft(&self) -> bool {
        true
    }
}

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         println!("req for pending");
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
//     fn reject(self: Box<Self>) -> Box<dyn State> {
//         println!("req for reject");
//         Box::new(Draft {})
//     }
//     fn get_state(self: Box<Self>) -> bool {
//         false
//     }
// }

struct PendingReview_Pre {}

impl State for PendingReview_Pre {
    fn request_review_pre(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review_final(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview_Final {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn is_draft(&self) -> bool {
        false
    }
}
struct PendingReview_Final {}

impl State for PendingReview_Final {
    fn request_review_pre(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review_final(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview_Pre {})
    }
    fn is_draft(&self) -> bool {
        false
    }
}

struct Published {}

impl State for Published {

    // fn request_review(self: Box<Self>) -> Box<dyn State> {
    //     self
    // }

    fn request_review_pre(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn request_review_final(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn is_draft(&self) -> bool {
        false
    }
}