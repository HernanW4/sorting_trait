// Fast and simple sorting trait for a list
pub trait Sort{
    type Item;
    fn bubble_sorting(&mut self)-> &mut [Self::Item];
    fn swap(&mut self, index: usize, index2: usize);
    

}

pub struct SortingConfig<'a,T:PartialOrd + Copy>{
    pub slice: &'a mut [T]
}


impl <'a, T:PartialOrd + Copy> SortingConfig<'a,T>{
    pub fn new(slice: &'a mut [T])-> SortingConfig<'a,T>{
        SortingConfig{slice}
    }
    
    pub fn get_slice(self)-> &'a mut [T]{
        self.slice
    }
}

impl <'a, T:PartialOrd + Copy> Sort for SortingConfig<'a, T>{
    type Item = T;

    fn bubble_sorting(&mut self)->& mut [T]{ // sorts the unsorted list by bubble sort
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
    fn swap(&mut self, index: usize, index2: usize){ //swaps two indexes
        let temp = self.slice[index];
        self.slice[index] = self.slice[index2];
        self.slice[index2] = temp;
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

        let mut config = SortingConfig::new(&mut unsorted);
        let new_list = Sort::bubble_sorting(&mut config);

        assert_eq!(new_list,sorted);

        
        
    }

}
