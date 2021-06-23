// #![windows_subsystem = "windows"]

// extern crate winapi;
// use std::io::Error;

use std::fs::*;
//use std::net::*;
use std::io::*;
use std::string::String;
//use std::str;
// use std::path::*;
// use std::error::Error;

// fn print_message(msg: &str) -> Result<i32, Error> {
//     use std::ffi::OsStr;
//     use std::iter::once;
//     use std::os::windows::ffi::OsStrExt;
//     use std::ptr::null_mut;
//     use winapi::um::winuser::{MB_OK, MessageBoxW};
//     let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
//     let ret = unsafe {
//         MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
//     };
//     if ret == 0 { Err(Error::last_os_error()) }
//     else { Ok(ret) }
// }

fn main() {
    // print_message("aa").unwrap();
    
    let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // let s3 = format!("{}{}",&s1,&s2);
    println!("{}", &s1);


    let mut v = Vec::new();

    v.push(5.5);
    v.push(6.0);
    // v.push(s1.toString());

    println!("{:?}", v);


    
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    println!("{:?}", v1_iter);

//    let mut socket = TcpStream::connect("127.0.0.1:8080").unwrap();
//    socket.write(b"GET / HTTP/1.0\n\n").expect("Failed to write to stream");
    
//    let mut reader = BufReader::new(&socket);
//    let mut buffer: Vec<u8> = Vec::new();
//    reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
//    println!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));


    // -------------------------------------------
    let file = File::open("./Cargo.toml").unwrap();
    let fin = BufReader::new(&file);
    for line in fin.lines() {
        println!("## {}", line.unwrap());
    }
    

    // println!("{:?}", File::open("./Cargo.toml"));
    

}