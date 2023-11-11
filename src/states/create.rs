use cursive::{
    view::Resizable,
    views::{Button, Checkbox, Dialog, EditView, LinearLayout, ListView, Panel, TextView},
    With,
};
use std::path::Path;

use crate::{colls, utils};

pub struct CreateRecipe;

impl colls::State for CreateRecipe {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        let files = utils::listdir(&Path::new("executors"), true);
        cur.add_layer(
            Dialog::around(
                LinearLayout::vertical().child(
                    LinearLayout::horizontal()
                        .child(
                            Dialog::around(EditView::new())
                                .title("Set Name")
                                .min_width(20),
                        )
                        .child(
                            Dialog::around(ListView::new().with(|ls| {
                                for file in files {
                                    ls.add_child(&file, Checkbox::new())
                                }
                            }))
                            .title("Select Pre-Executors"),
                        ),
                ),
            )
            .button("Close", |c| {
                c.pop_layer();
            }).button("Quit", utils::close),
        );
    }
}
