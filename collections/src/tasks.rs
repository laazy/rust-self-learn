use rand::Rng;
use std::collections::{HashMap, hash_map::Entry};
pub fn num_count(length: usize) {
    let mut rng = rand::thread_rng();
    let mut num = vec![0; length];
    for i in 0..length {
        num[i] = rng.gen_range(1..length);
    }
    num.sort();
    let mid_val = num[length >> 1];
    let mut cnt_map = HashMap::new();
    for i in &num {
        *cnt_map.entry(*i).or_insert(0) += 1;
    }
    let (mut key, mut val) = (0usize, 0);
    for (k, v) in &cnt_map {
        if *v > val {
            key = *k;
            val = *v;
        }
    }
    println!("{:?}", num);
    println!("middle value: {mid_val}, most value: {key}");
    // (mid_val, key)
}

pub fn pig_latin(word: &str) {
    let first_char = word.chars().next().unwrap();
    if matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u') {
        println!("{}hay", word);
        return;
    }
    for (idx, ch) in word.chars().enumerate() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => (),
            _ => {
                // println!("{idx} - {ch}");
                println!("{}{}{ch}ay", &word[..idx], &word[idx + 1..]);
                return;
            }
        }
    }
}

pub fn employee_api() {
    struct EmployeeMap {
        departments: Vec<String>,
        department_employee: HashMap<String, Vec<String>>,
    }

    impl EmployeeMap {
        fn new() -> EmployeeMap {
            EmployeeMap {
                departments: Vec::new(),
                department_employee: HashMap::new(),
            }
        }

        fn add(self: &mut Self, department: &str, employee: &str) {
            match self.department_employee.entry(String::from(department)) {
                Entry::Occupied(mut o) => {
                    o.get_mut().push(String::from(employee));
                }
                Entry::Vacant(v) => {
                    v.insert(vec![String::from(employee)]);
                    self.departments.push(String::from(department));
                }
            }
        }

        fn print(self: &Self) {
            println!("department list: {:?}", self.departments);
            println!("department employee list: {:?}", self.department_employee)
        }
    }

    let mut map = EmployeeMap::new();
    map.add("Sales", "Alice");
    map.add("Sales", "Bob");
    map.add("Engineering", "Carol");
    map.print();
}
