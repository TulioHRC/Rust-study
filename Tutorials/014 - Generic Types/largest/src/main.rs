fn get_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    let i32_list: Vec<i32> = vec![10, 5, 6, 22, 19];
    let char_list: Vec<char> = vec!['e', 'a', 'z', 'b', 'i', 'p'];

    let i32_largest = get_largest(&i32_list);
    let char_largest = get_largest(&char_list);

    println!("i32 biggest -> {i32_largest}");
    println!("char biggest -> {char_largest}");

}