use structopt::StructOpt;
use wca_scorecards_lib::*;

#[derive(StructOpt, Debug)]
struct Args {
    /// Competition name
    competion: String,

    /// Command
    #[structopt(subcommand)]
    command: Command,

    /// Number of stages used
    #[structopt(long, short, default_value = "1")]
    stages: u32,

    /// Sort by name instead of groups.
    #[structopt(name = "sort-by-name", long)]
    sort_by_name: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Generate from DVE csvs
    Csv {
        /// Path to groups csv
        groups: String,

        /// Path to limit csvs. If unspecified no time limits will be written on the scorecards
        limit: Option<String>,

        ///Number of stations per stage. If unspecified it is infinite
        stations: Option<u32>
    },

    /// Generate from WCIF
    Wcif {
        /// Number of solving stations per stage
        stations: u32,
    },

    /// Generate sheet with blank scorecards
    Blank,
}

fn main() {
    let args = Args::from_args();

    match args.command {
        Command::Csv { groups, limit, stations } => {
            let stages = Stages::new(u32::MAX, stations.unwrap_or(u32::MAX));
            print_round_1_english(&groups, limit, &args.competion, stages, args.sort_by_name);
        },
        Command::Wcif { stations } => {
            let stages = Stages::new(args.stages, stations);
            print_subsequent_rounds(args.competion, stages, args.sort_by_name);
        },
        Command::Blank => {
            blank_scorecard_page(&args.competion);
        },
    }
}