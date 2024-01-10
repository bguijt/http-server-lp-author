#[derive(Debug)]
#[derive(PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Dutch,
}

#[derive(Debug)]
struct Greeting {
    message: String,
    lang: Lang,
}

struct Greetings {
  greetings: Vec<Greeting>,
}

impl Greetings {
  fn new() -> Greetings {
    Greetings {
      greetings: Vec::new()
    }
  }

  fn add(self: &mut Greetings, greeting: Greeting) {
    self.greetings.push(greeting);
  }

  fn find(self: &Greetings, lang: Lang) -> Result<&Greeting, String> {
    for g in &self.greetings {
      if g.lang == lang {
        return Ok(g);
      }
    }
    Err(String::from("Not found"))
  }

  fn all(self: &Greetings) -> &Vec<Greeting> {
    &self.greetings
  }
}

fn main() {
  let mut v: Greetings = Greetings::new();

  v.add(Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") });
  v.add(Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") });
  v.add(Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") });
  v.add(Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") });
  v.add(Greeting { lang: Lang::Dutch, message: String::from("Hallo WasmEdge!") });

  for e in v.all() {
    println!("{:?} {}", e.lang, e.message);
  }

  println!("Spanish: {:?}", v.find(Lang::Spanish).unwrap());
  println!("Spanish: {:?}", v.find(Lang::Spanish).unwrap());
  println!("Texan: {:?}", v.find(Lang::Texan).unwrap());
}
