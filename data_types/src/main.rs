fn main() {
    let _x = 2.0;

    let _y: f32 = 3.0;

    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _product = 4 * 30;

    let _quotient = 56.7 / 32.2;

    let _floored = 2 / 3; // Results in 0

    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false;

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _y);

    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _a = [3; 5]; // let _a = [3, 3, 3, 3, 3];
}
