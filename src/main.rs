mod lib;
use core::panic;

use lib::SortingConfig;
use rand::Rng;



fn main(){
    println!("Hello World");

    let mut unsorted = Vec::new();


    random_i32_list(&mut unsorted);

    

    println!("Unsorted: {:?}", &unsorted);
    let sorted = run(&mut unsorted)
        .unwrap_or_else(||{
            panic!("There was an error at i32 sorting ");
        });


    println!("Sorted: {:?}", &sorted);


    let mut unsorted = vec!['f','h','y','t','i','p','q','g','a','c','b'];

    println!("Unsorted: {:?}",unsorted);

    let sorted = run(&mut unsorted)
        .unwrap_or_else(||{
            panic!("There was an error at char sorting");
        });

    println!("Sorted: {:?}",sorted);
    

   
    
}

fn random_i32_list(list: &mut Vec<i32>){ //makes a random integer list
    let mut rng = rand::thread_rng();

    let len = rng.gen_range(5..20);

    for _x in 0..len{
        let random = rng.gen_range(0..1000);

        list.push(random);

    }
}
fn run <'a, T:PartialOrd + Copy + Sized>(unsorted: &'a mut Vec<T>)-> Option<&'a mut [T]>{// runs the program
    let mut config = SortingConfig::new(unsorted).unwrap();
    
    config.bubble_sorting();

    

    Some(config.get_slice())

    

}
