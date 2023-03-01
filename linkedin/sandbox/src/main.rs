// #[derive(Debug, PartialEq)]
// enum Friends {
//     Fred,
//     Marge,
// }

fn main() {
    // use Friends::*;

    // let v = vec![Fred, Marge, Fred, Fred];

    // let num_freds = v.iter().filter(|&n| *n == Friends::Fred).count();
    // println!("The number of Freds is {}", num_freds);
    let num_aces: usize = 4;
    for element in 1..=num_aces {
        println!("{:?}", element);
    }
    // 1
    // 2
    // 3
    // 4
}
