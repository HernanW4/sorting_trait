// Fast and simple sorting trait for a list
pub trait Sort{
    type Item;
    fn bubble_sorting(&mut self)-> &mut [Self::Item];
    fn swap(&mut self, index: usize, index2: usize);
    

}
//TODO:
// QuickSort


impl<T:PartialOrd + Copy> Sort for Vec<T>{
    type Item = T;
    fn bubble_sorting(&mut self) -> &mut [Self::Item] {
        let len = self.len();
        for _items in 0..len{
            for y in 0..len-1{
                if &self[y] > &self[y+1] {
                    self.swap(y, y+1);
                }
            }
        }

        return self;
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

       
        let new_list = Sort::bubble_sorting(&mut unsorted);

        assert_eq!(new_list,sorted);

        
        
    }

}
