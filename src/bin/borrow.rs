fn read(arg: &String) {
    println!("{arg}")
    // arg.push_str(", world!");
    // let moved = WrappedString(arg);
}

fn change(arg: &mut String) {
    println!("{arg}");
    arg.push_str(", world!");
    // let moved = WrappedString(arg);
}

struct WrappedString(String);
fn own(mut arg: String) -> WrappedString {
    println!("{arg}");
    arg.push_str(", world!");
    WrappedString(arg)
}

//---------------------------//
//---------------------------//
//---------------------------//

pub fn shared_ref() {
    let s = String::from("hello");
    let ref_s = &s;

    // read(&s);

    read(ref_s);
    // change(&mut ref_s);
    // own(ref_s);
}

pub fn unique_ref() {
    let mut s = String::from("hello");
    let mut_ref_s = &mut s;

    read(mut_ref_s);
    change(mut_ref_s);
    // own(ref_s);
}

pub fn ownership() {
    let mut s = String::from("hello");

    // own(s);

    read(&s);
    change(&mut s);
    own(s);
}

//---------------------------//
//---------------------------//
//---------------------------//

pub struct MyStruct {
    s: String,
}

impl MyStruct {
    pub fn shared_self(&self) {
        read(&self.s);
        // change(&mut self.s);
        // own(self.s);
    }

    pub fn unique_self(&mut self) {
        read(&self.s);
        change(&mut self.s);
        // own(self.s);
    }

    pub fn own_self(mut self) {
        read(&self.s);
        change(&mut self.s);
        own(self.s);
    }
}

fn main() {}
