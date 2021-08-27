pub struct SortingConfig <'a,T>{ //struct to put store the unsorted list
    slice: &'a mut [T]
}


impl<'a,T: PartialOrd + Copy> SortingConfig<'a,T> {
    pub fn new(slice: &'a mut [T])->Option<SortingConfig<'a,T>>{//constructor
       Some(SortingConfig{slice})

        

    }

   

    pub fn bubble_sorting(&mut self)->& mut [T]{ // sorts the unsorted list by bubble sort
        let len = self.slice.len();
        for _items in 0..len{
            for y in 0..len-1{
                if &self.slice[y] > &self.slice[y+1] {
                    self.swap(y, y+1);
                }
            }
        }

        return self.slice;

    }
    pub fn swap(&mut self, index: usize, index2: usize){ //swaps two indexes
        let temp = self.slice[index];
        self.slice[index] = self.slice[index2];
        self.slice[index2] = temp;

        

    }

    pub fn get_slice(self)->&'a mut [T]{
        return self.slice;
    }

    
        
    
}



//TODO:
// QuickSort





#[cfg(test)]
mod tests {//TESTS
    use super::*;

    #[test]
    fn buble_works() {
        let mut unsorted = vec![5,2,4,1,3];
        let sorted = vec![1,2,3,4,5];

        let mut config  = SortingConfig::new(&mut unsorted).unwrap();
        let result = config.bubble_sorting();

        assert_eq!(result, sorted);
    }

    #[test]
    fn quick_sort_works(){
        todo!();
    }

    #[test]
    fn character_sort(){
        let mut unsorted = vec!['f','b','a','c'];
        let sorted = vec!['a','b','c','f'];

        let mut config = SortingConfig::new(&mut unsorted)
            .unwrap_or_else(||{
            panic!("There was an error at setting up the slice");
        });
        let result = config.bubble_sorting();

        assert_eq!(result,sorted);

    }
}
