use crate::records::day_records;
use chrono::prelude::*;
use std::collections::BTreeMap;

struct DayRecordsList {
    records: BTreeMap<DateTime<Utc>, day_records::DayRecords>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
