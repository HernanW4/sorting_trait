mod sort;

use rand::Rng;
use sort::Sort;

fn main() {
    println!("Hello World");

    let mut unsorted = Vec::new();

    random_i32_list(&mut unsorted);

    println!("Unsorted: {:?}", &unsorted);

    Sort::bubble_sorting(&mut unsorted);

    println!("Sorted: {:?}", &unsorted);

    let mut unsorted = vec!['f', 'h', 'y', 't', 'i', 'p', 'q', 'g', 'a', 'c'];

    println!("Unsorted: {:?}", unsorted);

    Sort::bubble_sorting(&mut unsorted);

    println!("Sorted: {:?}", &unsorted);
}

fn random_i32_list(list: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();

    let len = rng.gen_range(5..20);

    for _x in 0..len {
        let random = rng.gen_range(0..1000);

        list.push(random);
    }
}
