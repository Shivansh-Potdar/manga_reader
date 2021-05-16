fn main(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run()).unwrap();
}

async fn run() -> Result<(), Box<dyn std::error::Error>>{
    manga::run().await?;
    Ok(())
}

mod manga {
    use std::collections::HashMap;
    use serde_json::{Value};
    use serde::Deserialize;
    use std::collections::LinkedList;
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

        let chapter_links: HashMap<String, String> = get_chapters_id_name(rel_val).await;

        //insert pages here
        let mut _pages: HashMap<String, Vec<String>> = HashMap::new();

        tui::run_load_list(chapter_links, name_val);

        Ok(())
    }

    async fn get_chapters_id_name(data: String) -> HashMap<String, String>{
        let v: Vec<DataHolder> = serde_json::from_str(data.as_str()).expect("You doof that ain't no JSON");
        let mut linked_list: LinkedList<String> = LinkedList::new();
        let mut hash_map: HashMap<String , String> = HashMap::new();

        for t in v {
            match t.r#type.as_str() {
                "chapter" =>{
                    println!("{} contains chapter", t.id);
                    linked_list.push_back(t.id);
                },

                _ => println!("useless")
            }
        };

        for i in linked_list.iter() {
            let uri: String = "https://api.mangadex.org/chapter/".to_owned();
            let req_url = uri + &i;
            println!("{}", &req_url);
            let res = reqwest::get(req_url).await.unwrap().text().await.unwrap();

            let v: Value = serde_json::from_str(&res).unwrap();
            let chap_num: String = v["data"]["attributes"]["chapter"].to_string();

            println!("{}", chap_num);

            hash_map.insert(i.to_string(), chap_num);
        }

        println!("End of Chapter HashMap Code");
        hash_map
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

    // We'll use a SelectView here.
    //
    // A SelectView is a scrollable list of items, from which the user can select
    // one.

    pub fn run_load_list(v: std::collections::HashMap<String, String>, name: String) {
        println!("Loading the interface");
        let mut select = SelectView::new()
            // Center the text horizontally
            .h_align(HAlign::Center)
            // Use keyboard to jump to the pressed letters
            .autojump();

        select.add_all_str(v.values());

        // Sets the callback for when "Enter" is pressed.
        select.set_on_submit(show_next_window);

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
            Dialog::around(select.scrollable().fixed_size((20, 10)))
                .title(name)
                .button("Quit", |s| s.quit()),
        );

        siv.run();
    }

    // Let's put the callback in a separate function to keep it clean,
    // but it's not required.
    fn show_next_window(siv: &mut Cursive, chap: &str) {
        siv.pop_layer();
        let text = format!("{} is a great chapter!", chap);
        siv.add_layer(
            Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
        );
    }
}