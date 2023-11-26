use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{read, write};

#[allow(dead_code)]
pub struct DynamicArray<Type> { 
    pointer: *mut Type,
    size: u32,
    length: u32,
}

#[allow(dead_code)]
impl<Type> DynamicArray<Type> {
    pub fn new() -> Self {
        let initial_size: u32 = 2;
        let pointer: *mut Type = Self::allocate(initial_size);
        DynamicArray { pointer, size: initial_size, length: 0 }
    }

    pub fn length(&self) -> u32 { self.length as u32 }

    pub fn get(&self, index: u32) -> Type {
        if index < self.length { Self::read(self.pointer, index) }
        else { panic!("Invalid Index") }
    }

    pub fn set(&mut self, index: u32, value: Type) {
        if index < self.length { Self::write(self.pointer, index, value); }
        else { panic!("Invalid Index") }
    }

    pub fn push(&mut self, value: Type) {
        self.length += 1;
        self.set(self.length-1, value);
        self.check_length();
    }

    pub fn pop(&mut self) -> Type {
        if self.length > 0 {
            let value = self.get(self.length-1);
            self.length -= 1;
            self.check_length();
            value
        }
        else { panic!("Nothing in array to pop") }
    }

    pub fn insert_at(&mut self, index: u32, value: Type) -> bool {
        if index < self.length {
            self.length += 1;
            self.check_length();
            let mut i = self.length-1;
            while i > index {
                self.set(i, self.get(i-1));
                i -= 1;
            }
            self.set(index, value);
            true
        }
        else { false }
    }

    pub fn remove_at(&mut self, index: u32) -> bool{
        if index < self.length {
            self.check_length();
            let mut i: u32 = index;
            while i < self.length-1 {
                self.set(i, self.get(i+1));
                i += 1;
            }
            self.length -= 1;
            true
        }
        else { false }
    }

    pub fn copy(&self) -> DynamicArray<Type> {
        let mut new_array: DynamicArray<Type> = DynamicArray::new();
        for index in 0..self.length {
            new_array.push(self.get(index));
        }
        new_array
    }

    fn check_length(&mut self) {
        if self.length == self.size {
            self.grow();
        }
        if (self.length >= 8) && (self.length <= self.size/3) {
            self.shrink();
        }
    }

    fn allocate(size: u32) -> *mut Type {
        let layout: Layout = Layout::array::<Type>(size as usize).unwrap();
        unsafe{ alloc(layout) as *mut Type }
    }

    fn deallocate(pointer: *mut Type, size: u32) {
        let layout: Layout = Layout::array::<Type>(size as usize).unwrap();
        unsafe { dealloc(pointer as *mut u8, layout) };
    }

    fn read(pointer: *mut Type, index: u32) -> Type {
        unsafe { read::<Type>(pointer.offset(index as isize)) }
    }

    fn write(pointer: *mut Type, index: u32, value: Type) {
        unsafe { write(pointer.offset(index as isize), value) }
    }

    fn copy_contents(source: *mut Type,destination: *mut Type, count: u32) {
        for index in 0..count {
            let value: Type = Self::read(source, index);
            Self::write(destination, index, value)          
        }
    }

    fn reallocate(&mut self, new_size: u32) {
        let pointer: *mut Type = Self::allocate(new_size);
        Self::copy_contents(self.pointer, pointer, self.length);
        Self::deallocate(self.pointer, self.size);
        self.pointer = pointer;
        self.size = new_size;
    }

    fn grow(&mut self) {
        let new_size: u32 = self.size*2;
        self.reallocate(new_size);
    }

    fn shrink(&mut self) {
        let new_size: u32 = self.size/2;
        self.reallocate(new_size);
    }

}

impl<Type> Drop for DynamicArray<Type> {
    fn drop(&mut self) { Self::deallocate(self.pointer, self.size); }
}

#[allow(dead_code)]
impl<Type> DynamicArray<Type> where Type: std::fmt::Display {
    pub fn print(&self) {
        print!("[ ");
        for i in 0..self.length {
            print!("{}, ", self.get(i));
        }
        println!("]");
    }
}

#[allow(dead_code)]
impl<Type> DynamicArray<Type> where Type: PartialEq {
    pub fn find(&self, value: Type) -> i32 {
        for index in 0..self.length {
            if self.get(index) == value {
                return index as i32;
            }
        }
        return -1;
    }

    pub fn remove(&mut self, value: Type) -> bool{
        self.remove_at(self.find(value) as u32)
    }
}




