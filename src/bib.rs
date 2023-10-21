use regex::Regex;

use crate::util::read;

pub fn get_bib(filename: String) -> Vec<[String; 5]>{
    let mut v_bibs: Vec<[String; 5]> = Vec::new();
    let mut s: String = read(filename.into());

    let mut v_string: Vec<&str> = s.split('@').collect();

    for s in v_string.iter_mut(){

        let mut v_bib: [String; 5] = Default::default();
        v_bib[4] = "@".to_string() + s;

        let mut v: Vec<&str> = s.split('\n').collect();

        let tmp0 = v[0];
        let tmp1 = Regex::new(r"^ *|,$").unwrap().replace_all(&tmp0, "");
        let tmp2 = Regex::new(r"(^article)").unwrap().replace_all(&tmp1, "${1}_");
        let tmp3 = Regex::new(r"([0-9]+)").unwrap().replace_all(&tmp2, "${1}_");
        v_bib[3] = (&tmp3).to_string();

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
        if (v_bib[0] != "") && (v_bib[1] != "") && (v_bib[2] != "") && (v_bib[3] != "") {
            v_bibs.push(v_bib);
        }
    }
    v_bibs
}