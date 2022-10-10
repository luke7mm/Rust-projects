struct Musician {
  name: String,
  age: u8,
  location: String,
  shot_a_man_down: bool,
}

fn main() {
  let Waylon = Musician {
    name: String::from("Waylon"),
    age: 64,
    location: String::from("Luckenbach, Texas"),
    shot_a_man_down: false,
  };

  let Willie = Musician {
    name: String::from("Willie"),
    age: 89,
    location: String::from("Abbott, Texas"),
    shot_a_man_down: false,
  };

  let Johnny = Musician {
    name: String::from("Johnny"),
    age: 21,
    location: String::from("Folsom Prison"),
    shot_a_man_down: true,
  };

  println!("{}", Johnny.age);
}