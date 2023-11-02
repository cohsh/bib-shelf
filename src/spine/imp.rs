use gtk::{
    glib::{self, prelude::*, Properties},
    subclass::prelude::*,
};
use std::{
    path::PathBuf,
    cell::{Cell, RefCell},
};


#[derive(Debug, Properties)]
#[properties(wrapper_type = super::Spine)]
pub struct Spine {
    #[property(get, set)]
    year: Cell<u64>,
    #[property(get, set)]
    author: RefCell<String>,
    #[property(get, set)]
    title: RefCell<String>,
    #[property(get, set)]
    path: RefCell<PathBuf>,
}

impl Default for Spine {
    fn default() -> Self {
        Self {
            year: Cell::new(0),
            author: RefCell::new(String::new()),
            title: RefCell::new(String::new()),
            path: RefCell::new(PathBuf::new()),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Spine {
    const NAME: &'static str = "Spine";
    type Type = super::Spine;
    type ParentType = glib::Object;
}

impl ObjectImpl for Spine {
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