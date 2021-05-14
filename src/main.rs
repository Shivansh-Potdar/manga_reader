fn main(){
    manga::run();
}

mod manga {
    use std::collections::HashMap;
    use serde_json::{Value};
    use serde::Deserialize;
    use std::collections::LinkedList;

    #[derive(Default)]
    #[allow(dead_code)]
    pub struct Contents{
        id: String,
        title: String,
        description: String,
        hash: String,
        chapters: Vec<String>
    }

    /** impl fmt::Debug for Contents{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "title:\n {},\n description:\n{}\n", self.title, self.description);
            write!(f, "hash is yet to be set and so are the chapters")
        }
    } **/

    impl Contents{
        pub fn new() -> Contents {
            Contents::default()
        }

        pub fn set(&mut self, i: String, t: String, d: String, c: Vec<String>, h: String){
            self.id = i;
            self.title = t;
            self.description = d;
            self.chapters = c;
            self.hash = h;
        }
    }

    struct ChapterNeeds{
        id: String,
        hash: String,
    }

    #[derive(Debug, Deserialize)]
    struct DataHolder {
        id: String,
        r#type: String
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("Running API");
        let res = reqwest::blocking::get("https://api.mangadex.org/manga/random")?;
        let body = res.text()?;
    
        let v: Value = serde_json::from_str(&body[..])?;
        let rel_val: String = v["relationships"].to_string();

        let chapter_links: LinkedList<String> = get_chapters_list(rel_val);

        let mut hasher: HashMap<String, String> = HashMap::new();
        
        for i in chapter_links.iter() {
            let copy_hash: HashMap<String, String> = get_hash_id_map(i.to_string())?;
            for (key, val) in copy_hash.iter() {
                hasher.insert(key.to_string(), val.to_string());
            }
        };

        for (key, val) in hasher.iter() {
            println!("id_is: {} hash_is: {}", key, val);
        }

        println!("End of hashcode Code");

        Ok(())
    }

    pub fn get_chapters_list(data: String) -> LinkedList<String>{
        let v: Vec<DataHolder> = serde_json::from_str(data.as_str()).expect("You doof that ain't no JSON");
        let mut linked_list: LinkedList<String> = LinkedList::new();

        for t in v {
            match t.r#type.as_str() {
                "chapter" =>{
                    println!("{} contains chapter", t.id);
                    linked_list.push_back(t.id);
                },

                _ => println!("useless")
            }
        };
        println!("End of Chapter LinkList Code");
        linked_list
    }

    fn get_hash_id_map(c: String) ->  Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let data: String = c.to_owned();
        let mut static_url: String = "https://api.mangadex.org/chapter/".to_owned();

        static_url.push_str(&data);

        let res = reqwest::blocking::get(static_url)?;
        let body = res.text()?;

        let v: Value = serde_json::from_str(&body)?;

        let mut my_map: HashMap<String, String> = HashMap::new();

        my_map.insert(c, v["data"]["attributes"]["hash"].to_string());

        Ok(my_map)
    }

}