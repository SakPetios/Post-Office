use cursive::views::{Dialog, LinearLayout, TextView};

use crate::{traits, utils};

pub struct ResultViewer;

impl traits::State for ResultViewer {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        cur.add_layer(
            LinearLayout::horizontal().child(
                Dialog::around(TextView::new("Results Will Be Displayed Here"))
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
