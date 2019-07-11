use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

fn BubbleSort<T: PartialOrd>(collection :Vec<T>) -> Vec<T> {
    return collection;
}

#[test]
fn sort_collection_of_one() {
    let collection = vec!["blah".to_string()];
    let sortedCollection = BubbleSort(collection);

    assert!(sortedCollection == vec!["blah".to_string()]);
}

#[test]
fn sort_when_already_sorted() {
    let collection = vec!["apple".to_string(), "banana".to_string()];
    let sortedCollection = BubbleSort(collection);

    assert!(sortedCollection == vec!["apple".to_string(), "banana".to_string()]);
}

#[test]
fn sort_two_strings() {
    let collection = vec!["banana".to_string(), "apple".to_string()];
    let sortedCollection = BubbleSort(collection);

    assert!(sortedCollection == vec!["apple".to_string(), "banana".to_string()]);
}
