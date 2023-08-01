use rexcel::Rexcel;

#[derive(Rexcel)]
struct Test {
    name: String,
    age: i32,
    sex: bool,
}

#[test]
fn it_works() {
    Test::print_field();
}
