fn main() {
    advanced_fn()
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn advanced_fn(){
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
    println!("{}", returns_closure()(1));
}
