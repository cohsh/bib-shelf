use crate::Paper;
use gtk;
use gtk::prelude::*;

pub fn display_ui(paper: &Paper) -> impl IsA<gtk::Widget> {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(20)
        .homogeneous(true)
        .build();

    let year = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .build();
    paper
        .bind_property("year", &year, "label")
        .sync_create()
        .build();

    let author = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .build();
    paper
        .bind_property("author", &author, "label")
        .sync_create()
        .build();

    let title = gtk::Label::builder()
        .halign(gtk::Align::Start)
        .build();
    paper
        .bind_property("title", &title, "label")
        .sync_create()
        .build();


    title.set_hexpand(true);
    author.set_hexpand(true);
    hbox.append(&year);
    hbox.append(&author);
    hbox.append(&title);
    hbox
}