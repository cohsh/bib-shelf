use gtk::prelude::*;
use gtk::gio;

mod paper;
mod ui;
mod util;
mod bib;
use paper::Paper;
use bib::get_bib;
use util::mkdir;

fn main() {
    let application = gtk::Application::new(Some("com.github.cohsh.pdf-bib"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("PDF-bib"));

    window.set_default_size(1600, 900);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let model = gio::ListStore::new::<Paper>();

    let mut v_bib = get_bib("ref.bib".into());

    mkdir("data".to_string());
    for v in v_bib.iter_mut(){
        let path = "data/".to_string() + &v[3].clone();
        model.append(&Paper::new(
            v[0].clone(),
            v[1].clone(),
            v[2].clone(),
            path.clone(),
        ));
        mkdir(path);
    }

    let list_box = gtk::ListBox::new();
    list_box.bind_model(Some(&model), |item| {
        let paper = item.downcast_ref::<Paper>().unwrap();
        ui::display_ui(paper).upcast::<gtk::Widget>()
    });

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(900)
        .min_content_width(1600)
        .child(&list_box)
        .build();

    vbox.append(&scrolled_window);

    window.set_child(Some(&vbox));
    window.show();
}