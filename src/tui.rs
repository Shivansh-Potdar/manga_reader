    //Tui libs
    use cursive::align::HAlign;
    use cursive::event::EventResult;
    use cursive::traits::*;
    use cursive::views::{Dialog, OnEventView, SelectView, TextView};
    use cursive::Cursive;

    //global variables and json
    use std::collections::BTreeMap;
    use mut_static::MutStatic;
    use serde_json::{Value};

    //threading
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;

    //manga crate init
    use crate::manga;

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

pub struct IdHolder {
    value: String
}

impl IdHolder {
    pub fn set(val: String) -> Self {
        IdHolder{
            value: val
        }
    }
}

lazy_static!{
    static ref MATCH_AGAINST: MutStatic<ValHolder> = {
        MutStatic::new()
    };

    static ref ID_STRING: MutStatic<IdHolder> = {
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

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, chap: &str) {
    siv.pop_layer();
    let mut id: String = String::new();

    for (key, _val) in MATCH_AGAINST.read().unwrap().value.iter() {
        match MATCH_AGAINST.read().unwrap().value.get(key){
            Some(&value) => {
                if value.to_string() == String::from(chap) {
                    id.push_str(key);
                    break;
                }
            },
            _ => {println!("yet to find val")}
        }
    }

    let mut text: String = String::new();
    let base_u = "https://api.mangadex.org/chapter/".to_string() + &id.to_string();

    let (sender, reciver) = mpsc::channel::<String>();

    let req_handler = thread::spawn(move ||{

        let res = reqwest::blocking::get(base_u).unwrap();

        let body = res.text().unwrap();
        let v: Value = serde_json::from_str(&body[..]).unwrap();

        text.push_str(&v["data"]["attributes"]["title"].to_string());

        //use thread messages ref: https://doc.rust-lang.org/book/ch16-02-message-passing.html

        sender.send(text).unwrap();
        thread::sleep(Duration::from_micros(10));
    });
    req_handler.join().expect("Thread couldn't be joined");

    ID_STRING.set(IdHolder::set(String::from(&id))).unwrap();

    siv.add_layer(
        Dialog::around(TextView::new(id))
        .title(reciver.recv().unwrap())
        .button("Download", show_download_page)
        .button("Quit", |s| s.quit()),
    );
}

fn show_download_page(siv: &mut Cursive){
    siv.pop_layer();

    let download_handler = thread::spawn(|| {
        let final_id: String = String::from(&ID_STRING.read().unwrap().value);
        manga::download_pages(final_id);
    });

    download_handler.join().expect("Error in downloading");

    siv.add_layer(
        Dialog::around(TextView::new("Downloaded Chapter"))
        .title("Download Page")
        .button("Quit", |s| s.quit()),
    );
}