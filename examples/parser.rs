use std::env::args;

use ariadne::{Color, Label, Report, ReportKind, Source};
use lndice::parser::parse_command;

fn main() {
    let Some(source) = args().nth(1) else {
        return;
    };

    match parse_command(&source) {
        Ok(command) => {
            println!("Parsed:");
            println!("{command:?}");
        }
        Err(errs) => {
            for err in errs {
                let report = Report::build(ReportKind::Error, err.span().into_range())
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .with_labels(err.contexts().map(|(label, span)| {
                        Label::new(span.into_range())
                            .with_message(format!("while parsing {label}"))
                            .with_color(Color::Yellow)
                    }))
                    .finish();
                report.eprint(Source::from(&source)).expect("failed to print errors");
            }
        }
    }
}
