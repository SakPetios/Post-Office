use std::fmt::format;
use std::path::Path;
use std::{fs, io};

use cursive::view::Nameable;
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};
use cursive::With;

use crate::backend::{self, LuaBackend};
use crate::colls;
use crate::utils;

use super::ResultViewer;
// TODO Add text highlighting on the recipes ~ cursive-syntec
pub struct HomeState {
    blueprints: Vec<String>,
}

impl colls::State for HomeState {
    fn render(&mut self, cur: &mut cursive::Cursive) {
        let blueprints = &self.blueprints;
        cur.add_layer(
            LinearLayout::horizontal()
                .child(
                    Dialog::around(
                        SelectView::new()
                            .with(|sel| {
                                for blueprint in blueprints {
                                    if !blueprint.ends_with(".lua") {
                                        continue;
                                    };
                                    sel.add_item_str(blueprint.replace(".lua", ""));
                                }
                            })
                            .on_submit(move |c, val: &str| {
                                let mut blueprint_view =
                                    c.find_name::<TextView>("blue_print_view_area").unwrap();
                                let file_content = fs::read_to_string(
                                    Path::new("blueprints").join(val.to_owned() + ".lua"),
                                );
                                match file_content {
                                    Ok(content) => {
                                        blueprint_view.set_content(content);
                                    }
                                    Err(er) => {
                                        utils::show_error(c, er, None);
                                        return;
                                    }
                                }
                            })
                            .with_name("select_test"),
                    )
                    .title("Select Recipe")
                    .button("Close", |c| {
                        c.pop_layer();
                    })
                    .button("Quit", utils::close)
                    .button("Run", |c| {
                        let select = c.find_name::<SelectView>("select_test");
                        let select = match select {
                            Some(selection) => selection,
                            None => {
                                utils::show_error(
                                    c,
                                    io::Error::new(io::ErrorKind::NotFound, "SelectView Not Found"),
                                    Some(
                                        "The SelectView containing the selected test was not found",
                                    ),
                                );
                                return;
                            }
                        };
                        let test = select.selection().unwrap();
                        let code = fs::read_to_string(format!("blueprints/{}.lua", test));
                        let code = match code {
                            Ok(cd) => cd,
                            Err(er) => {
                                utils::show_error(
                                    c,
                                    er,
                                    Some(&format!("Couldn't read file {}", test)),
                                );
                                return;
                            }
                        };
                        let mut lua = LuaBackend::new();
                        lua.init();
                        let mut resutls = ResultViewer::new();
                        let res = lua.blueprint(code);
                        match res {
                            Ok(_) => (),
                            Err(er) => utils::show_error(
                                c,
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("LuaBacked::Blueprint returned Error\n{}", er),
                                ),
                                Some("Error Running Blue Print"),
                            ),
                        }
                        let test_results = lua.fetch();
                        for data in test_results {
                            match data {
                                backend::Data::TestResult { name, result } => {
                                    resutls.tests.insert(name, result);
                                }
                            }
                        }
                        resutls.render(c);
                    }),
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
