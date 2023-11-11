use cursive::views::{Dialog, LinearLayout, ListView, TextView};

use crate::{colls, utils};


pub struct ResultViewer;

impl colls::State for ResultViewer {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        cur.add_layer(
            LinearLayout::horizontal().child(
                Dialog::around(
                    ListView::new()
                        .child("Test 1", TextView::new("+ Passed"))
                        .child("Test 2", TextView::new("- Failed"))
                        .child("Test 3", TextView::new("- Failed"))
                        .child("Test 4", TextView::new("+ Passed"))
                        .child("Test 5", TextView::new("- Failed"))
                        .child("Test 6", TextView::new("+ Passed"))
                )
                .button("Close", |c| {
                    c.pop_layer();
                })
                .button("Quit", utils::close),
            ),
        )
    }
}

impl ResultViewer {
    pub fn new() -> ResultViewer {
        ResultViewer
    }
}
