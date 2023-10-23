use std::fs;
use regex::Regex;

use crate::util::mkdir;

fn extract(text: String) -> [String; 5] {
    let mut v_bib: [String; 5] = Default::default();

    let text = text.replace("\r", "").replace("\'", "");

    v_bib[4] = text.clone();

    let mut v: Vec<&str> = text.split('\n').collect();

    let tmp0 = format!("{}", v[0]);
    let tmp1 = Regex::new(r"^ *|,|\{|\}|@$").unwrap().replace_all(&tmp0, "");
    let tmp2 = Regex::new(r"(^article)").unwrap().replace_all(&tmp1, "${1}_");
    let tmp3 = Regex::new(r"([0-9]+)").unwrap().replace_all(&tmp2, "${1}_");
    v_bib[3] = format!("{}", (&tmp3).to_string());

    for item in v.iter_mut(){
        let item_re = Regex::new(r"^ *|,$|\{|\}").unwrap().replace_all(item, "");
        if item_re.contains("title") {
            let tmp = Regex::new(r"title|=").unwrap().replace_all(&item_re, "");
            let tmp2 = Regex::new(r"^ *").unwrap().replace_all(&tmp, "");
            v_bib[0] = (&tmp2).to_string();
        }
        if item_re.contains("author") {
            let tmp = Regex::new(r"author|=").unwrap().replace_all(&item_re, "");
            let tmp2 = Regex::new(r"^ *").unwrap().replace_all(&tmp, "");
            v_bib[1] = (&tmp2).to_string();
        }
        if item_re.contains("year") {
            let tmp = Regex::new(r"year|=").unwrap().replace_all(&item_re, "");
            let tmp2 = Regex::new(r"^ *").unwrap().replace_all(&tmp, "");
            v_bib[2] = (&tmp2).to_string();
        }
    }
    v_bib
}

fn is_empty(v: [String; 5]) -> bool {
    (v[0] != "") && (v[1] != "") && (v[2] != "") && (v[3] != "")
}

pub fn get_bib(text: String) -> Vec<[String; 5]> {
    let mut v_bibs: Vec<[String; 5]> = Vec::new();

    let mut v_string: Vec<&str> = text.split('@').collect();

    for s in v_string.iter_mut(){

        let text = "@".to_string() + s;
        let v_bib: [String; 5] = extract(text);

        if is_empty(v_bib.clone()) {
            v_bibs.push(v_bib);
        }
    }
    v_bibs
}

pub fn get_bib_first() -> Vec<[String; 5]> {
    let mut v_bibs: Vec<[String; 5]> = Vec::new();

    mkdir("./papers".to_string());
    let dirs = fs::read_dir("./papers").unwrap();

    for dir_entry in dirs {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();
        println!("{:?}", path);

        let file_bib = path.join(format!("{}.bib", path.file_name().unwrap().to_str().unwrap()));
        let text = fs::read_to_string(file_bib).expect("Unable to read file");
        
        let v_bib: [String; 5] = extract(text);

        if is_empty(v_bib.clone()) {
            v_bibs.push(v_bib);
        }
    }

    v_bibs
}