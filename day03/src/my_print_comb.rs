fn _last_check(i: i32) {
    if i % 10 > i % 100 / 10 && i % 10 > i / 100 && i % 100 / 10 > i / 100 {
        if i != 789 {
            print!("{}{}{}, ", i / 100, i % 100 / 10, i % 10);
        } else {
            println!("{}", i);
        }
    }
}

pub fn _my_print_comb() {
    for i in 1..=789 {
        if i % 10 != i % 100 / 10 && i % 10 != i / 100 && i % 100 / 10 != i / 100 {
            //last_check(i);
        }
    }
}
