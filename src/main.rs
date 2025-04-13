use std::path::Path;

mod parameters;
mod reporter;

use crate::parameters::{Change, ChangeType, Parameters};
use crate::reporter::ChangeReporter;
use clap::Parser;

fn analyze(parameters: &Parameters, mut reporter: Box<dyn ChangeReporter>) {
    // I prefer to see the sorted reports
    let mut parameters_names = parameters.keys().fold(Vec::new(), |mut acc, name| {
        acc.push(name);
        acc
    });
    parameters_names.sort();

    // Let's collect some simple statistic
    let mut added = 0;
    let mut modified = 0;
    let mut removed = 0;
    let mut unmodified = 0;

    for parameter_name in parameters_names {
        let values @ [old_value, new_value] = parameters.get(parameter_name).unwrap();
        let change_type = parameters::analyze_changes(values);
        let change = Change {
            change_type,
            parameter_name,
            old_value,
            new_value,
        };
        reporter.as_mut().on_change(&change);
        modified += matches!(change_type, ChangeType::Modified) as u64;
        unmodified += matches!(change_type, ChangeType::Unmodified) as u64;
        added += matches!(change_type, ChangeType::Added) as u64;
        removed += matches!(change_type, ChangeType::Removed) as u64;
    }

    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("SUMMARY");
    eprintln!("--------------------------------------------------------------------------------");
    eprintln!("Parameters:");
    eprintln!("  - Modified:{}", modified);
    eprintln!("  - Added:{}", added);
    eprintln!("  - Removed:{}", removed,);
    eprintln!("  - Unmodified:{}", unmodified,);
    eprintln!("  - Analyzed:{}", parameters.len());
    eprintln!("--------------------------------------------------------------------------------");
}

/// A simple app for comparing Linux kernel configs.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the first config
    first_config: String,

    /// Path to the second config
    second_config: String,

    /// Output format. Available formats are [text, text-with-links, md, csv]
    #[arg(short, long)]
    format: Option<String>,
}

fn main() {
    let args = Args::parse();
    let configs = [
        Path::new(&args.first_config),
        Path::new(&args.second_config),
    ];
    let parameters = parameters::read(&configs).expect("Please provide a valid kernel config(s)");
    let reporter = reporter::build(&args.format.unwrap_or("".to_string()));
    analyze(&parameters, reporter);
}
