fn main() {
    let tup: (i32, bool, u8) = (500, true, 2);
    let (a,b,c) = tup;
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    let a: [i32; 5] = [1,2,3,4,5];
    let b = [3; 5]; // results in [3,3,3,3,3]
    let c: [i8; 5] = [3; 5];

    let d = a[0]; 

    let e = a[9]; // will cause an error
}
