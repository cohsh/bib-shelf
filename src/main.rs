use std::process::Command;
use gtk::prelude::*;
use gtk::gio;

mod paper;
mod ui;
mod util;
mod bib;
use paper::Paper;
use bib::get_bib;
use util::{mkdir, write};

fn main() {
    let application = gtk::Application::new(Some("com.github.cohsh.pdf-bib"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("PDF-bib"));

    window.set_default_size(800, 600);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let model = gio::ListStore::new::<Paper>();

    let mut v_bib = get_bib("ref.bib".into());

    mkdir("data".to_string());
    for v in v_bib.iter_mut(){
        let dir = "data/".to_string() + &v[3].clone();
        mkdir(dir.clone());
        let path_pdf = dir.clone() + "/" + &v[3].clone() + ".pdf";
        model.append(&Paper::new(
            v[0].clone(),
            v[1].clone(),
            v[2].clone(),
            path_pdf,
        ));
        let path_bib = dir + "/" + &v[3].clone() + ".bib";
        write(path_bib, &v[4]);
    }

    let list_box = gtk::ListBox::new();
    list_box.bind_model(Some(&model), |item| {
        let paper = item.downcast_ref::<Paper>().unwrap();
        ui::display_ui(paper).upcast::<gtk::Widget>()
    });

    list_box.connect_row_activated(move |list_box, row| {
        let index = row.index();
        if let Some(item) = model.item(index as u32) {
            if let Some(paper) = item.downcast_ref::<Paper>() {
                let pdf_path = paper.path();
                println!("PDF path: {}", pdf_path);
    
                if let Err(err) = Command::new("open").arg(pdf_path).spawn() {
                    eprintln!("Failed to open PDF: {}", err);
                }
            }
        }
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