use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use std::ops::Deref;

    use List::{Cons, Nil};
    let b = Box::new(5);
    println!("b = {}", b);

    //recursive data structure
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    dbg!(list);

    // deref
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("y = {}", y);
    // assert_eq!(5, y); cannot pass
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    let z = *y;
    println!("z = {}", z);

    // let y = MyBox::new(Cons(5, Box::new(Nil)));
    // // Note: Cannot use *y directly, because List does not implement the Copy trait
    // let z = *y;
    // println!("{}", match z {
    //     Cons(i, _) => i,
    //     Nil => 0
    //     })

    let m = MyBox::new(String::from("Rust"));
    // Note: cause String is not certain in compile time, so can not make String -> str in stack
    // let z: str = (*m)[..];
    let z: &str = &(*m)[..];
    hello(z);
    hello(&m);
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // drop trait
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }
    {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");

        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }

    println!("\n\n   rc:");
    rc();

    println!("\n\n   RefCell:");
    ref_cell();

    println!("\n\n   reference loop:");
    reference_loop();

    println!("\n\n   pure rc loop with unsafe:");
    pure_rc_loop();

    println!("\n\n   weak reference:");
    weak_reference();
}

fn rc() {
    // Rc<T> for multiple ownership
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn ref_cell() {
    // RefCell<T> and the Interior Mutability Pattern
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn reference_loop() {
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    use List::{Cons, Nil};

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

fn pure_rc_loop() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&Rc<List>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
        fn value(&self) -> i32 {
            match self {
                Cons(val, _) => *val,
                Nil => 0,
            }
        }
    }

    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Nil)));
    println!("a initial rc count = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(10, Rc::clone(&a)));
    unsafe {
        // 获取a的内部可变性，绕过Rc的不可变性
        let a_ptr: *mut List = Rc::as_ptr(&a) as *mut List;

        // 根据List枚举的内存布局，直接修改其中的Rc<List>
        match &*a_ptr {
            Cons(val, _) => {
                *a_ptr = Cons(*val, Rc::clone(&b));
            }
            Nil => {}
        }
    }
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // uncomment the next line to see that we have a cycle
    // println!("a = {:?}", a);
    // check the cycle is correct
    assert_eq!(a.value(), b.tail().unwrap().value());
    assert_eq!(b.value(), a.tail().unwrap().value());
    use std::ptr;
    assert!(ptr::eq(&*a, &**b.tail().unwrap()));
    assert!(ptr::eq(&*b, &**a.tail().unwrap()));
}

fn weak_reference() {
    #[allow(unused)]
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // leaf.parent.borrow();
}
