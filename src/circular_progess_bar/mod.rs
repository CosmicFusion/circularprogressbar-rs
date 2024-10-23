mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CircularProgressBar(ObjectSubclass<imp::CircularProgressBar>)
        @extends gtk::Widget, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CircularProgressBar {
    pub fn new() -> Self {
        Object::builder().build()
    }
    /*pub fn new_with_widgets(
        partitions_scroll: &gtk::ScrolledWindow,
        parent_window: &adw::ApplicationWindow,
    ) -> Self {
        Object::builder()
            .property("partitionscroll", partitions_scroll)
            .property("transient-for", parent_window)
            .build()
    }*/
}
// ANCHOR_END: mod

impl Default for CircularProgressBar {
    fn default() -> Self {
        Self::new()
    }
}