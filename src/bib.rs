use regex::Regex;

use crate::util::read;

pub fn get_bib(filename: String) -> Vec<[String; 3]>{
    let mut s: String = read(filename.into());
    s.retain(|c| c != '{');
    s.retain(|c| c != '}');
    let mut v_string: Vec<&str> = s.split('@').collect();

    let mut v_bibs: Vec<[String; 3]> = Vec::new();

    for s in v_string.iter_mut(){
        let mut v_bib: [String; 3] = Default::default();
        let mut v: Vec<&str> = s.split('\n').collect();
        for item in v.iter_mut(){
            let item_re = Regex::new(r"^ *|,$").unwrap().replace_all(item, "");
            if item_re.contains("title") {
                v_bib[0] = item_re.replace("title=", "");
            }
            if item_re.contains("author") {
                v_bib[1] = item_re.replace("author=", "");
            }
            if item_re.contains("year") {
                v_bib[2] = item_re.replace("year=", "");
            }
        }
        if (v_bib[0] != "") && (v_bib[1] != "") && (v_bib[2] != "") {
            v_bibs.push(v_bib);
        }
    }
    v_bibs
}