fn main() {
    let mut s = String::from("hello");
    let ptr = s.as_ptr();

    println!("ptr address: {ptr:?}");
    let data = unsafe { *ptr } as char;
    println!("data: {data}");

    s = String::from("world");
    
    println!("str address: {:?}", s.as_ptr());
    println!("old ptr address: {ptr:?}");
    let data = unsafe { *ptr } as char;
    println!("data: {data}");
}
