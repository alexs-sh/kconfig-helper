use crate::parameters::{Change, ChangeType};

pub(crate) fn make_link(config_name: &str) -> Option<String> {
    const PREFIX_LEN: usize = "CONFIG_".len();
    if config_name.len() > PREFIX_LEN {
        Some(format!(
            "https://cateee.net/lkddb/web-lkddb/{}.html",
            &config_name[PREFIX_LEN..]
        ))
    } else {
        None
    }
}

pub(crate) trait ChangeReporter {
    fn on_change(&mut self, change: &Change);
}

struct SimpleTextReporter;

impl ChangeReporter for SimpleTextReporter {
    fn on_change(&mut self, change: &Change) {
        if matches!(change.change_type, ChangeType::Unmodified) {
            return;
        }
        println!(
            "{:?}({},{},{})",
            change.change_type, change.parameter_name, change.old_value, change.new_value
        );
    }
}

struct SimpleTextReporterWithLinks {}

impl ChangeReporter for SimpleTextReporterWithLinks {
    fn on_change(&mut self, change: &Change) {
        if matches!(change.change_type, ChangeType::Unmodified) {
            return;
        }
        println!(
            "{:?}({},{},{},{}",
            change.change_type,
            change.parameter_name,
            change.old_value,
            change.new_value,
            make_link(change.parameter_name).unwrap_or_else(|| "NA".to_string())
        );
    }
}

struct MDReporter {
    print_header: bool,
    counter: u64,
}

impl MDReporter {
    fn new() -> Self {
        MDReporter {
            print_header: true,
            counter: 0,
        }
    }
}

impl ChangeReporter for MDReporter {
    fn on_change(&mut self, change: &Change) {
        if matches!(change.change_type, ChangeType::Unmodified) {
            return;
        }
        if self.print_header {
            println!("|# | Parameter | Status | Old Value | New Value |");
            println!("|--|-----------|--------|-----------|-----------|");
            self.print_header = false;
        }
        let link = make_link(change.parameter_name).unwrap_or_else(|| "NA".to_string());
        self.counter += 1;
        println!(
            "|{}|[{}]({})|{:?}|{}|{}|",
            self.counter,
            change.parameter_name,
            link,
            change.change_type,
            change.old_value,
            change.new_value
        );
    }
}

struct CSVReporter {
    print_header: bool,
    counter: u64,
}

impl CSVReporter {
    fn new() -> Self {
        CSVReporter {
            print_header: true,
            counter: 0,
        }
    }
}

impl ChangeReporter for CSVReporter {
    fn on_change(&mut self, change: &Change) {
        if matches!(change.change_type, ChangeType::Unmodified) {
            return;
        }
        if self.print_header {
            println!("#,Parameter,Status,Old Value,New Value,Description");
            self.print_header = false;
        }

        let link = make_link(change.parameter_name).unwrap_or_else(|| "NA".to_string());
        self.counter += 1;
        println!(
            "{},{},{:?},{},{},{}",
            self.counter,
            change.parameter_name,
            change.change_type,
            change.old_value,
            change.new_value,
            link
        );
    }
}

pub(crate) fn build(reporter_name: &str) -> Box<dyn ChangeReporter> {
    match reporter_name {
        "text-with-links" => Box::new(SimpleTextReporterWithLinks {}),
        "md" => Box::new(MDReporter::new()),
        "csv" => Box::new(CSVReporter::new()),
        _ => Box::new(SimpleTextReporter {}),
    }
}
