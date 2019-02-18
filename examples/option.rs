use inspector::OptionInspector;

fn main() {
    let some = Some(42);
    some.inspect(|x| println!("some value is {}", x)).unwrap();
}
