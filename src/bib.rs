use std::fs;
use regex::Regex;

use crate::util::mkdir;

#[derive(Clone, Debug)]
pub struct Bib {
    year: Option<u64>,
    author: Option<String>,
    title: Option<String>,
    identifier: Option<String>,
    text: Option<String>,
}

impl Default for Bib {
    fn default() -> Self {
        Bib {
            year: None,
            author: None,
            title: None,
            identifier: None,
            text: None,
        }
    }
}

impl Bib {
    pub fn year(&self) -> Option<u64> {
        self.year
    }

    pub fn author(&self) -> Option<&String> {
        self.author.as_ref()
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    pub fn set_year(&mut self, year: u64) {
        self.year = Some(year);
    }

    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }

    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }

    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = Some(identifier);
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }

    pub fn is_not_empty(&self) -> bool {
        self.year.filter(|&year| year != 0).is_some() &&
        self.author.as_ref().filter(|author| !author.is_empty()).is_some() &&
        self.title.as_ref().filter(|title| !title.is_empty()).is_some() &&
        self.identifier.as_ref().filter(|identifier| !identifier.is_empty()).is_some() &&
        self.text.as_ref().filter(|text| !text.is_empty()).is_some()
    }
}

pub fn get_bib(text: String) -> Vec<Bib> {
    let mut v_bibs: Vec<Bib> = Vec::new();

    let mut v_string: Vec<&str> = text.split('@').collect();

    for s in v_string.iter_mut(){

        let text = "@".to_string() + s;
        let bib = extract(text);

        if bib.is_not_empty() {
            v_bibs.push(bib);
        }
    }
    v_bibs
}

pub fn get_bib_first() -> Vec<Bib> {
    let mut v_bibs: Vec<Bib> = Vec::new();

    mkdir("./papers".to_string());
    let dirs = fs::read_dir("./papers").unwrap();

    for dir_entry in dirs {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();

        let file_bib = path.join(format!("{}.bib", path.file_name().unwrap().to_str().unwrap()));
        let text = fs::read_to_string(file_bib).expect("Unable to read file");
        
        let bib = extract(text);

        if bib.is_not_empty() {
            v_bibs.push(bib);
        }
    }
    v_bibs
}

fn extract(text: String) -> Bib {
    let mut bib = Bib::default();

    let text = text.replace("\r", "").replace("\'", "").replace("\t", "").replace("\"", "").replace("\\", "");
    bib.set_text(text.clone());

    let text = text.replace("@", "");

    let mut v: Vec<&str> = text.split('\n').collect();

    let tmp0 = format!("{}", v[0]);
    let tmp1 = Regex::new(r"^ *|,|\{|\}|@$").unwrap().replace_all(&tmp0, "");
    let tmp2 = Regex::new(r"(^article)").unwrap().replace_all(&tmp1, "${1}_");
    let tmp3 = Regex::new(r"([0-9]+)").unwrap().replace_all(&tmp2, "${1}_");
    bib.set_identifier(format!("{}", (&tmp3).to_string()));

    for item in v.iter_mut(){
        let item_re = Regex::new(r"^ *|,$|\{|\}").unwrap().replace_all(item, "");
        if item_re.contains("year") {
            let tmp = Regex::new(r"year|=").unwrap().replace_all(&item_re, "");
            let tmp2 = Regex::new(r"^ *").unwrap().replace_all(&tmp, "");
            let tmp3: u64 = tmp2.parse().unwrap();
            bib.set_year(tmp3);
        }
        if item_re.contains("author") {
            let tmp = Regex::new(r"author|=").unwrap().replace_all(&item_re, "");
            let tmp2 = Regex::new(r"^ *").unwrap().replace_all(&tmp, "");
            bib.set_author(shorten(&tmp2.to_string(), 30).to_string());
        }
        if item_re.contains("title") {
            let tmp = Regex::new(r"title|=").unwrap().replace_all(&item_re, "");
            let tmp2 = Regex::new(r"^ *").unwrap().replace_all(&tmp, "");
            bib.set_title(shorten(&tmp2.to_string(), 50).to_string());
        }
    }
    bib
}

fn shorten(s: &str, n_max: usize) -> String {
    assert!(n_max > 0, "n_max must be greater than or equal to 0");

    let char_vec: Vec<char> = s.chars().collect();
    if char_vec.len() <= n_max {
        let tmp: String = char_vec.into_iter().collect();
        let result = format!("{:-width$}", tmp, width = n_max);
        result
    } else {
        let shortened: String = s.chars().take(n_max).collect();
        let result = format!("{:-width$}...", shortened, width = n_max - 3);
        result
    }
}