/**
 * Write a function to calculate the median of a Vec<f32>
 * (1) sort
 * (2) odd number of elements = take middle one
 * (3) even number of elements = take average of middle two
 */
fn main() {
    let mut v1: Vec<f32> = vec![5.0, 10.0, 8.0, 12.0, 3.0];
    v1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(v1, [3.0, 5.0, 8.0, 10.0, 12.0]);
    assert_eq!(median(&v1), Some(8.0));

    let mut v2: Vec<f32> = vec![7.7, 3.5, 12.2, 11.1];
    v2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(v2, [3.5, 7.7, 11.1, 12.2]);
    assert_eq!(median(&v2), Some(9.4));

    let v3: Vec<f32> = vec![];
    assert_eq!(median(&v3), None);

    let mut v4: Vec<f32> = vec![1.0, 2.0, 10.0, 11.0, 55.0, 8.0];
    v4.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(median(&v4), Some(9.0));
}

fn median(v: &Vec<f32>) -> Option<f32> {
    if v.is_empty() {
        return None;
    }

    let length = v.len();
    let median = match length % 2 {
        0 => {
            let index2 = length / 2;
            let index1 = index2 - 1;
            (v[index1] + v[index2]) / 2.0
        }
        1 => {
            let index = (length - 1) / 2;
            v[index]
        }
        _ => unreachable!(),
    };

    Some(median)

    // if v.is_empty() {
    //     None
    // } else if length % 2 == 0 {
    //     let index2 = length / 2;
    //     let index1 = index2 - 1;
    //     Some((v[index1] + v[index2]) / 2.0)
    // } else {
    //     let index = (length - 1) / 2;
    //     Some(v[index])
    // }
}
