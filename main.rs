mod dynamic_array;
use crate::dynamic_array::DynamicArray;

fn main() {
    let mut array: DynamicArray<u32> = DynamicArray::new();
    let amount: u32 = 8;
    for i in 0..amount { array.push(i); }
    for index in 0..array.length() {
        let item = array.get(index);
        println!("{}", item);
    }
    array.print();
}