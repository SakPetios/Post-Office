use std::collections::HashMap;

use cursive::{
    views::{Dialog, LinearLayout, ListView, TextView},
    With,
};

use crate::{colls, utils};

pub struct ResultViewer {
    pub tests: HashMap<String, bool>,
}

impl colls::State for ResultViewer {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        let tests = &self.tests;
        cur.add_layer(
            LinearLayout::horizontal().child(
                Dialog::around(ListView::new().with(|ls| {
                    for (name, status) in tests {
                        ls.add_child(
                            &name,
                            if status == &true {
                                TextView::new("+ Passed")
                            } else {
                                TextView::new("- Failed")
                            },
                        )
                    }
                }))
                .button("Close", |c| {
                    c.pop_layer();
                })
                .button("Quit", utils::close).title("Results"),
            ),
        )
    }
}

impl ResultViewer {
    pub fn new() -> ResultViewer {
        ResultViewer {
            tests: HashMap::new(),
        }
    }

}
