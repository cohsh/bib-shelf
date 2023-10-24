mod imp;
use gtk;
use gtk::glib;

glib::wrapper! {
    pub struct Paper(ObjectSubclass<imp::Paper>);
}

impl Paper {
    pub fn new(year: String, author: String, title: String, path: String) -> Self {
        let obj = glib::Object::new::<Paper>();
        obj.set_year(year);
        obj.set_author(author);
        obj.set_title(title);
        obj.set_path(path);
        obj
    }
}