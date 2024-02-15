pub mod gui;
use crate::gui::{Button, Draw, Screen};

#[allow(unused)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn gui() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
pub mod blog;
use crate::blog::Post; //, State, Draft};
fn blog() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
pub mod blog_struct;
use crate::blog_struct::Post as StructPost;

fn blog_struct() {
    let mut post = StructPost::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();

    let post = post.reject().request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
pub mod blog_enum;
use crate::blog_enum::Post as EnumPost;
fn blog_enum() {
    let mut post = EnumPost::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
fn main() {
    gui();
    blog();
    blog_struct();
    blog_enum();
}
