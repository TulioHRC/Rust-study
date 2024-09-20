fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    
    v.push(6); // Here works

    let first = &v[0];
    
    v.push(7); // Here the program will not compile, because first reference can be now wrong, if the memory needs to be re-allocated

    println!("The first element is: {first}");
}
