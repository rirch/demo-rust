#![allow(dead_code)]

use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::time::Instant;
use std::process::Command;


fn main() {
    let (tx, rx) = mpsc::channel();
	let now = Instant::now();
    println!("{:?}", now);

    thread::spawn(move || {
        println!("th1 1: {:?}", thread::current().name());
        thread::sleep(Duration::from_millis(300));
        println!("th1 2: {:?}", thread::current().name());
    });
    thread::spawn(move || {
        println!("th2 1: {:?}", thread::current().name());
        thread::sleep(Duration::from_millis(200));
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("th2 2: {:?}", thread::current().name());
    });
    println!("main: {:?}", thread::current().name());

    println!("Got: ...");
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
	
    println!("----------------");

    let cn = ClassName::new(1212);
    println!("cn: {:?}", cn);

    ClassName::public_method(&cn);



    let builder = thread::Builder::new().name("foo".into()).stack_size(32 * 1024);
    builder.spawn(|| {
        println!("foo:{:?}", thread::current().name());
    }).unwrap().join().unwrap();

    let output = Command::new("rustc").arg("--version").output().expect("output error");
    let status = &output.status.code();
    println!("{:?}", status);
    
    let outstr = String::from_utf8_lossy(&output.stdout);
    println!("{:?}", outstr);
    println!("-----------------");
	
	let i1 = 11;
	let s1 = "s11";
	print_type_of(i1);
	print_type_of(s1);
	print_type_of(now);
	print_type_of(core::any::type_name::<&str>());
	
	let max_nanoseconds = i32::MAX ;
	println!("{:?}", max_nanoseconds);
	
	println!("secs:{}", now.elapsed().as_millis());
	println!("{:?}", now);
	
}

fn tt(){

}

fn print_type_of<T>(_: T) {
    println!("{}", core::any::type_name::<T>())
}


#[derive(Debug)]
struct ClassName {
    field: i32,
}

impl ClassName {
    pub fn new(value: i32) -> ClassName {
        ClassName {
            field: value
        }
    }

    pub fn public_method(&self) {
        println!("from public method {}", self.field);
        self.private_method();
    }

    fn private_method(&self) {
        println!("from private method");
    }
}