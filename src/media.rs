pub trait Playable {
  
  fn play(&self);

  // There are: 
  // - associated methods
  // - instance methods (selfs)
  //
  
  fn pause() {
    println!("Paused");
  }
}

// use crate::media::Playable;

pub struct Audio {
  pub name: String,
}

impl Playable for Audio {
  fn play(&self) {
    println!("Now playing: {}", self.name);
  }
  
  fn pause() {
    std::println!("Paused");
  }
}

