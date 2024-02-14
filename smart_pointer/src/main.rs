use std::rc::Rc;

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

    rc();

    ref_cell();

    // reference loop
    reference_loop();
}

fn rc(){
        // Rc<T> for multiple ownership
        enum List {
            Cons2(i32, Rc<List>),
            Nil2,
        }
    
        use List::{Cons2, Nil2};
    
        let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons2(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = Cons2(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn ref_cell(){
        // RefCell<T> and the Interior Mutability Pattern
        #[derive(Debug)]
        enum List {
            Cons3(Rc<RefCell<i32>>, Rc<List>),
            Nil3,
        }
    
        use std::cell::RefCell;
        use List::{Cons3, Nil3};
    
        let value = Rc::new(RefCell::new(5));
    
        let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));
    
        let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
        *value.borrow_mut() += 10;
    
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    
}

fn reference_loop() {
    use std::cell::RefCell;

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
}
