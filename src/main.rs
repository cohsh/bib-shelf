use std::{
    process::Command,
    cell::RefCell,
    rc::Rc,
    env,
    path::PathBuf,
};

use gtk::{
    prelude::*,
    gio,
    glib,
    CssProvider,
};

mod spine;
mod ui;
mod util;
mod bib;

use spine::Spine;
use bib::{get_bibs, get_bibs_first, Bib};
use util::{mkdir, write};

#[derive(Debug)]
pub struct Shelf {
    model: gio::ListStore,
}

impl Default for Shelf {
    fn default() -> Self {
        let model = gio::ListStore::new::<Spine>();

        Self {
            model,
        }
    }
}

impl Shelf {
    pub fn add_bibs(&mut self, bibs: Vec<Bib>) {
        for bib in bibs.iter() {
            if let Some(identifier) = bib.identifier() {
                let category = bib.category();

                let mut dir_path = PathBuf::from("library");
                if let Some(category) = category {
                    dir_path = dir_path.join(category);
                }
                dir_path = dir_path.join(identifier);
                
                if let Err(e) = mkdir(&dir_path) {
                    eprintln!("Failed to create directory {}: {}", dir_path.display(), e);
                }
    
                let path_pdf = dir_path.join(format!("{}.pdf", identifier));
    
                if let (Some(year), Some(author), Some(title)) = (bib.year(), bib.author(), bib.title()) {
                    let spine = Spine::new(year, author.clone(), title.clone(), path_pdf);
                    self.model.append(&spine);
                    println!("Identifier: {}", identifier);
                } else {
                    eprintln!("Missing required fields for bib: {}", identifier);
                }
    
                let path_bib = dir_path.join(format!("{}.bib", identifier));
                match write(&path_bib, bib.text().unwrap_or(&String::new())) {
                    Ok(_) => println!("Successfully wrote to {:?}", path_bib),
                    Err(e) => eprintln!("Failed to write to {:?}: {}", path_bib, e),
                };
            } else {
                eprintln!("Missing identifier for bib");
            }
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

    let notebook = gtk::Notebook::new();
    window.set_child(Some(&notebook));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    window.set_title(Some("Bib Shelf"));
    window.set_default_size(1200, 1000);

    vbox.append(&item_name_box());

    let mut shelf = Shelf::default();

    let list_box = gtk::ListBox::new();
    list_box.bind_model(Some(shelf.model()), |item| {
        let spine = item.downcast_ref::<Spine>().unwrap();
        ui::display_ui(spine).upcast::<gtk::Widget>()
    });

    let model = shelf.model();

    list_box.connect_row_activated(glib::clone!(@weak model => move |_list_box, row| {
        let index = row.index();
        if let Some(item) = model.item(index as u32) {
            if let Some(spine) = item.downcast_ref::<Spine>() {
                let pdf_path = spine.path();

                if !pdf_path.exists() {
                    eprintln!("Error: File does not exist at {:?}", pdf_path);
                    return;
                }
    
                println!("PDF path: {:?}", pdf_path);
    
                let result = if cfg!(target_os = "linux") {
                    if env::var("WSL_DISTRO_NAME").is_ok() {
                        Command::new("powershell.exe").args(&["/c", "start", pdf_path.to_str().expect("Invalid Unicode in file path")]).spawn()
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

    let bibs = get_bibs_first();
    shelf.add_bibs(bibs);

    let shelf = Rc::new(RefCell::new(shelf));

    let bib_label = gtk::Label::builder()
        .label("New bib(s)")
        .halign(gtk::Align::Start)
        .build();

    vbox.append(&bib_label);
    vbox.append(&input_box(shelf));

    let tab_label_article = gtk::Label::new(Some("article"));
    notebook.append_page(&vbox, Some(&tab_label_article));

    let provider = CssProvider::new();
    provider.load_from_data("* {
                                font-size: 14px;
                                font-family: 'Segoe UI', 'Arial', 'Noto Sans', sans-serif;
                            }");
    let style_context = window.style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.show();
}

fn input_box(shelf: Rc<RefCell<Shelf>>) -> gtk::Box {
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
        glib::clone!(@weak text_view, @strong shelf => move |_| {
            let buffer = text_view.buffer();
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let t = buffer.text(&start, &end, false).to_string();

            let bibs = get_bibs(t);
            shelf.borrow_mut().add_bibs(bibs);
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