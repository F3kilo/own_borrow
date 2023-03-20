pub fn non_copy() {
    let s = String::from("hello");
    let _new_owner = s;
    // let _other_owner = s;
}

pub fn copy() {
    let s = 42;
    let _new_owner = s;
    let _other_owner = s;
}

pub fn arrays() {
    let arr = ['a', 'b', 'c'];
    let _new_owner = arr;
    let _other_owner = arr;

    let arr = [String::from("hello"), String::from("world")];
    let _new_owner = arr;
    // let _other_owner = arr;
}

pub fn tuples() {
    let tup = (42, true, 4.2);
    let _new_owner = tup;
    let _other_owner = tup;

    let tup = (tup.0, tup.1, String::from("hello"));
    let _new_owner = tup;
    // let _other_owner = tup;
}

pub fn move_to_fn() {
    let s = String::from("hello");
    move_string(s);
    // move_string(s);
}

pub fn other_moves() {
    let s = String::from("hello");
    let _arr = [s, String::from("world")];

    let s = String::from("hello");
    let _tup = [s];

    struct SomeStruct(String);
    let s = String::from("hello");
    let _some_struct = SomeStruct(s);
}

pub fn move_string(_val: String) {}

fn main() {}