use std::path::PathBuf;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct Spine(ObjectSubclass<imp::Spine>);
}

impl Spine {
    pub fn new(year: u64, author: String, title: String, path: PathBuf) -> Self {
        let obj = glib::Object::new::<Spine>();
        obj.set_year(year);
        obj.set_author(author);
        obj.set_title(title);
        obj.set_path(path);
        obj
    }
}