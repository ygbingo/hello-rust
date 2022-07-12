fn main() {
    print_measurement(5, 'h');
}

fn print_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value}{unit_label}");
}
