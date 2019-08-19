use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "edit-roblox-place",
)]
struct Opt {
    place_id: u64,
}

fn main() {
    let args = Opt::from_args();

    opener::open(format!("roblox-studio:1+task:EditPlace+placeId:{}", args.place_id))
        .expect("Couldn't open Roblox Studio");
}
