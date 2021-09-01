mod sort;
use core::panic;

use sort::{Sort};
use rand::Rng;


// impl<T:PartialOrd + Copy> Sort for Vec<T>{
//     type Item = T;
//     fn bubble_sorting(&mut self) -> &mut [Self::Item] {
//         let len = self.len();
//         for _items in 0..len{
//             for y in 0..len-1{
//                 if &self[y] > &self[y+1] {
//                     self.swap(y, y+1);
//                 }
//             }
//         }

//         return self;
//     }

//     fn swap(&mut self, index: usize, index2: usize) {
//         let temp = self[index];
//         self[index] = self[index2];
//         self[index2] = temp;
//     }

// }
fn main(){
    println!("Hello World");

    let mut unsorted = Vec::new();


    random_i32_list(&mut unsorted);

    

    println!("Unsorted: {:?}", &unsorted);
    
    let sorted = Sort::bubble_sorting(&mut unsorted);
    
    println!("Sorted: {:?}",&sorted);

    let mut unsorted = vec!['f','h','y','t','i','p','q','g','a','c','b'];

    println!("Unsorted: {:?}",unsorted);

    let sorted = Sort::bubble_sorting(&mut unsorted);

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
