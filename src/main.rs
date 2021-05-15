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

        //insert pages here
        let mut _pages: HashMap<String, Vec<String>> = HashMap::new();

        Ok(())
    }

    fn get_chapters_id(data: String) -> LinkedList<String>{
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

    fn get_hash_id_map(c: String) ->  Result<String, Box<dyn std::error::Error>> {
        let data: String = c.to_owned();
        let mut static_url: String = "https://api.mangadex.org/chapter/".to_owned();

        static_url.push_str(&data);

        let res = reqwest::blocking::get(static_url)?;
        let body = res.text()?;

        let v: Value = serde_json::from_str(&body)?;

        Ok(v["data"]["attributes"]["hash"].to_string())
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

    /**
     * Takes id url from loop and gets the page numbers in a Vec<String>
     * return a Vec<String>
     * @Params String &mut HashMap<String, Vec<String>>
     * use after input from user
     */
    fn get_pages(c: String, h: &mut Vec<String>){
        let res = reqwest::blocking::get(&c).unwrap();
        let body = res.text().unwrap();

        let v: Value = serde_json::from_str(&body).unwrap();

        let chaps: Vec<String> = v["data"]["attributes"]["data"].to_string().as_str()
            .split(",").map(|s| s.to_string().replace("[", "").replace("]", ""))
            .collect();

        for i in chaps.iter(){
            h.push(i.to_string());
        }
    }
}

//add chaps name assets/chapter_list.txt
//maybe write all base urls with pages to .txt in order and just load em up

mod tui {

}