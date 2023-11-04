use std::{
    process::Command,
    cell::RefCell,
    rc::Rc,
    env,
    path::PathBuf,
    collections::HashMap,
    fs,
};

use gtk::{
    prelude::*,
    gio,
    glib,
    CssProvider,
    ComboBoxText,
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
    pub fn add_bib(&mut self, bib: &Bib) {
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
    window.set_title(Some("Bib Shelf"));
    window.set_default_size(1200, 600);

    let active_path = Rc::new(RefCell::new(PathBuf::new()));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let notebook = gtk::Notebook::new();
    
    let mut shelves: HashMap<&str, Shelf> = HashMap::new();

    // Ref. http://exlight.net/tutorial/bibtex-category.html
    let categories = [
        "article", "inproceedings", "phdthesis", "masterthesis", "book", "incollection",
        "inbook", "booklet", "manual", "proceedings", "techreport", "unpublished", "misc",
        ];

    for category in categories.iter() {
        shelves.insert(category, Shelf::default());

        let small_vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
        small_vbox.append(&item_name_box());

        let list_box = gtk::ListBox::new();

        if let Some(shelf) = shelves.get_mut(category) {
            list_box.bind_model(Some(shelf.model()), |item| {
                let spine = item.downcast_ref::<Spine>().unwrap();
                ui::display_ui(spine).upcast::<gtk::Widget>()
            });

            let model = shelf.model();

            list_box.connect_row_activated(glib::clone!(@weak model, @strong active_path => move |_list_box, row| {
                let index = row.index();
                if let Some(item) = model.item(index as u32) {
                    if let Some(spine) = item.downcast_ref::<Spine>() {
                        let pdf_path = spine.path();

                        *active_path.borrow_mut() = pdf_path.clone();
        
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
                .min_content_height(400)
                .min_content_width(1200)
                .child(&list_box)
                .build();

            small_vbox.append(&scrolled_window);

            let bibs = get_bibs_first(category);

            for bib in bibs.iter() {
                shelf.add_bib(bib);
            };

            let tab_label_article = gtk::Label::new(Some(category));
            notebook.append_page(&small_vbox, Some(&tab_label_article));    
        }
    }

    let combo_label = gtk::Label::builder()
    .label("Add PDF to Active Bib")
    .halign(gtk::Align::Start)
    .build();

    let combo_box = ComboBoxText::new();

    if let Ok(entries) = fs::read_dir("./pdf_pool/") {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    combo_box.append(None, file_name);
                }
            }
        }
    }

    combo_box.set_active(Some(0));

    combo_box.connect_changed(|c| {
        if let Some(text) = c.active_text() {
            println!("Selected: {}", text);
        }
    });

    let button = gtk::Button::with_label("Add");

    button.connect_clicked(glib::clone!(@weak combo_box, @weak active_path => move |_| {
        if let Some(text) = combo_box.active_text() {
            let mut path = PathBuf::from("./pdf_pool");
            path.push(&text);
    
            let active_path_borrowed = active_path.borrow();
            if !active_path_borrowed.exists() {
                if let Err(e) = fs::rename(&path, &*active_path_borrowed) {
                    eprintln!("Failed to rename file: {}", e);
                }
            }
        }
    }));
    
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    hbox.append(&combo_box);
    hbox.append(&button);

    let bib_label = gtk::Label::builder()
    .label("Add New Bib(s)")
    .halign(gtk::Align::Start)
    .build();

    let shelves = Rc::new(RefCell::new(shelves));

    vbox.append(&notebook);

    vbox.append(&combo_label);
    vbox.append(&hbox);

    vbox.append(&bib_label);
    vbox.append(&input_box(shelves));    

    window.set_child(Some(&vbox));

    let provider = CssProvider::new();
    provider.load_from_data("* {
                                font-size: 14px;
                                font-family: 'Segoe UI', 'Arial', 'Noto Sans', sans-serif;
                            }");
    let style_context = window.style_context();
    style_context.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

    window.show();
}

fn input_box(shelves: Rc<RefCell<HashMap<&'static str, Shelf>>>) -> gtk::Box {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let text_view = gtk::TextView::builder()
        .editable(true)
        .width_request(1100)
        .height_request(200)
        .build();
    
    let new_button = gtk::Button::builder()
        .label("Add")
        .width_request(100)
        .height_request(200)
        .build();

    new_button.connect_clicked(
        glib::clone!(@weak text_view, @strong shelves => move |_| {
            let buffer = text_view.buffer();
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let t = buffer.text(&start, &end, false).to_string();

            let bibs = get_bibs(t);

            let mut shelves_borrowed = shelves.borrow_mut();

            for bib in bibs.iter() {
                let category = match bib.category() {
                    Some(c) => c.as_str(),
                    None => continue,
                };

                if let Some(shelf) = shelves_borrowed.get_mut(category) {
                    shelf.add_bib(bib);
                } else {
                    eprintln!("Category {:?} not found.", category);
                };
            }
        }),
    );

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(1100)
        .min_content_height(200)
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

    label_title.set_hexpand(true);
    label_author.set_hexpand(true);
    hbox.append(&label_year);
    hbox.append(&label_author);
    hbox.append(&label_title);
    hbox
}

fn get_active_page_listbox_item(notebook: &gtk::Notebook) {
    // アクティブなNotebookのページを取得
    if let Some(page_number) = notebook.current_page() {
        println!("Active notebook page: {}", page_number);
    } else {
        println!("No active page found.");
    }
}