use gtk::prelude::*;
use gtk::gio;

mod paper;
mod ui;
mod util;
mod bib;
use paper::Paper;
use bib::get_bib;

fn main() {
    get_bib("test.bib".into());
    let application = gtk::Application::new(Some("com.github.cohsh.pdf-bib"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("PDF-bib"));
    window.set_default_size(1200, 600);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let model = gio::ListStore::new::<Paper>();

    model.append(&Paper::new(
        "Development of Hogehoge".into(),
        "Cat Schrodinger".into(),
        1925,
    ));
    model.append(&Paper::new(
        "Theory of Fugafuga".into(),
        "Fugafuga".into(),
        1185,
    ));

    let list_box = gtk::ListBox::new();
    list_box.bind_model(Some(&model), |item| {
        let paper = item.downcast_ref::<Paper>().unwrap();
        ui::display_ui(paper).upcast::<gtk::Widget>()
    });

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(400)
        .child(&list_box)
        .build();

    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));
    window.show();
}