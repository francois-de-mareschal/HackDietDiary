use crate::recordings::day_records::DayRecords;
use std::error::Error;

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
}
