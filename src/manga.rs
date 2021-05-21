use std::collections::{HashMap, BTreeMap};
use serde_json::{Value};
use serde::Deserialize;

//Input
use std::io;

//Logging
use std::fs::OpenOptions;

//UI
use crate::tui;

#[derive(Debug, Deserialize)]
struct DataHolder {
    id: String,
    r#type: String
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {

    println!("Enter manga ID: ");

    let mut id = String::new();
    match io::stdin().read_line(&mut id) {
        Ok(n) => println!("ID: {} sucessfully recieved!", n),
        Err(e) => println!("Error: {}", e),
    }

    println!("Running API");
    let res = reqwest::get(format!("https://api.mangadex.org/manga/{}", id)).await?;
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

    let base_u: String = "https://api.mangadex.org/chapter/".to_string() + &c.to_string();

    let res = reqwest::blocking::get(base_u).unwrap();
    let body = res.text().unwrap();

    let v: Value = serde_json::from_str(&body).unwrap();

    let chaps: Vec<String> = v["data"]["attributes"]["data"].to_string().as_str()
        .split(",").map(|s| s.to_string().replace("[", "").replace("]", ""))
        .collect();

    for i in chaps.iter(){
        h.push(i.to_string());
    }
    }

pub fn download_pages(c: String){

    use std::io::prelude::*;
    use std::io::Cursor;
    use std::fs::File;

    let hash = get_hash(c.to_string()).unwrap();
    let base_url = get_base_url(String::from(&c)).unwrap();
    let mut pages: Vec<String> = vec![];

    get_pages(c, &mut pages);

    for page in pages.iter_mut() {
        *page = format!("{}{}/{}", base_url, hash, page).to_string().replace('"', "");
    }

    //Downloading
    for page in pages.iter(){
        let response = reqwest::blocking::get(page).unwrap();
        let mut file: File = File::create(format!("Downloads/{}", &page[page.len()-71..])).unwrap();

        let mut content =  Cursor::new(response.bytes().unwrap());
        std::io::copy(&mut content, &mut file).unwrap();
    }

    //Logging
    std::fs::write("log.txt", "").unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open("log.txt")
        .unwrap();

    for i in pages{
        file.write_all(i.as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
}