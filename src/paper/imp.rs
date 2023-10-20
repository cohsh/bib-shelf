use gtk::glib;
use gtk::glib::{prelude::*, Properties};
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Debug, Properties)]
#[properties(wrapper_type = super::Paper)]
pub struct Paper {
    #[property(get, set)]
    title: RefCell<String>,
    #[property(get, set)]
    author: RefCell<String>,
    #[property(get, set)]
    year: Cell<i64>,
}

impl Default for Paper {
    fn default() -> Self {
        Self {
            title: RefCell::new(String::new()),
            author: RefCell::new(String::new()),
            year: Cell::new(0),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Paper {
    const NAME: &'static str = "Paper";
    type Type = super::Paper;
    type ParentType = glib::Object;
}

impl ObjectImpl for Paper {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }
}