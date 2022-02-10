
use std::collections::HashMap;


#[macro_use]
extern crate better_anymap_derive;
pub use better_anymap_derive::Id;

pub trait Id{
    fn get_id() -> u32;
    fn get_instance_id(&self) -> u32;
}


pub struct AnyMap{
    data: HashMap<u32, Vec<u8>>
}

impl AnyMap {
    
    pub fn insert<'a, T: Id>(&'a mut self, item: T)
    {
        let v = vec![item];
        unsafe{
            let bits = v.align_to::<u8>().1;
            self.data.insert(T::get_id(), bits.try_into().expect("Unable to convert to Vec"));
        }
    }

    pub fn get<T: Id>(&self) -> Option<&T>
    {
        self.data.get(&T::get_id()).map(
            |v| {
                unsafe{
                    v.align_to::<T>().1.get(0).unwrap()
                }
            }
        )
    }

    pub fn get_mut<T: Id>(&mut self) -> Option<&mut T>{
        self.data.get_mut(&T::get_id()).map(
            |v|{
                unsafe {
                    v.align_to_mut::<T>().1.get_mut(0).unwrap()
                }
            }
        )
    }

    pub fn new() -> Self{
        Self{
            data: HashMap::new()
        }
    }

}


// Allows Vec<T> to be stored in anymap, it does however halve the possible items that can be stored. This should not be an issue for pretty much anyone though
// It is unlikely you will have more than 2^31 unique structs you wish to store
impl<T: Id> Id for Vec<T>{

    fn get_id() -> u32 {
        T::get_id() | 2147483648 // this number is the 32nd bit in u32 so 10000000....
    }

    fn get_instance_id(&self) -> u32 {
        T::get_id() | 2147483648 // this number is the 32nd bit in u32 so 10000000....
    }
}


// Convenience implementation for AnyMap with Vec<T> 
pub struct AnyVecMap{
    data: AnyMap
}

impl AnyVecMap {
    
    pub fn push<T: Id>(&mut self, value: T){
        if let Some(v) = self.data.get_mut::<Vec<T>>(){
            return v.push(value);
        }
        let mut v = Vec::new();
        v.push(value);
        self.data.insert(v);
    }

    pub fn pop<T: Id>(&mut self) -> Option<T>{
        if let Some(v) = self.data.get_mut::<Vec<T>>(){
            return v.pop();
        }
        None
    }

    pub fn new() -> Self{
        Self{
            data: AnyMap::new()
        }
    }

    

}





#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Id, PartialEq, Debug)]
    struct S1{
        x: u32
    }

    #[derive(Id, PartialEq, Debug)]
    struct S2{
        y: u32
    }

    #[derive(Id, PartialEq, Debug)]
    struct Empty;





    #[test]
    fn anymap_working() {
        let mut any_map = AnyMap::new();
        any_map.insert(S1{x: 24});
        any_map.insert(S2{y: 13});
        any_map.insert(Empty);

        let s1: Option<&S1> = any_map.get();

        //assert_eq!(any_map.get::<Empty>(), Some(&Empty));
        assert_eq!(any_map.get::<S1>(), Some(&S1{x:24}));

    }

    #[test]
    fn anyvecmap_working(){
        let mut any_vec_map = AnyVecMap::new();
        any_vec_map.push(S1{x: 24});
        any_vec_map.push(S1{x: 45});

        assert_eq!(any_vec_map.pop::<S1>(), Some(S1{x: 24}));
        assert_eq!(any_vec_map.pop::<S1>(), Some(S1{x: 45}));
        assert_eq!(any_vec_map.pop::<S1>(), None);
    }

    #[test]
    fn derive_id_workig(){

        let mut any_map = AnyMap::new();
        any_map.insert(S1{x: 24});
        any_map.insert(S2{y: 13});
        

        assert_ne!(S1::get_id(), S2::get_id());

        assert_ne!(any_map.get::<S1>().unwrap().get_instance_id(), any_map.get::<S2>().unwrap().get_instance_id());
    }

    // // Usure of how to write an actual unit test for something like this. If it fails to compile though then lifetime bounds are being respected.
    // #[test]
    // fn lifetime_sanity_check(){

    //     let mut r = None;
    //     {
    //         let mut any_map = AnyMap::new();
    //         any_map.insert(S1{x: 24});
    //         any_map.insert(S2{y: 13});

    //         r = any_map.get_mut::<S1>();
    //     }

    //     assert!(r.is_none());
    // }
}
