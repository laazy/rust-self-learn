fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let x = {
        2;
        1;
    };
    // :? 是一个特殊的格式化字符串，它允许我们打印出来调试信息
    // 这里 x 的类型是 ()，即 unit 类型
    println!("The value of x is: {x:?}");
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}