use clap::Parser;
use repeat_audio;
use wavers::WaversError;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    min_duration_seconds: f64,
    #[arg()]
    input_file_name: String,
}

fn main() -> Result<(), WaversError> {
    let args = Args::parse();

    let min_duration_seconds = args.min_duration_seconds;
    let input_file_name = args.input_file_name.as_str();

    repeat_audio::repeat_audio(min_duration_seconds, input_file_name)
}
