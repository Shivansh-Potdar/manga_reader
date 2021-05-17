#[macro_use]
extern crate lazy_static;

fn main(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run()).unwrap();
}

async fn run() -> Result<(), Box<dyn std::error::Error>>{
    manga::run().await?;
    Ok(())
}

mod manga {
    use std::collections::{HashMap, BTreeMap};
    use serde_json::{Value};
    use serde::Deserialize;

    use crate::tui;

    #[derive(Debug, Deserialize)]
    struct DataHolder {
        id: String,
        r#type: String
    }

    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        println!("Running API");
        let res = reqwest::get("https://api.mangadex.org/manga/random").await?;
        let body = res.text().await?;
    
        let v: Value = serde_json::from_str(&body[..])?;
        let rel_val: String = v["relationships"].to_string();
        let name_val: String = v["data"]["attributes"]["title"]["en"].to_string();

        let chapter_links: BTreeMap<String, u32> = get_chapters_id_name(rel_val).await;

        //insert pages here
        let mut _pages: HashMap<String, Vec<String>> = HashMap::new();

        tui::run_load_list(chapter_links, name_val);

        Ok(())
    }

    async fn get_chapters_id_name(data: String) -> BTreeMap<String, u32>{
        let v: Vec<DataHolder> = serde_json::from_str(data.as_str()).expect("You doof that ain't no JSON");
        let mut b_tree_map: BTreeMap<String , u32> = BTreeMap::new();

        let client = reqwest::Client::new();

        for t in v {
            match t.r#type.as_str() {
                "chapter" =>{
                    let uri: String = "https://api.mangadex.org/chapter/".to_owned();
                    let req_url = uri + &t.id;
                    let res = client.get(req_url).send().await.unwrap();

                    let v: Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
                    let chap_num: String = v["data"]["attributes"]["chapter"].to_string();
                    //chap_num.push_str(&v["data"]["attributes"]["title"].to_string());
                    let mut num: String = chap_num.replace('"', "");
                    let num_ref = &num;
                    if num_ref.contains("."){
                        println!("sefu desu");
                        b_tree_map.insert((&t.id).to_string(), num.parse::<f64>().unwrap() as u32);
                    } else {
                        num.to_string().push_str(&".".to_owned());
                        b_tree_map.insert((&t.id).to_string(), (&mut *num).parse::<f64>().unwrap() as u32);
                    }
                    
                    println!("{}", t.id);
                },

                _ => println!("useless")
            }
        };

        println!("End of Chapter BTree Code");
        b_tree_map
    }

    fn get_hash(c: String) ->  Result<String, Box<dyn std::error::Error>> {
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
    use cursive::align::HAlign;
    use cursive::event::EventResult;
    use cursive::traits::*;
    use cursive::views::{Dialog, OnEventView, SelectView, TextView};
    use cursive::Cursive;

    use std::collections::BTreeMap;
    use mut_static::MutStatic;

    // We'll use a SelectView here.
    //
    // A SelectView is a scrollable list of items, from which the user can select
    // one.

    pub struct ValHolder {
        value: BTreeMap<String, u32>
    }

    impl ValHolder {
        pub fn set(val: BTreeMap<String, u32>) -> Self{
            ValHolder{
                value: val
            }
        }
    }

    lazy_static!{
        static ref MATCH_AGAINST: MutStatic<ValHolder> = {
            MutStatic::new()
        };
    }
    
    pub fn run_load_list(v: BTreeMap<String, u32>, name: String) {
        println!("Loading the interface");
        let mut select = SelectView::new()
            // Center the text horizontally
            .h_align(HAlign::Center)
            // Use keyboard to jump to the pressed letters
            .autojump();

        let mut hash_vec: Vec<(&String, &u32)> = v.iter().collect();
        hash_vec.sort_by(|a, b| a.1.cmp(b.1));

        let chap_nums = itertools::Itertools::sorted(v.values()).map(|s| s.to_string());

        select.add_all_str(chap_nums);

        // Sets the callback for when "Enter" is pressed.
        select.set_on_submit(show_next_window);

        MATCH_AGAINST.set(ValHolder::set(v)).unwrap();

        // Let's override the `j` and `k` keys for navigation
        let select = OnEventView::new(select)
            .on_pre_event_inner('k', |s, _| {
                let cb = s.select_up(1);
                Some(EventResult::Consumed(Some(cb)))
            })
            .on_pre_event_inner('j', |s, _| {
                let cb = s.select_down(1);
                Some(EventResult::Consumed(Some(cb)))
            });

        let mut siv = cursive::default();

        // Let's add a ResizedView to keep the list at a reasonable size
        // (it can scroll anyway).
        siv.add_layer(
            Dialog::around(select.scrollable())
                .title(name)
                .button("Quit", |s| s.quit()),
        );

        siv.run();
    }

    // Let's put the callback in a separate function to keep it clean,
    // but it's not required.
    fn show_next_window(siv: &mut Cursive, chap: &str) {
        siv.pop_layer();
        let mut text: String = String::new();

        for (key, _val) in MATCH_AGAINST.read().unwrap().value.iter() {
            match MATCH_AGAINST.read().unwrap().value.get(key){
                Some(&value) => {
                    if value.to_string() == String::from(chap) {
                        text.push_str(key);
                    }
                },
                _ => {println!("yet to find val")}
            }
        }

        siv.add_layer(
            Dialog::around(TextView::new(text))
            .title("Amogus")
            .button("Quit", |s| s.quit()),
        );
    }
}

//show chapter name on next page
//split id from double chapters