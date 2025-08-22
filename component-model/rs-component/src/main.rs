mod bindings;

use bindings::wasmcloud_tutorial::adder::add::add;

fn main() {
    let result = add(1, 5);
    println!("1 + 5 = {result}");
}
