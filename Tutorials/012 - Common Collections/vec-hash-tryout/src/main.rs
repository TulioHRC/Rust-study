use std::collections::HashMap;

mod sort_algorithms;

fn main() {
    println!("Type the vector size.");
    let mut times = String::new();
    std::io::stdin()
        .read_line(&mut times)
        .expect("Failed to read line");
    
    let mut times: usize = times.trim().parse().expect("Failed to convert times.");

    let mut vec: Vec<isize> = Vec::new();
    vec.reserve(times);

    let mut usages: HashMap<isize, usize> = HashMap::new();
    let mut most_used_element: Option<(isize, usize)> = None;

    while times > 0 {
        let mut num = String::new();
        std::io::stdin()
            .read_line(&mut num)
            .expect("Failed to read number!");
        
        let num: isize = num.trim().parse().expect("Failed to convert number!");
        *usages.entry(num).or_insert(0) += 1;

        if let Some((_val, count)) = most_used_element {
            if let Some(&current_count) = usages.get(&num) {
                if current_count > count {
                    most_used_element = Some((num, current_count));
                }
            }
        } else {
            most_used_element = Some((num, 1));
        }

        vec.push(num);
        times -= 1;
    }

    sort_algorithms::bubble_sort(&mut vec);
    
    println!("Sorted vector: {:?}", vec);
    if let Some((val, count)) = most_used_element {
        println!("Most used element: {} with {} occurrences", val, count);
    }
}
