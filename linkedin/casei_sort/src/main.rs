/**
 * case insensitive sort
 * Write a function that sorts usernames, ignoring the case of any letters within them
 * sort_usernames(&mt users)
 * sort in place
 * accept all unicode characters
 */
fn main() {
    let mut users = vec!["Todd", "amy", "Ñ", "Fred", "Jack", "adam", "@", "Å"];
    println!("Unsorted: {:?}", users);
    sort_usernames(&mut users);
    println!("Sorted: {:?}", users);
}

fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

#[test]
fn friend_names() {
    let mut friends = vec!["Fred", "Chad", "wopper", "aaron", "Rabbit"];
    sort_usernames(&mut friends);
    assert_eq!(friends, vec!["aaron", "Chad", "Fred", "Rabbit", "wopper"]);
}
