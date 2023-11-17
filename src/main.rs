use animals::Eat;
mod animals;

fn main() {
    println!(
        "使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法"
    );

    let rabbit = animals::AnimalsType::Rabbit(animals::Rabbit);
    let cat = animals::AnimalsType::Cat(animals::Cat);
    let dog = animals::AnimalsType::Dog(animals::Dog);
    let fauna: Vec<animals::AnimalsType> = vec![rabbit, cat, dog];
    for value in &fauna {
        match value {
            animals::AnimalsType::Rabbit(rabbit) => rabbit.eat(),
            animals::AnimalsType::Cat(cat) => cat.eat(),
            animals::AnimalsType::Dog(dog) => dog.eat(),
        }
    }

    println!("");
    println!("定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法");

    let ra = animals::Rabbit;
    let ca = animals::Cat;
    let dg = animals::Dog;
    let list: Vec<&dyn animals::Eat> = vec![&ra, &ca, &dg];
    for value in &list {
        value.eat();
    }
}
