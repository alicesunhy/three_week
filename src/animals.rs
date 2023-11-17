pub trait Eat {
    fn eat(&self);
}

#[derive(Debug)]
pub struct Rabbit;

#[derive(Debug)]
pub struct Cat;

#[derive(Debug)]
pub struct Dog;

impl Eat for Rabbit {
    fn eat(&self) {
        println!("Rabbit eat......");
    }
}

impl Eat for Cat {
    fn eat(&self) {
        println!("Cat eat......");
    }
}

impl Eat for Dog {
    fn eat(&self) {
        println!("Dog eat......");
    }
}

#[warn(dead_code)]
pub fn eat<T: Eat>(eat: T) {
    eat.eat();
}

#[derive(Debug)]
pub enum AnimalsType {
    Rabbit(Rabbit),
    Cat(Cat),
    Dog(Dog),
}
