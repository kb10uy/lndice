use std::env::args;

use ariadne::{Color, Label, Report, ReportKind, Source};
use lndice::parser::{ErrorElement, parse};

fn main() {
    let Some(source) = args().nth(1) else {
        return;
    };

    match parse(&source) {
        Ok(expr) => {
            println!("Parsed:");
            println!("{expr:?}");
        }
        Err(errs) => {
            let report_source = Source::from(&source);
            for err in errs {
                let report = make_report(err);
                report.eprint(report_source.clone()).expect("failed to print errors");
            }
        }
    }
}

fn make_report(ee: ErrorElement) -> Report<'static> {
    Report::build(ReportKind::Error, ee.span.clone())
        .with_message(ee.message)
        .with_label(Label::new(ee.span).with_message(ee.reason).with_color(Color::Red))
        .finish()
}
