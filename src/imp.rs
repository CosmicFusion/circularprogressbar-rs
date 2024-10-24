use std::cell::RefCell;
use gtk::glib as glib;
use gtk::cairo as cairo;
use gtk::pango as pango;
use gtk::gdk as gdk;
use gtk::{prelude::*, subclass::prelude::*, ffi::gtk_widget_queue_draw};
use glib::{Properties, clone, translate::ToGlibPtr};

// ANCHOR: custom_button
// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::CircularProgressBar)]
pub struct CircularProgressBar {
    #[property(get, set)]
    child: RefCell<Option<gtk::Widget>>,
    #[property(get, set)]
    fill_center: RefCell<bool>,
    #[property(get, set)]
    fill_radius: RefCell<bool>,
    #[property(get, set)]
    center_fill_color: RefCell<Option<gdk::RGBA>>,
    #[property(get, set)]
    radius_fill_color: RefCell<Option<gdk::RGBA>>,
    #[property(get, set)]
    progress_fill_color: RefCell<Option<gdk::RGBA>>,
    #[property(get, set)]
    progress_font: RefCell<Option<String>>,
    #[property(get, set)]
    fraction_font_size: RefCell<i32>,
    #[property(get, set)]
    center_text_font_size: RefCell<i32>,
    #[property(get, set)]
    center_text: RefCell<Option<String>>,
    #[property(get, set)]
    fraction: RefCell<f64>,
    #[property(get, set)]
    line_width: RefCell<f64>,
}
// ANCHOR_END: custom_button

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CircularProgressBar {
    const NAME: &'static str = "CircularProgressBar";
    type Type = super::CircularProgressBar;
    type ParentType = gtk::Widget;

    fn class_init(gtk_class: &mut Self::Class) {
        gtk_class.set_layout_manager_type::<gtk::BinLayout>();
        gtk_class.set_css_name("progess-bar");
        gtk_class.set_accessible_role(gtk::AccessibleRole::ProgressBar);
    }
}

fn calculate_radius(w: f64, h: f64) -> f64 {
    std::cmp::min(w.round() as i64 ,(h - 1.0).round() as i64) as f64
}

fn redraw_widget(widget: &gtk::Widget) {
    unsafe {
        gtk_widget_queue_draw(widget.to_glib_full());
    }
}

// ANCHOR: object_impl
// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for CircularProgressBar {
    fn constructed(&self) {
        self.parent_constructed();

        // Bind label to number
        // `SYNC_CREATE` ensures that the label will be immediately set
        let obj = self.obj();

        let obj_clone0 = obj.clone();

        let draw_func = move |_da: &gtk::DrawingArea, cr: &gtk::cairo::Context, width: i32, height: i32| {
            let layout = pangocairo::functions::create_layout(cr);
            //
            let fraction = obj_clone0.fraction();
            let mut line_width = obj_clone0.line_width();
            let fill_center = obj_clone0.fill_center();
            let fill_radius = obj_clone0.fill_radius();
            let center_fill_color = obj_clone0.center_fill_color().unwrap_or(gdk::RGBA::new(60.0, 255.0, 0.0, 1.0));
            let radius_fill_color = obj_clone0.radius_fill_color().unwrap_or(gdk::RGBA::new(0.0, 213.0, 255.0, 1.0));
            let progress_fill_color = obj_clone0.progress_fill_color().unwrap_or(gdk::RGBA::new(252.0, 244.0, 0.0, 1.0));
            let progress_font = obj_clone0.progress_font().unwrap_or("URW Gothic".to_owned());
            let center_text_font_size = obj_clone0.center_text_font_size();
            let fraction_font_size = obj_clone0.fraction_font_size();
            let center_text = obj_clone0.center_text().unwrap_or("PERCENT".to_owned());
            //
            let center_x = (width / 2) as f64;
            let center_y = (height / 2)  as f64;
            let radius: f64 = calculate_radius(center_x, center_y);
            
            cr.save().unwrap();
        
            let delta = if radius - line_width < 0.0 {
                line_width = radius;
                0.0
            } else {
                radius - (line_width / 2.0)
            };
        
            let line_cap = cairo::LineCap::Butt;

            cr.set_line_cap  (line_cap);
            cr.set_line_width (line_width);

            // Center Fill
            if fill_center == true {
                cr.arc(center_x, center_y, delta, 0.0, 2.0 * std::f64::consts::PI);
                cr.set_source_color(&center_fill_color);
                cr.fill().unwrap();
            }
        
            // Radius Fill
            if fill_radius == true {
                cr.arc(center_x, center_y, delta, 0.0, 2.0 * std::f64::consts::PI);
                cr.set_source_color(&radius_fill_color);
                cr.stroke().unwrap();
            }
        
            // Progress/fraction Fill
            if fraction > 0.0 {
                cr.set_source_color(&progress_fill_color);
                if line_width == 0.0 {
                    cr.move_to (center_x, center_y);
                    cr.arc(center_x,
                            center_y,
                            delta+1.0,
                            1.5  * std::f64::consts::PI,
                            (1.5 + fraction * 2.0 ) * std::f64::consts::PI
                        );
                    cr.fill().unwrap();
                } else {
                    cr.arc(center_x,
                            center_y,
                            delta,
                            1.5  * std::f64::consts::PI,
                            (1.5 + fraction * 2.0 ) * std::f64::consts::PI
                        );
                    cr.stroke().unwrap();
                }
            }
        
            // Textual information
            let context = obj_clone0.style_context();
            context.save();
            // FIXME: Gtk4 has changes in the styles that need to be reviewed
            // For now we get the text color from the defaut context.
            cr.set_source_color(&context.color());
        
            // fraction
            layout.set_text(&((fraction * 100.0).round()).to_string());
            let desc = pango::FontDescription::from_string(&(progress_font.clone() + " " + &fraction_font_size.to_string()));
            layout.set_font_description(Some(&desc));
            pangocairo::functions::update_layout(cr, &layout);
            let (out_w, _out_h) = layout.size(); 
            cr.move_to (center_x - ((out_w / pango::SCALE) / 2) as f64, center_y - 27.0);
            pangocairo::functions::show_layout (cr, &layout);
        
            // Units indicator (fraction)
            layout.set_text(&center_text);
            let desc = pango::FontDescription::from_string(&(progress_font + " " + &center_text_font_size.to_string()));
            layout.set_font_description(Some(&desc));
            pangocairo::functions::update_layout(cr, &layout);
            let (out_w, _out_h) = layout.size(); 
            cr.move_to (center_x - ((out_w / pango::SCALE) / 2) as f64, center_y + 13.0);
            pangocairo::functions::show_layout (cr, &layout);
            context.restore();
            cr.restore().unwrap();
        };
        
        let child = gtk::DrawingArea::new();
        child.set_draw_func(draw_func);
        let child_widget = child.clone().upcast::<gtk::Widget>();
        //
        obj.connect_fill_center_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_fill_radius_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_center_fill_color_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_radius_fill_color_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_progress_fill_color_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_progress_font_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_fraction_font_size_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_center_text_font_size_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_center_text_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_fraction_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        obj.connect_line_width_notify(clone!(
            #[strong] child_widget,
            move |_|
                {
                    redraw_widget(&child_widget);
                }
            )
        );
        //
        child.set_parent(&*obj);
        *self.child.borrow_mut() = Some(child_widget);
    }

    fn dispose(&self) {
        // Child widgets need to be manually unparented in `dispose()`.
        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }
    }
}
// Trait shared by all widgets
impl WidgetImpl for CircularProgressBar {}
