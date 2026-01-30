fn main() {
    example_scalars();
    example_compounds();
}

fn example_scalars() {
    let x = 2.0;
    let y: f32 = 3.0;
    println!("floats: x = {x}, y = {y}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;
    println!(
        "ints: sum = {sum}, diff = {difference}, prod = {product}, quot = {quotient}, trunc = {truncated}, rem = {remainder}"
    );

    let t = true;
    let f: bool = false;
    println!("bools: t = {t}, f = {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("chars: c = {c}, z = {z}, cat = {heart_eyed_cat}");
}

fn example_compounds() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tuple destructured: x = {x}, y = {y}, z = {z}");
    println!(
        "tuple direct: .0 = {}, .1 = {}, .2 = {}",
        tup.0, tup.1, tup.2
    );

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let first = months[0];
    let second = months[1];
    println!("array months: first = {first}, second = {second}");
}
