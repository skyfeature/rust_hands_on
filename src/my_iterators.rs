#![allow(dead_code)]

fn vector_iterator() {
    let arr = vec![3, 5, 6];

    let vec_iter = arr.iter();

    for item in vec_iter {
        println!("{}", item);
    }

    println!("{:?}", arr);
}

fn iterate_to_get_values() {
    let arr = vec![3, 5, 6];

    let mut arr_iter = arr.iter();
    assert_eq!(arr_iter.next(), Some(&3));
    assert_eq!(arr_iter.next(), Some(&5));
    assert_eq!(arr_iter.next(), Some(&6));
}

fn iterator_sum() {
    let v1 = vec![3, 6, 2];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // sum consumes the iterator. v1_iter is no longer available after this line.

    assert!(total == 11);
}

fn iterator_adaptor() {
    let v1 = vec![1, 2, 3];
    let mut v1_new_iter = v1.iter().map(|x| 2 * x);

    assert_eq!(v1_new_iter.next(), Some(2));
    assert_eq!(v1_new_iter.next(), Some(4));
    assert_eq!(v1_new_iter.next(), Some(6));

    println!("{:?}", v1); // the underlying vector wasn't modified
}

fn iterator_adaptor_collect() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?} vs {:?}", v1, v2);
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn iterator_adaptor_capture_env() {
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}



pub fn entry_point() {
    vector_iterator();
    iterate_to_get_values();

    iterator_sum();

    iterator_adaptor();

    iterator_adaptor_collect();

    iterator_adaptor_capture_env();
}