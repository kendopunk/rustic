fn main() {
    let mut list1 = vec![34, 50, 25, 100, 65, 8850];
    let mut list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    list1.append(&mut list2);

    let result = generic_largest(&list1);

    println!("The largest of the 2 arrays is {}", result);

    println!(
        "The largest in the character array is {}",
        generic_largest(&vec!['a', 'b', 'c'])
    )
}

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

/**
 * Defined as: "This function is generic over some type T and has a parameter of slice T and returns a reference to
 * a single T"
 */
fn generic_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}
