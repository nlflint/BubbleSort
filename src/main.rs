use std::cmp::Ordering;

fn bubble_sort<T: PartialOrd>(mut collection :Vec<T>) -> Vec<T> {
    let mut swap_happened = false;
    loop {
        for index in 0..collection.len() - 1 {
            if collection[index].partial_cmp(&collection[index + 1]) == Some(Ordering::Greater) {
                collection.swap(index, index + 1);
                swap_happened = true;
            }
        }
        if !swap_happened {
            return collection;
        }
        swap_happened = false;
    }
}

#[test]
fn sort_collection_of_one() {
    let collection = vec!["apple".to_string()];
    let sorted_collection = bubble_sort(collection);

    assert!(sorted_collection == vec!["apple".to_string()]);
}

#[test]
fn sort_when_already_sorted() {
    let collection = vec!["apple".to_string(), "banana".to_string()];
    let sorted_collection = bubble_sort(collection);

    assert!(sorted_collection == vec!["apple".to_string(), "banana".to_string()]);
}

#[test]
fn sort_two_strings() {
    let collection = vec!["banana".to_string(), "apple".to_string()];
    let sorted_collection = bubble_sort(collection);

    assert!(sorted_collection == vec!["apple".to_string(), "banana".to_string()]);
}

#[test]
fn sort_many_strings() {
    let collection = vec!["banana".to_string(), "pineapple".to_string(), "pear".to_string(),"pinenut".to_string(),"apple".to_string()];
    let sorted_collection = bubble_sort(collection);

    assert!(sorted_collection == vec!["apple".to_string(), "banana".to_string(),"pear".to_string(), "pineapple".to_string(), "pinenut".to_string()]);
}
