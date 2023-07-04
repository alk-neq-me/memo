use clap::Parser;


#[derive(Parser, Debug)]
#[command(
	version = "1.0",
    author = "Aung koko Lwin",
    about = "🌱 A terminal-based memo note app 📝"
)]
pub struct Args {
	#[arg(long)]
	pub book:  Option<String>,  // get tasks in a book

    #[arg(long)]
    pub list_book: bool,

    #[arg(long)]
    pub list_task: bool,

    #[arg(long)]
    pub add_book: Option<String>,

    #[arg(long)]
    #[arg(long, help = "Type book id")]
    pub remove_book: Option<u32>,

    #[arg(long)]
    pub add_task: Option<String>,

    #[arg(long, help = "Type task id")]
    pub remove_task: Option<u32>,

    #[arg(long, help="Type task id")]
    pub complete_task: Option<u32>,

    #[arg(long)]
    pub count: bool,

    #[arg(long)]
    pub completed: bool,

    #[arg(long)]
    pub clean_all: bool,
}

pub fn parse_args() -> Args {
	let args: Args = Args::parse();
    args
}
