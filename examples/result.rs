use inspector::ResultInspector;

fn main() {
    let ok: Result<_, ()> = Ok(42);
    ok.inspect(|x| println!("ok value is {}", x)).unwrap();
}
