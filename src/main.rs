mod backend;
mod colls;
mod states;
mod tests;
mod utils;

use clap::Parser;
use colls::State;
use cursive::{
    event::Key,
    menu,
    views::{Dialog, LinearLayout, SelectView},
    Cursive, CursiveExt,
};
use log::info;
use states::*;

#[derive(Parser)]
#[command(
    author = "SakPetios",
    version,
    name = "Post-Office",
    about = "Rust-Powered API Testing at Your Command",
    long_about = "Post-Office: Rust-Powered API Testing at Your Command!"
)]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Run Only One Test")]
    test: Option<String>,
    #[arg(
        short,
        long,
        value_name = "FOLDER",
        help = "This Will Run All The Lua Scripts With in a Folder"
    )]
    folder: Option<String>,
}
// const LOGO2: &str = "┏┓      ┏┓┏┏•   \n┃┃┏┓┏╋  ┃┃╋╋┓┏┏┓\n┣┛┗┛┛┗  ┗┛┛┛┗┗┗ ";
struct UserInterface {
    cur: Cursive,
}

impl UserInterface {
    pub fn new() -> Self {
        let mut siv = Cursive::default();
        siv.set_global_callback('q', utils::close);
        siv.set_global_callback(Key::Backspace, |c| {
            c.pop_layer();
        });
        siv.add_global_callback(Key::Esc, |s| s.select_menubar());

        UserInterface { cur: siv }
    }
    pub fn init(&mut self) {
        // + Greed User
        // TODO REMOVE THE COMMENT AFTER DEVELOPMENT

        // self.cur.add_layer(
        //     Dialog::around(TextView::new("Hello Welcome To Post-Office"))
        //         .button("close", |s| {
        //             s.pop_layer();
        //         })
        //         .title("Welcome")
        //         .title("Welcome")
        //         .title_position(cursive::align::HAlign::Left),
        // );

        // + Menu bar
        self.cur
            .menubar()
            .add_subtree(
                "File",
                menu::Tree::new()
                    .item(menu::Item::leaf("Import Test", utils::unimpl)) // TODO
                    .item(menu::Item::leaf("Import Tests", utils::unimpl)), // TODO
            )
            .add_delimiter()
            .add_leaf("Home", |c| {
                let mut home = HomeState::new();
                home.render(c);
            })
            .add_leaf("Create Tests", |c| {
                let mut testsv = CreateRecipe;
                testsv.render(c)
            })
            .add_leaf("Console", |c| {
                let mut testsv = Console::new();
                testsv.render(c)
            })
            .add_delimiter()
            .add_subtree(
                "Help",
                menu::Tree::new().item(menu::Item::leaf("What's Lua?", utils::unimpl)), // TODO
            )
            .add_leaf("Exit", utils::close);

        //// # Widget Code

        // + Home
        self.cur.add_layer(
            LinearLayout::horizontal().child(
                Dialog::around(
                    SelectView::<states::States>::new()
                        .item("Home", states::States::Home)
                        .item("Create Test", states::States::RecipeCreator)
                        .item("Lua Console", states::States::LuaConsole)
                        .on_submit(|c, val| match val {
                            states::States::Home => {
                                let mut home = HomeState::new();
                                home.render(c);
                            }
                            states::States::RecipeCreator => {
                                let mut testgod = CreateRecipe;
                                testgod.render(c);
                            }
                            states::States::LuaConsole => {
                                let mut con = Console::new();
                                con.render(c);
                            }
                        }),
                )
                .title("Welcome!")
                .button("Quit", utils::close)
                .button("Close", |c| {
                    c.pop_layer();
                }),
            ),
        )
    }

    pub fn run(&mut self) {
        self.cur.run();
    }
}

fn main() {
    color_backtrace::install();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    // let args = Args::parse();

    info!("STARTING");
    let mut ui = UserInterface::new();
    ui.init();
    ui.run();
}
