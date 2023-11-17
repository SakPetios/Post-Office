use cursive::{
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, TextView},
};

use crate::{colls, utils};

pub struct Console {}

impl colls::State for Console {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        cur.add_layer(
            Dialog::around(
                TextView::new("")
                    .with_name("stdout_console")
                    .scrollable()
                    .scroll_x(true)
                    .fixed_width(80)
                    .fixed_height(20),
            )
            .button("Close", |c| {
                c.pop_layer();
            })
            .button("Quit", utils::close)
            .button("Reload", |cr| {
                let mut txt = match cr.find_name::<TextView>("stdout_console") {
                    Some(txt) => txt,
                    None => {
                        log::error!("Error getting Stdout_Console");
                        return;
                    }
                };
                txt.set_content("");
                cr.with_user_data(|buff: &mut Vec<String>| {
                    for item in buff {
                        txt.append(item.to_owned() + "\n");
                    }
                });
            })
            .title("Console"),
        )
    }
}

impl Console {
    pub fn new() -> Console {
        Console {}
    }
}
