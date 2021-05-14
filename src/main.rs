fn main(){
    manga::run();
}

mod manga {
    use std::collections::HashMap;
    use serde_json::{Value};
    use serde::Deserialize;
    use std::collections::LinkedList;

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

        let chapter_links: LinkedList<String> = get_chapters_id(rel_val);

        let mut hasher: HashMap<String, String> = HashMap::new();
        let mut baser: HashMap<String, String> = HashMap::new();
        
        for i in chapter_links.iter() {
            let copy_hash: HashMap<String, String> = get_hash_id_map(i.to_string())?;
            for (key, val) in copy_hash.iter() {
                hasher.insert(key.to_string(), val.to_string());
            }
        };

        for (key, val) in hasher.iter() {
            let mut base_url: String = get_base_url(key.to_string())?.to_string().to_owned();
            base_url.push_str(val);

            base_url = base_url.replace('"', "");

            println!("{}", base_url);

            let data: String = key.to_owned();
            let mut static_url: String = "https://api.mangadex.org/chapter/".to_owned();

            static_url.push_str(&data);

            baser.insert(static_url, base_url);
        }

        println!("End of hashcode Code");

        for (id, base) in baser.iter() {
            get_chapters(id.to_string());
            break;
        }

        Ok(())
    }

    pub fn get_chapters_id(data: String) -> LinkedList<String>{
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

    fn get_base_url(id: String) -> Result<String, Box<dyn std::error::Error>>{
        let mut url: String = "https://api.mangadex.org/at-home/server/".to_owned();
        url.push_str(&id);

        let res = reqwest::blocking::get(url)?;
        let body = res.text()?;

        let v: Value = serde_json::from_str(&body[..])?;

        let mut base_url: String = v["baseUrl"].to_string().to_owned();
        base_url.push_str("/data/");

        Ok(base_url)
    }

    fn get_chapters(c: String) {
        let res = reqwest::blocking::get(c).unwrap();
        let body = res.text().unwrap();

        let v: Value = serde_json::from_str(&body).unwrap();

        println!("{:#?}", v["data"]["attributes"]["data"]);
    }
}

//Get the chapters which come as an array in a string use a vec<struct> to map content