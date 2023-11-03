use crate::Spine;
use gtk;
use gtk::prelude::*;
use std::path::Path;

pub fn display_ui(spine: &Spine) -> impl IsA<gtk::Widget> {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(20)
        .homogeneous(true)
        .build();

    let year = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .build();
    spine
        .bind_property("year", &year, "label")
        .sync_create()
        .build();

    let author = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .build();
    spine
        .bind_property("author", &author, "label")
        .sync_create()
        .build();

    let title = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .build();
    spine
        .bind_property("title", &title, "label")
        .sync_create()
        .build();

    title.set_hexpand(true);
    author.set_hexpand(true);
    hbox.append(&year);
    hbox.append(&author);
    hbox.append(&title);

    let path_str = spine.path();
    let path = Path::new(&path_str);

    if path.is_file() {
        let green_icon = gtk::Image::from_file("assets/icons/green.svg");
        hbox.append(&green_icon);
    } else {
        let cream_icon = gtk::Image::from_file("assets/icons/cream.svg");
        hbox.append(&cream_icon);
    }

    hbox
}