use clap::{Parser, Subcommand, Args};
use windows::Devices::Enumeration::{DeviceClass, DeviceInformation};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, global = true)]
    verbose: bool,

    #[clap(short, long)]
    list: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
struct GetArgs {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(long)]
    watch: bool,
}

#[derive(Debug, Args)]
struct SetArgs {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(long)]
    value: u16,
}

#[derive(Debug, Args)]
struct AddArgs {
    #[clap(short, long)]
    name: Option<String>,

    #[clap(long)]
    value: i32,
}

#[derive(Debug, Args)]
struct MuteArgs {
    #[clap(short, long)]
    name: Option<String>,
}

#[derive(Debug, Args)]
struct StateArgs {
    #[clap(short, long)]
    name: Option<String>,
}


fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    futures::executor::block_on(get_list());
}


async fn get_list() {

    // デバイス一覧を非同期的に取得
    let devices = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::AudioRender).unwrap().await.unwrap();

    for device in devices {

        let mut name = device.Name().unwrap().to_os_string().into_string().unwrap();
        // sjis to utf8
        let (cow, _encoding_used, _had_errors) = encoding_rs::SHIFT_JIS.decode(&name.as_bytes());
        name = cow.into_owned();

        println!("{}: {}", device.Id().unwrap(), name);
    }

}
