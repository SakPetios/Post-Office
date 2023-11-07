use clap::Parser;
use cursive::{
    event::Key,
    menu,
    views::{Dialog, LinearLayout, SelectView, TextView},
    Cursive, CursiveExt, With,
};
use log::info;

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
        siv.set_global_callback('q', |s| s.quit());
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
                    .item(menu::Item::leaf("Import Test", |_c| {})) // TODO
                    .item(menu::Item::leaf("Import Tests", |_c| {})), // TODO
            )
            .add_delimiter()
            .add_subtree(
                "Help",
                menu::Tree::new().item(menu::Item::leaf("What's Lua?", |_c| {})), // TODO
            )
            .add_leaf("Exit", |c| c.quit());
        // + # Widget Code
        
        self.cur.add_layer(
            LinearLayout::horizontal()
                .child(Dialog::around(SelectView::<String>::new().with(|_c| {
                    // for test in tests {
                    //     c.add_item(test.clone(), test)
                    // }
                })))
                .child(Dialog::around(TextView::new("Hello"))),
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
