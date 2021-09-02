// Fast and simple sorting trait for a list
pub trait Sort{
    fn bubble_sorting(&mut self);
    fn swap(&mut self, index: usize, index2: usize);
    

}


impl<T:PartialOrd + Copy> Sort for Vec<T>{
    fn bubble_sorting(&mut self){
        let len = self.len();
        for _items in 0..len{
            for y in 0..len-1{
                if &self[y] > &self[y+1] {
                    self.swap(y, y+1);
                }
            }
        }

        
    }

    fn swap(&mut self, index: usize, index2: usize) {
        let temp = self[index];
        self[index] = self[index2];
        self[index2] = temp;
    }

}


#[cfg(test)]
mod tests {//TESTS
    use super::*;

    #[test]
    fn buble_works() {
        let mut unsorted = vec![5,2,4,1,3];
        let sorted = vec![1,2,3,4,5];

       
        Sort::bubble_sorting(&mut unsorted);

        assert_eq!(unsorted,sorted);

        
        
    }

}
