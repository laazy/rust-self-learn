fn vector() {
    let mut v = Vec::new();
    v.push(2);

    let v = vec![1, 2, 3, 4, 5];

    let third = v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(3);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(row);
    println!("size of SpreadsheetCell: {}", std::mem::size_of::<SpreadsheetCell>());
}

fn string(){
    let data = "initial contents";

    let s = data.to_string();
    println!("{}", s);
    // 该方法也可直接用于字符串字面值：
    let _s = "initial contents".to_string();

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 is {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");
    println!("{s1} {s2} {s3}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

}

fn hash_map(){
    use std::collections::HashMap;
    use std::collections::hash_map::Entry;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).unwrap_or(&0);
    dbg!(score);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    dbg!(score);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}: {}", field_name, field_value)
    // value borrowed!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    dbg!(&scores);

    for &key in ["Blue", "Yellow"].iter() {
        match scores.entry(String::from(key)){
            Entry::Occupied(o) => {
                println!("Occupied: {:#?}", o);
            },
            Entry::Vacant(v) => {
                println!("Vacant: {:#?}", v);
            },
        }
        }
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    dbg!(scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}


fn main() {
    println!("vector:");
    vector();
    println!("string:");
    string();
    println!("hash_map:");
    hash_map();
}
