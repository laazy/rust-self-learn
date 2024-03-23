use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn with_logging(f: fn(i32) -> i32, arg: i32) -> i32 {
    println!("Calling function with argument: {:?}", arg);
    f(arg)
}

macro_rules! log_function_call {
    ($fn_name:ident) => {
        fn logged_func(arg: i32) -> i32 {
            with_logging($fn_name, arg)
        }
    };
}

// 定义一个普通的函数
fn square(x: i32) -> i32 {
    x * x
}

// 使用宏为函数添加日志功能
log_function_call!(square);

fn main() {
    Pancakes::hello_macro();
    println!("Result: {}", square(5));
    println!("Result: {}", logged_func(5));
}
