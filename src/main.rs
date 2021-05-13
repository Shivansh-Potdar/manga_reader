fn main(){
    manga::run();
}

mod manga {
    use std::fmt;
    use serde_json::{Value};

    #[derive(Default)]
    #[allow(dead_code)]
    pub struct Contents{
        title: String,
        description: String,
        hash: String,
        chapters: Vec<String>
    }

    impl fmt::Debug for Contents{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "title:\n {},\n description:\n{}\n", self.title, self.description);
            write!(f, "hash is yet to be set and so are the chapters")
        }
    }
    
    impl Contents{
        pub fn new() -> Contents {
            Contents::default()
        }

        #[allow(dead_code)]
        pub fn set(&mut self, t: String, d: String){
            self.title = t;
            self.description = d;
        }
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("Running API");
        let res = reqwest::blocking::get("https://api.mangadex.org/manga/random")?;
        let body = res.text()?;
    
        let v: Value = serde_json::from_str(&body[..])?;
    
        println!("{}", v["relationships"]);

        Ok(())
    }
}