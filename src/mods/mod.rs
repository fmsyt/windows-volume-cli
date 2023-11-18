use clap::{Parser, Subcommand, Args};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long, global = true)]
    pub verbose: bool,

    #[clap(short, long)]
    pub list: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    State(StateArgs),
    Set(SetArgs),
    Get(GetArgs),
    Add(AddArgs),
    SetLeft(SetArgs),
    GetLeft(GetArgs),
    AddLeft(AddArgs),
    SetRight(SetArgs),
    GetRight(GetArgs),
    AddRight(AddArgs),
    Mute(MuteArgs),
}

#[derive(Debug, Args)]
pub struct GetArgs {
    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(long)]
    watch: bool,
}

#[derive(Debug, Args)]
pub struct SetArgs {
    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(long)]
    pub value: u16,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    #[clap(short, long)]
    pub name: Option<String>,

    #[clap(long)]
    pub value: i32,
}

#[derive(Debug, Args)]
pub struct MuteArgs {
    #[clap(short, long)]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct StateArgs {
    #[clap(short, long)]
    pub name: Option<String>,
}
