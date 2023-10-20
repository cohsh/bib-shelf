use crate::Paper;
use gtk;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;

pub fn display_ui(paper: &Paper) -> impl IsA<gtk::Widget> {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .homogeneous(true)
        .build();

    let title = gtk::Label::builder().halign(gtk::Align::Start).build();
    paper
        .bind_property("title", &title, "label")
        .sync_create()
        .build();

    let author = gtk::Label::new(None);
    paper
        .bind_property("author", &author, "label")
        .sync_create()
        .build();

    let year = gtk::Label::builder().halign(gtk::Align::End).build();
    paper
        .bind_property("year", &year, "label")
        .sync_create()
        .build();
    
    hbox.append(&title);
    hbox.append(&author);
    hbox.append(&year);
    hbox
}