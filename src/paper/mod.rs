mod imp;
use gtk;
use gtk::glib;

glib::wrapper! {
    pub struct Paper(ObjectSubclass<imp::Paper>);
}

impl Paper {
    pub fn new(title: String, author: String, year: String) -> Self {
        let obj = glib::Object::new::<Paper>();
        obj.set_title(title);
        obj.set_author(author);
        obj.set_year(year);
        obj
    }
}