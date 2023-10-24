use std::process::Command;
use std::cell::RefCell;
use std::rc::Rc;
use std::env;

use gtk::prelude::*;
use gtk::gio;
use gtk::glib;

mod paper;
mod ui;
mod util;
mod bib;

use paper::Paper;
use bib::{get_bib, get_bib_first, Bib};
use util::{mkdir, write};

#[derive(Debug)]
pub struct Shelf {
    model: gio::ListStore,
}

impl Default for Shelf {
    fn default() -> Self {
        let model = gio::ListStore::new::<Paper>();

        Self {
            model,
        }
    }
}

impl Shelf {
    pub fn add_papers(&mut self, mut v_bib: Vec<Bib>) {
        for v in v_bib.iter_mut(){
            let dir = "papers/".to_string() + v.identifier().unwrap_or(&String::new());
            mkdir(dir.clone());
            let path_pdf = dir.clone() + "/" + v.identifier().unwrap_or(&String::new()) + ".pdf";

            if let Some(year) = v.year() {
                if let Some(author) = v.author() {
                    if let Some(title) = v.title() {
                        let paper = Paper::new(year, author.clone(), title.clone(), path_pdf);
                        self.model.append(&paper);
                    }
                }
            }            

            let path_bib = dir + "/" + v.identifier().unwrap_or(&String::new()) + ".bib";
            write(path_bib, v.text().unwrap_or(&String::new()));
        }
    }

    pub fn model(&self) -> &gio::ListStore {
        &self.model
    }
}

fn main() {
    let application = gtk::Application::new(Some("com.github.cohsh.bib-shelf"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    window.set_title(Some("Bib Shelf"));
    window.set_default_size(1200, 1000);

    vbox.append(&item_name_box());

    let mut bib = Shelf::default();

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
    
                let result = if cfg!(target_os = "linux") {
                    if env::var("WSL_DISTRO_NAME").is_ok() {
                        Command::new("powershell.exe").args(&["/c", "start", &pdf_path]).spawn()
                    } else {
                        Command::new("xdg-open").arg(pdf_path).spawn()
                    }
                } else if cfg!(target_os = "macos") {
                    Command::new("open").arg(pdf_path).spawn()
                } else if cfg!(target_os = "windows") {
                    Command::new("explorer").arg(pdf_path).spawn()
                } else {
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS"))
                };
    
                if let Err(err) = result {
                    eprintln!("Failed to open PDF: {}", err);
                }
            }
        }
    }));
    
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(700)
        .min_content_width(1200)
        .child(&list_box)
        .build();

    vbox.append(&scrolled_window);

    let v_bib = get_bib_first();
    bib.add_papers(v_bib);

    let bib = Rc::new(RefCell::new(bib));

    let bib_label = gtk::Label::builder()
        .label("New bib(s)")
        .halign(gtk::Align::Start)
        .build();

    vbox.append(&bib_label);
    vbox.append(&input_box(bib));

    window.set_child(Some(&vbox));
    window.show();
}

fn input_box(bib: Rc<RefCell<Shelf>>) -> gtk::Box {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let text_view = gtk::TextView::builder()
        .editable(true)
        .width_request(1100)
        .height_request(300)
        .build();
    
    let new_button = gtk::Button::builder()
        .label("Add")
        .width_request(100)
        .build();

    new_button.connect_clicked(
        glib::clone!(@weak text_view, @strong bib => move |_| {
            let buffer = text_view.buffer();
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let t = buffer.text(&start, &end, false).to_string();

            let v_bib = get_bib(t);
            bib.borrow_mut().add_papers(v_bib);
        }),
    );

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(1100)
        .min_content_height(300)
        .child(&text_view)
        .build();

    hbox.append(&scrolled_window);
    hbox.append(&new_button);
    hbox
}

fn item_name_box() -> gtk::Box {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(20)
        .homogeneous(true)
        .build();
    
    let label_year = gtk::Label::builder()
        .label("Year")
        .halign(gtk::Align::Start)
        .build();

    let label_author = gtk::Label::builder()
        .label("Author")
        .halign(gtk::Align::Start)
        .build();

    let label_title = gtk::Label::builder()
        .label("Title")
        .halign(gtk::Align::Start)
        .build();
    
    let label_pdf = gtk::Label::builder()
        .label("📚")
        .halign(gtk::Align::Start)
        .build();

    label_title.set_hexpand(true);
    label_author.set_hexpand(true);
    hbox.append(&label_year);
    hbox.append(&label_author);
    hbox.append(&label_title);
    hbox.append(&label_pdf);
    hbox
}