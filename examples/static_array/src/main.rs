use static_array_helpers::static_array;

static_array!(
    ARR = [
        (1, (Some(true), "True")),
        (2, (Some(false), "False")),
        (3, (None, "Unknown"))
    ]
);

fn main() {
    println!("{}", (ARR[1].1).1);
}
