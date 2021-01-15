use crate::recordings::day_records::DayRecords;
use crate::recordings::error::RecordingsError;
use chrono::prelude::*;
use std::collections::BTreeMap;
use std::error::Error;

pub struct DayRecordsList {
    records: BTreeMap<Date<Utc>, DayRecords>,
}

impl DayRecordsList {
    pub fn add(
        &mut self,
        day_records: DayRecords,
        date: Option<Date<Utc>>,
    ) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().date();
        let date = date.unwrap_or(now);

        if date > now {
            return Err(Box::new(RecordingsError::DateAddingInTheFuture(date)));
        }

        self.records.insert(date, day_records);

        Ok(())
    }

    pub fn remove(&mut self, date: Option<Date<Utc>>) -> Result<(), Box<dyn Error>> {
        let now = Utc::now().date();
        let date = date.unwrap_or(now);

        if date > now {
            return Err(Box::new(RecordingsError::DateRemovingInTheFuture(date)));
        }

        if self
            .records
            .keys()
            .find(|&key_date| key_date == &date)
            .is_none()
        {
            return Err(Box::new(RecordingsError::NoDayRecordsAtDate(date)));
        }

        self.records.remove(&date);

        Ok(())
    }

    pub fn range(
        &self,
        from: Option<Date<Utc>>,
        to: Option<Date<Utc>>,
    ) -> Result<Option<BTreeMap<Date<Utc>, DayRecords>>, Box<dyn Error>> {
        let now = Utc::now().date();
        let from = from.unwrap_or(now);
        let to = to.unwrap_or(now);

        if from > to {
            return Err(Box::new(RecordingsError::StartDateAfterEndDate {
                start_date: from,
                end_date: to,
            }));
        }

        let mut day_records_range = BTreeMap::<Date<Utc>, DayRecords>::new();
        for (key_date, value_records) in &self.records {
            if key_date >= &from && key_date <= &to {
                day_records_range.insert(key_date.clone(), value_records.clone());
            }
        }

        return if day_records_range.is_empty() {
            Ok(None)
        } else {
            Ok(Some(day_records_range))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn add_today_day_records() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc::now().date();

        drl.add(dr.clone(), None)?;

        assert_eq!(*drl.records.get(&date).unwrap(), dr);

        Ok(())
    }

    #[test]
    fn add_today_day_records_specifying_current_date() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc::now().date();

        drl.add(dr.clone(), Some(date.clone()))?;

        assert_eq!(*drl.records.get(&date).unwrap(), dr);
        Ok(())
    }

    #[test]
    fn add_someday_day_records() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc.ymd(1994, 05, 18);

        drl.add(dr.clone(), Some(date))?;

        assert_eq!(*drl.records.get(&date).unwrap(), dr);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "DateAddingInTheFuture(3994-05-18Z)")]
    fn add_future_day_records() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc.ymd(3994, 05, 18);

        drl.add(dr, Some(date)).unwrap();
    }

    #[test]
    fn add_today_day_records_twice() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr_first = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let dr_second = DayRecords {
            weight: Some(80.0),
            notes: None,
        };
        let date = Utc::now().date();

        drl.add(dr_first, None)?;
        drl.add(dr_second.clone(), None)?;

        assert_eq!(*drl.records.get(&date).unwrap(), dr_second);

        Ok(())
    }

    #[test]
    fn add_today_day_records_twice_specifying_current_date() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr_first = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let dr_second = DayRecords {
            weight: Some(80.0),
            notes: None,
        };
        let date = Utc::now().date();

        drl.add(dr_first, Some(date))?;
        drl.add(dr_second.clone(), Some(date))?;

        assert_eq!(*drl.records.get(&date).unwrap(), dr_second);

        Ok(())
    }

    #[test]
    fn add_someday_day_records_twice() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr_first = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let dr_second = DayRecords {
            weight: Some(80.0),
            notes: None,
        };
        let date = Utc.ymd(1994, 05, 18);

        drl.add(dr_first, Some(date))?;
        drl.add(dr_second.clone(), Some(date))?;

        assert_eq!(*drl.records.get(&date).unwrap(), dr_second);

        Ok(())
    }

    #[test]
    fn remove_today_existent_records() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc::now().date();

        drl.records.insert(date, dr);
        drl.remove(None)?;

        assert!(!drl.records.keys().any(|&key_date| key_date == date));

        Ok(())
    }

    #[test]
    #[should_panic(expected = "NoDayRecordsAtDate(2021-01-15Z)")]
    fn remove_today_non_existent_records() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        drl.remove(None).unwrap();
    }

    #[test]
    fn remove_today_existent_records_specifying_current_date() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc::now().date();

        drl.records.insert(date, dr);
        drl.remove(Some(date))?;

        assert!(!drl.records.keys().any(|&key_date| key_date == date));

        Ok(())
    }

    #[test]
    #[should_panic(expected = "NoDayRecordsAtDate(2021-01-15Z)")]
    fn remove_today_non_existent_records_specifying_current_date() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let date = Utc::now().date();

        drl.remove(Some(date)).unwrap();
    }

    #[test]
    fn remove_someday_existent_records() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc.ymd(1994, 05, 18);

        drl.records.insert(date, dr);
        drl.remove(Some(date))?;

        assert!(!drl.records.keys().any(|&key_date| key_date == date));

        Ok(())
    }

    #[test]
    #[should_panic(expected = "NoDayRecordsAtDate(1994-05-18Z)")]
    fn remove_someday_non_existent_records() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let date = Utc.ymd(1994, 05, 18);

        drl.remove(Some(date)).unwrap();
    }

    #[test]
    #[should_panic(expected = "DateRemovingInTheFuture(3994-05-18Z)")]
    fn remove_future_day_records() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc.ymd(3994, 05, 18);

        drl.records.insert(date, dr);
        drl.remove(Some(date)).unwrap();
    }

    #[test]
    fn range_from_lt_to_records_existent() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc::now().date();
        drl.records.insert(date, dr.clone());

        let btm = drl.range(None, Some(date))?.unwrap();

        assert_eq!(*btm.get(&date).unwrap(), dr);

        Ok(())
    }

    #[test]
    fn range_from_lt_to_records_non_existent() {
        let drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let date = Utc::now().date();

        let btm = drl.range(None, Some(date)).unwrap();

        assert_eq!(btm, None)
    }

    #[test]
    #[should_panic(
        expected = "StartDateAfterEndDate { start_date: 2021-01-15Z, end_date: 2021-01-14Z }"
    )]
    fn range_from_gt_to() {
        let drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let start_date = Utc::now().date();
        let end_date = start_date - Duration::days(1);

        drl.range(Some(start_date), Some(end_date)).unwrap();
    }

    #[test]
    fn range_from_eq_to() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let date = Utc.ymd(1994, 05, 18);
        drl.records.insert(date, dr.clone());

        let btm = drl.range(Some(date), Some(date))?.unwrap();

        assert_eq!(
            btm.values().find(|&value_records| value_records == &dr),
            Some(&dr)
        );

        Ok(())
    }

    #[test]
    fn range_from_lt_first_record_date() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let first_dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let second_dr = DayRecords {
            weight: Some(80.0),
            notes: None,
        };
        let search_date = Utc.ymd(1994, 05, 18);
        let first_date = Utc.ymd(2003, 05, 11);
        let last_date = Utc::now().date();
        drl.records.insert(first_date, first_dr.clone());
        drl.records.insert(last_date, second_dr.clone());

        let btm = drl.range(Some(search_date), Some(last_date))?.unwrap();

        assert_eq!(btm.get(&first_date).unwrap(), &first_dr);

        Ok(())
    }

    #[test]
    fn range_to_gt_last_record_date() -> Result<(), Box<dyn Error>> {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let first_dr = DayRecords {
            weight: Some(85.0),
            notes: None,
        };
        let second_dr = DayRecords {
            weight: Some(80.0),
            notes: None,
        };
        let search_date = Utc::now().date();
        let first_date = Utc.ymd(1994, 05, 18);
        let last_date = Utc.ymd(2003, 05, 11);
        drl.records.insert(first_date, first_dr.clone());
        drl.records.insert(last_date, second_dr.clone());

        let btm = drl.range(Some(first_date), Some(search_date))?.unwrap();

        assert_eq!(btm.get(&last_date).unwrap(), &second_dr);

        Ok(())
    }
}
