use std::process::Command;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::gio;
use gtk::glib;

mod paper;
mod ui;
mod util;
mod bib;
use paper::Paper;
use bib::get_bib;
use util::{mkdir, write};

#[derive(Debug)]
pub struct Bib {
    model: gio::ListStore,
}

impl Default for Bib {
    fn default() -> Self {
        let model = gio::ListStore::new::<Paper>();

        Self {
            model,
        }
    }
}

impl Bib {
    pub fn add_paper(&mut self, paper: &Paper) {
        self.model.append(paper);
    }

    pub fn model(&self) -> &gio::ListStore {
        &self.model
    }
}

fn main() {
    let application = gtk::Application::new(Some("com.github.cohsh.pdf-bib"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    window.set_title(Some("PDF-bib"));
    window.set_default_size(800, 600);

    let bib = Bib::default();

    let list_box = gtk::ListBox::new();
    list_box.bind_model(Some(bib.model()), |item| {
        let paper = item.downcast_ref::<Paper>().unwrap();
        ui::display_ui(paper).upcast::<gtk::Widget>()
    });

    let model = bib.model();

    list_box.connect_row_activated(glib::clone!(@weak model => move |_list_box, row| {
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
    }));    
    
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(900)
        .min_content_width(1600)
        .child(&list_box)
        .build();

    vbox.append(&scrolled_window);

    let bib = Rc::new(RefCell::new(bib));

    vbox.append(&input_box(bib));

    window.set_child(Some(&vbox));
    window.show();
}

fn input_box(bib: Rc<RefCell<Bib>>) -> gtk::Box {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let text = gtk::Entry::builder().placeholder_text("new .bib text(s)").build();
    
    let new_button = gtk::Button::builder().label("add").build();

    new_button.connect_clicked(
        glib::clone!(@weak text, @strong bib => move |_| {
            let t = text.buffer().text().to_string();

            let mut v_bib = get_bib(t);

            mkdir("data".to_string());
            for v in v_bib.iter_mut(){
                let dir = "papers/".to_string() + &v[3].clone();
                mkdir(dir.clone());
                let path_pdf = dir.clone() + "/" + &v[3].clone() + ".pdf";

                let paper = Paper::new(
                    v[0].clone(),
                    v[1].clone(),
                    v[2].clone(),
                    path_pdf,
                );
                bib.borrow_mut().add_paper(&paper);

                let path_bib = dir + "/" + &v[3].clone() + ".bib";
                write(path_bib, &v[4]);
            }
        }),
    );
    hbox.append(&text);
    hbox.append(&new_button);
    hbox
}