#[derive(Debug)]
enum Food {
  Cake,
  Pizza,
  Salad,
}

#[derive(Debug)]
struct Bag {
  food: Food,
}

fn main() {
  let bag = Bag {
    food: Food::Cake,
  };

  match bag.food {
    Food::Cake => println!("I  got cake"),
    ref a => println!("I got {:?}", a)
  }

  match bag.food {
    Food::Pizza => println!("Tomé una pizza"),
    ref a => println!("I got {:?}",a)
  }
}