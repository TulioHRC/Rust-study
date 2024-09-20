enum Cell {
    Tipo1(i32),
    Tipo2(f64),
    Tipo3(String)
}

fn main() {
    let mut v = vec![
        Cell::Tipo1(1),
        Cell::Tipo2(4.5),
        Cell::Tipo3(String::from("teste"))
    ];

    v.push(Cell::Tipo2(8.1));

    for i in &v {
        match i {
            Cell::Tipo1(val) => {println!("{}",val)},
            _ => {}
        }
    }
}
