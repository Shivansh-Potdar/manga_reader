fn main(){
    manga::test();
}

mod manga {
    use std::fmt;
    use serde_json::{Value};
    use serde::Deserialize;

    #[derive(Default)]
    #[allow(dead_code)]
    pub struct Contents{
        id: String,
        title: String,
        description: String,
        hash: String,
        chapters: Vec<String>
    }

    #[derive(Debug, Deserialize)]
    pub struct DataHolder {
        id: String,
        r#type: String
    }


    struct ChapterNeeds{
        id: String,
        base_url: String,
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
        let rel_val: String = v["relationships"].to_string();

        Ok(())
    }

    pub fn test() {
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        [{"id":"de437bb9-e5cf-4820-acb9-dc9faadbd19b","type":"author"},{"id":"de437bb9-e5cf-4820-acb9-dc9faadbd19b","type":"artist"},{"id":"098f4023-4fcd-47b3-93bb-3e4f2e28fd88","type":"chapter"},{"id":"4d32cc48-9f00-4cca-9b5a-a839f0764984","type":"tag"},{"id":"aafb99c1-7f60-43fa-b75f-fc9502ce29c7","type":"tag"},{"id":"caaa44eb-cd40-4177-b930-79d3ef2afe87","type":"tag"}]"#;

        let v: Vec<DataHolder> = serde_json::from_str(data).expect("You doof that ain't no JSON");

        for t in v {
            println!("{}, {}", t.id, t.r#type);
            match t.r#type.as_str() {
                "chapter" => println!("has chapter"),

                _ => println!("useless")
            }
        }

    }

}