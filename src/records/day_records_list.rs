use crate::records::day_records;
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;

pub struct DayRecordsList {
    records: BTreeMap<DateTime<Utc>, day_records::DayRecords>,
}

impl DayRecordsList {
    pub fn new() -> DayRecordsList {
        todo!()
    }

    pub fn add(
        &mut self,
        day_records: day_records::DayRecords,
        date: Option<DateTime<Utc>>,
    ) -> Result<day_records::DayRecords, Box<dyn Error>> {
        todo!()
    }

    pub fn remove(
        &mut self,
        day_records: day_records::DayRecords,
        date: Option<DateTime<Utc>>,
    ) -> Result<day_records::DayRecords, Box<dyn Error>> {
        todo!()
    }

    pub fn range(
        &self,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> Result<BTreeMap<DateTime<Utc>, day_records::DayRecords>, Box<dyn Error>> {
        todo!()
    }
}

impl Iterator for DayRecordsList {
    type Item = day_records::DayRecords;

    fn next(&mut self) -> Option<day_records::DayRecords> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
