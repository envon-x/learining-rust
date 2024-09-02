use std::{str::FromStr, vec};

mod media;
mod trait_inheritance;
use media::{Audio, Playable};
use trait_bounds_functions::{Apple, Food};
use trait_bounds_intro_fixed::{Enemy, Game, Hero};
use trait_inheritance::{TeslaRoadster, Vehicle};
use trait_objects::Square;

mod trait_bounds_intro_fixed;

mod trait_bounds_functions;
struct Video(String);

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}


fn main() {
  println!("Hello, world!");

  let audio = Audio {
    name: String::from("ambient_music.mp3")
  };
  // let audio = Audio("ambient_music.mp3".to_string());

  let video = Video("big_buck_bunny.mkv".to_string());

  audio.play();
  video.play();
  
  // let music = Audio::pause();

  let my_roadster = TeslaRoadster::new("Tesla Roadster II", 2020);
  println!("{} is priced at ${}", my_roadster.model, my_roadster.get_price())

  // Section: Using traits with generics – trait bounds
  let game = Game;
  game.load(Enemy);
  game.load(Hero);



  // 
  // let apple = Food(Apple);
  // eat(apple);




}

// mod trait_objects;