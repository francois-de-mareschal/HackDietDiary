use crate::records::day_records::DayRecords;
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;

pub struct DayRecordsList {
    records: BTreeMap<DateTime<Utc>, DayRecords>,
}

impl DayRecordsList {
    pub fn new() -> DayRecordsList {
        todo!()
    }

    pub fn add(
        &mut self,
        day_records: DayRecords,
        date: Option<DateTime<Utc>>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn remove(
        &mut self,
        day_records: DayRecords,
        date: Option<DateTime<Utc>>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn range(
        &self,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<BTreeMap<DateTime<Utc>, DayRecords>, Box<dyn Error>> {
        todo!()
    }
}

impl Iterator for DayRecordsList {
    type Item = DayRecords;

    fn next(&mut self) -> Option<DayRecords> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_today_day_record() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };

        drl.add(dr, None)
    }
}
