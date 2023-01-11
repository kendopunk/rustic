/**
 * Unique Items
 * Filter duplicates from Vec<i32>
 * implement the unique() function which accepts a Vec<i32> and returns a de-duped Vec<i32>
 * Use a generic so the unique function accepts a Vec<T> and T implements Ord
 */

fn main() {
    let empty: Vec<i32> = vec![];
    assert_eq!(unique(empty), vec![]);

    let v1 = vec![1, 2, 3, 4, 5];
    assert_eq!(unique(v1.clone()), v1);

    let v2 = vec![1, 1, 2, 2, 3, 3, 100];
    assert_eq!(unique(v2.clone()), vec![1, 2, 3, 100]);

    println!("All good.")
}

fn unique<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort();
    v.dedup();
    v
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![1, 23, 4];
    let expected: Vec<i32> = vec![1, 4, 23];
    let actual = unique(input);
    assert_eq!(expected, actual)
}

#[test]
fn i64_test() {
    let input: Vec<i64> = vec![1, 23, 4];
    let expected: Vec<i64> = vec![1, 4, 23];
    let actual = unique(input);
    assert_eq!(expected, actual)
}

#[test]
fn i16_test() {
    let input: Vec<i16> = vec![1, 23, 4];
    let expected: Vec<i16> = vec![1, 4, 23];
    let actual = unique(input);
    assert_eq!(expected, actual)
}

// fn unique(v: &Vec<i32>) -> Vec<i32> {
//     if v.is_empty() {
//         return vec![];
//     }

//     let mut new_vec: Vec<i32> = vec![];
//     for (_index, item) in v.into_iter().enumerate() {
//         if !new_vec.contains(item) {
//             new_vec.push(*item);
//         }
//     }

//     new_vec
// }
