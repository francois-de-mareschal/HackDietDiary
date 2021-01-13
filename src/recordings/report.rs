use crate::recordings::day_records::DayRecords;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
enum ReportError {
    DayRecordsEmpty(&'static str),
    ZeroOrNegativeWeight(&'static str),
}

impl Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ReportError::DayRecordsEmpty(message) | ReportError::ZeroOrNegativeWeight(message) => {
                fmt::write(f, format_args!("{}", message))
            }
        }
    }
}

impl Error for ReportError {}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub weight: Option<f32>,
    pub notes: Option<String>,
}

impl Report {
    pub fn new() -> Report {
        todo!()
    }

    pub fn weight(&mut self, weight: f32) -> &mut Report {
        todo!();

        self
    }

    pub fn notes(&mut self, notes: String) -> &mut Report {
        todo!();

        self
    }

    pub fn record(&self) -> Result<DayRecords, Box<dyn Error>> {
        todo!();

        Ok(DayRecords {
            weight: None,
            notes: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_report_creates_empty_struct() {
        assert_eq!(
            Report::new(),
            Report {
                weight: None,
                notes: None,
            }
        );
    }

    #[test]
    fn weight_set_builder_weight() {
        let mut report = Report::new();
        report.weight(85.0);

        assert_eq!(
            report,
            Report {
                weight: Some(85.0),
                notes: None
            }
        );
    }

    #[test]
    fn notes_set_builder_notes() {
        let mut report = Report::new();
        report.notes(String::from("Still waiting to be vaccinated"));

        assert_eq!(
            report,
            Report {
                weight: None,
                notes: Some(String::from("Still waiting to be vaccinated")),
            }
        );
    }

    #[test]
    fn weight_notes_builder() {
        let mut report = Report::new();
        report.weight(85.0);
        report.notes(String::from("Are ya winning son?"));

        assert_eq!(
            report,
            Report {
                weight: Some(85.0),
                notes: Some(String::from("Are ya winning son?")),
            }
        );
    }

    #[test]
    fn weight_notes_builder_empty() {
        let mut report = Report::new();

        assert_eq!(
            report,
            Report {
                weight: None,
                notes: None,
            }
        );
    }

    #[test]
    #[should_panic(expected = "provided day records are empty")]
    fn report_record_empty() {
        let mut report = Report::new();

        report.record().unwrap();
    }

    #[test]
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn weight_negative_no_notes() {
        let result = Report::new().weight(-85.0).record().unwrap();
    }

    #[test]
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn weight_zero_no_notes() {
        let result = Report::new().weight(0.0).record().unwrap();
    }

    #[test]
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn weight_negative_with_some_notes() {
        let result = Report::new()
            .weight(-85.0)
            .notes(String::from("Vanum est vobis ante lucem surgere."))
            .record()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn weight_zero_with_some_notes() {
        let result = Report::new()
            .weight(-85.0)
            .notes(String::from("Bonum vinum laetificat cor hominis."))
            .record()
            .unwrap();
    }
}
