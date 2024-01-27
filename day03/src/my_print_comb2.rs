fn display_numbers(i: i32, j: i32) {
    if j > i {
        if !(i == 98 && j == 99) {
            print!("{}{} {}{}, ", i / 10, i % 10, j / 10, j % 10);
        } else {
            println!("98 99")
        }
    }
}

pub fn my_print_comb2() {
    for i in 0..100 {
        for j in 0..100 {
            display_numbers(i, j);
        }
    }
}
