use std::fs;
use std::path::Path;

use cursive::view::Nameable;
use cursive::views::{Dialog, LinearLayout, SelectView, TextArea, TextView};
use cursive::With;

use crate::traits;
use crate::utils;
pub struct HomeState {
    blueprints: Vec<String>,
}

impl traits::State for HomeState {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        let blueprints = &self.blueprints;
        cur.add_layer(
            LinearLayout::horizontal()
                .child(
                    Dialog::around(
                        SelectView::new()
                            .with(|sel| {
                                for blueprint in blueprints {
                                    if !blueprint.ends_with(".json") {
                                        continue;
                                    };
                                    sel.add_item_str(blueprint.replace(".json", ""));
                                }
                            })
                            .on_submit(|c, val: &str| {
                                let mut blueprint_view =
                                    c.find_name::<TextView>("blue_print_view_area").unwrap();
                                let file_content = fs::read_to_string(
                                    Path::new("blueprints").join(val.to_owned() + ".json"),
                                );
                                match file_content {
                                    Ok(content) => blueprint_view.set_content(content),
                                    Err(er) => {
                                        utils::show_error(c, er, None);
                                        return;
                                    }
                                }
                            }),
                    )
                    .title("Select Recipe")
                    .button("Close", |c| {
                        c.pop_layer();
                    })
                    .button("Quit", utils::close)
                    .button("Run", utils::unimpl), // TODO Add a run function
                )
                .child(
                    Dialog::around(TextView::new("").with_name("blue_print_view_area"))
                        .title("Recipe"),
                ),
        )
    }
}
impl HomeState {
    // TODO Add a config from which the blueprint path will be extracted
    pub fn new() -> HomeState {
        let files = utils::listdir(Path::new("blueprints"), true);

        HomeState { blueprints: files }
    }
}
