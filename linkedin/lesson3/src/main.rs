// linkedin/lesson3/src/main.rs
fn main() {
    ////////////////////////////////////////
    // simple arrays
    ////////////////////////////////////////
    let mut letters = ['a', 'b', 'c'];
    println!("first letter is {}", letters[0]);
    println!("second letter is {}", letters[1]);
    println!("third letter is {}", letters[2]);
    letters[1] = 'x';
    println!("second letter is now {}", letters[1]);

    ////////////////////////////////////////
    // uninitialized
    ////////////////////////////////////////
    let numbers: [i32; 5];
    numbers = [0, 0, 0, 0, 0];
    println!("{:?}", numbers);

    ////////////////////////////////////////
    // initialization with repeat
    ////////////////////////////////////////
    let nums: [i32; 5] = [22; 5];
    println!("{:?}", nums);

    ////////////////////////////////////////
    // 2 dimensional
    ////////////////////////////////////////
    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    println!("{:?}", parking_lot);
    let spot = parking_lot[0][1];
    println!("spot is {}", spot);

    ////////////////////////////////////////
    // 3 dimensional
    // 100 spots
    // 20 rows
    // 5 floors
    ////////////////////////////////////////
    // let garage: [[[i32; 100]; 20]; 5];
    let garage: [[[i32; 100]; 20]; 5] = [[[0; 100]; 20]; 5];
    let spot = garage[3][10][50];
    println!("spot is now {}", spot);

    ////////////////////////////////////////
    // initialize array with "range", the Rust way
    // using Box
    ////////////////////////////////////////
    let arr: Box<[i32; 5]> = (1..=5)
        .collect::<Box<[i32]>>()
        .try_into()
        .expect("wrong size iterator");
    println!("arr is {:?}", arr);

    ////////////////////////////////////////
    // tuples
    // fixed length, contiguous, dot notation
    ////////////////////////////////////////
    let mut stuff: (i32, f32, char, &str) = (10, 3.14, 'x', "fred");
    println!("{}", stuff.0);
    stuff.0 += 10;
    println!("{}", stuff.0);

    let (a, b, c, d) = stuff;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}
