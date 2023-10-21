use regex::Regex;

use crate::util::read;

pub fn get_bib(filename: String) {
    let mut s: String = read(filename.into());
    s.retain(|c| c != '\n');
    s.retain(|c| c != '{');
    s.retain(|c| c != '}');
    let mut v_string: Vec<&str> = s.split('@').collect();
    println!("{:?}", v_string);

    for s in v_string.iter_mut(){
        let mut v: Vec<&str> = s.split(',').collect();
        for item in v.iter_mut(){
            let re = Regex::new(r"^ *").unwrap();
            let mut item_re = re.replace(item, "");
            if item_re.contains("title") {
                println!("{:?}", item_re);
            }
            if item_re.contains("author") {
                println!("{:?}", item_re);
            }
            if item_re.contains("year") {
                println!("{:?}", item_re);
            }
        }
    }
}