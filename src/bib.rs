use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::path::Path;

use crate::util::mkdir;

#[derive(Clone, Debug, Default)]
pub struct Bib {
    category: Option<String>,
    identifier: Option<String>,
    year: Option<u64>,
    author: Option<String>,
    title: Option<String>,
    text: Option<String>,
}

impl Bib {
    pub fn category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    pub fn identifier(&self) -> Option<&String> {
        self.identifier.as_ref()
    }

    pub fn year(&self) -> Option<u64> {
        self.year
    }

    pub fn author(&self) -> Option<&String> {
        self.author.as_ref()
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }

    pub fn set_category(&mut self, category: String) {
        self.category = Some(category);
    }

    pub fn set_identifier(&mut self, identifier: String) {
        self.identifier = Some(identifier);
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

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }

    pub fn is_not_empty(&self) -> bool {
        self.category.as_ref().filter(|category| !category.is_empty()).is_some() &&
        self.identifier.as_ref().filter(|identifier| !identifier.is_empty()).is_some() &&
        self.year.filter(|&year| year != 0).is_some() &&
        self.author.as_ref().filter(|author| !author.is_empty()).is_some() &&
        self.title.as_ref().filter(|title| !title.is_empty()).is_some() &&
        self.text.as_ref().filter(|text| !text.is_empty()).is_some()
    }
}

pub fn get_bibs(text: String) -> Vec<Bib> {
    let mut bibs: Vec<Bib> = Vec::new();

    let mut v_string: Vec<&str> = text.split('@').collect();

    for s in v_string.iter_mut(){

        let text = "@".to_string() + s;
        let bib = extract(text);

        if let Ok(bib) = bib {
            if bib.is_not_empty() {
                bibs.push(bib);
            }    
        } else {
            eprintln!("Error while extracting Bib: {:?}", bib);
        }
    }
    bibs
}

pub fn get_bibs_first() -> Vec<Bib> {
    let mut bibs: Vec<Bib> = Vec::new();

    let _ = mkdir("./library".to_string());
    // Ref. http://exlight.net/tutorial/bibtex-category.html
    let subdirs = [
        "article", "inproceedings", "phdthesis", "masterthesis", "book", "incollection",
        "inbook", "booklet", "manual", "proceedings", "techreport", "unpublished", "misc",
        ];

    for subdir in subdirs.iter() {
        let dir_path = format!("./library/{}", subdir);
        match mkdir(Path::new(&dir_path)) {
            Ok(_) => println!("Made directory {}", dir_path),
            Err(e) => eprintln!("Error mkdir {:?}: {}", dir_path, e),
        }

        if let Ok(dirs) = fs::read_dir(&dir_path) {
            for dir_entry in dirs {
                let dir_entry = match dir_entry {
                    Ok(entry) => entry,
                    Err(e) => {
                        eprintln!("Error reading directory entry: {}", e);
                        continue;
                    }    
                };
                let path = dir_entry.path();

                if path.is_dir() {
                    continue;
                }

                let file_stem = match path.file_stem().and_then(|s| s.to_str()) {
                    Some(stem) => stem,
                    None => {
                        eprintln!("Invalid file name: {:?}", path);
                        continue;
                    }
                };

                let file_bib = Path::new(&dir_path).join(format!("{}.bib", file_stem));

                let text = match fs::read_to_string(&file_bib) {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Error reading file {:?}: {}", file_bib, e);
                        continue;
                    }
                };

                let bib = extract(text);

                if let Ok(bib) = bib {
                    if bib.is_not_empty() {
                        bibs.push(bib);
                    }
                } else {
                    eprintln!("Error while extracting Bib: {:?}", bib);
                }
            }
        } else {
            eprintln!("Error reading directory {}: No such directory", dir_path);
        }
    }
    bibs
}

fn extract_field<'t>(text: &'t str, pattern: &Regex, capture_group_index: usize) -> Option<&'t str> {
    pattern.captures(text).and_then(|cap| cap.get(capture_group_index).map(|m| m.as_str()))
}

fn extract(text: String) -> Result<Bib, Box<dyn std::error::Error>> {
    let mut bib = Bib::default();

    let cleaned_text = text
        .replace("\r", "")
        .replace("\'", "")
        .replace("\t", "")
        .replace("\"", "")
        .replace("\\", "");

    bib.set_text(cleaned_text.clone());

    let identifier_pattern = Regex::new(concat!(
        r"@(?i)(article|inproceedings|phdthesis|masterthesis|",
        r"book|incollection|inbook|booklet|manual|",
        r"proceedings|techreport|unpublished|misc)\{(\S*)"
    ))?;

    if let Some(category) = extract_field(&cleaned_text, &identifier_pattern, 1) {
        bib.set_category(category.to_string());
    }


    let identifier_pattern = Regex::new(r"@article\{(\w*)")?;
    if let Some(identifier) = extract_field(&cleaned_text, &identifier_pattern, 2) {
        bib.set_identifier(identifier.to_string());
    }

    let year_pattern = Regex::new(r"year *= *([0-9]+)")?;
    if let Some(year_str) = extract_field(&cleaned_text, &year_pattern, 1) {
        let year = u64::from_str(year_str)?;
        bib.set_year(year);
    }

    let author_pattern = Regex::new(r"author *= *([^,\n]+)")?;
    if let Some(author) = extract_field(&cleaned_text, &author_pattern, 1) {
        bib.set_author(author.trim().to_string());
    }

    let title_pattern = Regex::new(r"title *= *([^,\n]+)")?;
    if let Some(title) = extract_field(&cleaned_text, &title_pattern, 1) {
        bib.set_title(title.trim().to_string());
    }

    Ok(bib)
}

fn shorten(s: &str, n_max: usize) -> String {
    if n_max == 0 {
        return String::new();
    } else if n_max <= 3 {
        return s.chars().take(n_max).collect();
    }

    if s.chars().count() <= n_max {
        s.to_string()
    } else {
        let shortened: String = s.chars().take(n_max - 3).collect();
        format!("{}...", shortened)
    }
}