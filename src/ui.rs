use crate::Spine;
use gtk::{
    prelude::*,
    CssProvider,
};
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


    let provider = CssProvider::new();
    provider.load_from_data("box { background-color: #CCDE68; }");
    let style_context = hbox.style_context();

    let path_str = spine.path();
    let path = Path::new(&path_str);

    if path.is_file() {
        style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
        // let green_icon = gtk::Image::from_file("assets/icons/green.svg");
        // hbox.append(&green_icon);
    } else {
        // let cream_icon = gtk::Image::from_file("assets/icons/cream.svg");
        // hbox.append(&cream_icon);
    }

    hbox
}