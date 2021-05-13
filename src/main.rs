use minreq;

fn main() -> Result<(), minreq::Error> {

    let mut contentHolder = manga::Contents::new();

    println!("{:#?}", contentHolder);

    Ok(())
}

mod manga {
    use std::fmt;
    pub struct Contents{
        title: String,
        description: String
    }

    impl fmt::Debug for Contents{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "title:\n {}\n, description:\n{}", self.title, self.description)
        }
    }
    
    impl Contents{
        pub fn new() -> Contents {
            Contents { 
                title: String::new(), 
                description: String::new()
            }
        }

        #[allow(dead_code)]
        pub fn set(&mut self, t: String, d: String){
            self.title = t;
            self.description = d;
        }
    }
}