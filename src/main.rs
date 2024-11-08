
// The Book
// Chapter 17

// to implement the state pattern in a more traditional object-oriented way,
// https://stackoverflow.com/questions/22282117/how-do-i-borrow-a-reference-to-what-is-inside-an-optiont


use oop_blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch ");
    assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());
    post.add_text("today");
    assert_eq!("I ate a salad for lunch today", post.content);
    
    post.request_review_pre();
    assert_eq!("", post.content());
    
    post.add_text("added");
    assert_eq!("I ate a salad for lunch today", post.content);

    post.reject();
    assert_eq!("", post.content());


    post.request_review_pre();
    assert_eq!("", post.content());


    post.request_review_final();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review_final();
    assert_eq!("", post.content());

    // post.request_review();
    // assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}