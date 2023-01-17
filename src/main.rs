use structopt::StructOpt;
use wca_scorecards_lib::*;

#[derive(StructOpt, Debug)]
struct Args {
    /// Competition name
    competion: String,

    /// Number of solving stations per stage
    capacity: u32,

    /// Command
    #[structopt(subcommand)]
    command: Command,

    /// Number of stages used
    #[structopt(long, short, global = true, default_value = "1")]
    stages: u32,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Generate from DVE csvs
    Csv {
        /// Path to groups csv
        groups: String,

        /// Path to limit csvs. If unspecified no time limits will be written on the scorecards
        limit: Option<String>,
    },

    /// Generate from WCIF
    Wcif,

    /// Generate sheet with blank scorecards
    Blank,
}

fn main() {
    let args = Args::from_args();

    let stages = Stages::new(args.stages, args.capacity);

    match args.command {
        Command::Csv { groups, limit } => {
            print_round_1_english(&groups, limit, &args.competion, stages);
        },
        Command::Wcif => {
            print_subsequent_rounds(args.competion, stages)
        },
        Command::Blank => {
            blank_scorecard_page(&args.competion)
        },
    }
}