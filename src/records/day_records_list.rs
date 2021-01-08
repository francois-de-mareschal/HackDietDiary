use crate::records::day_records::DayRecords;
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
    ) -> Result<(), &'static str> {
        todo!()
    }

    pub fn remove(
        &mut self,
        day_records: DayRecords,
        date: Option<Date<Utc>>,
    ) -> Result<(), &'static str> {
        todo!()
    }

    pub fn range(
        &self,
        from: Option<Date<Utc>>,
        to: Option<Date<Utc>>,
    ) -> Result<BTreeMap<Date<Utc>, DayRecords>, &'static str> {
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

        drl.add(dr.clone(), None)?;

        assert_eq!(*drl.records.get(&Utc::now().date()).unwrap(), dr);

        Ok(())
    }

    #[test]
    fn add_today_day_record_specifying_current_date() -> Result<(), Box<dyn Error>> {
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
    fn add_someday_day_record() -> Result<(), Box<dyn Error>> {
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
    #[should_panic(expected = "the record date is in the future")]
    fn add_future_day_record() {
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
    fn add_today_day_record_twice() -> Result<(), Box<dyn Error>> {
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

        drl.add(dr_first, None)?;
        drl.add(dr_second.clone(), None)?;

        assert_eq!(*drl.records.get(&Utc::now().date()).unwrap(), dr_second);

        Ok(())
    }

    #[test]
    fn add_today_day_record_twice_specifying_current_date() -> Result<(), Box<dyn Error>> {
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
    fn add_someday_day_record_twice() -> Result<(), Box<dyn Error>> {
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
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn add_today_day_record_negative_weight() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(-85.0),
            notes: None,
        };

        drl.add(dr, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn add_today_day_record_negative_weight_specifying_current_date() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(-85.0),
            notes: None,
        };

        drl.add(dr, Some(Utc::now().date())).unwrap();
    }

    #[test]
    #[should_panic(expected = "the provided weight is zero or negative")]
    fn add_someday_day_record_negative_weight() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: Some(-85.0),
            notes: None,
        };
        let date = Utc.ymd(1994, 05, 18);

        drl.add(dr, Some(date)).unwrap();
    }

    #[test]
    #[should_panic(expected = "provided day records are empty")]
    fn add_today_empty_day_record() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: None,
            notes: None,
        };

        drl.add(dr, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "provided day records are empty")]
    fn add_today_empty_day_record_specifying_current_date() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: None,
            notes: None,
        };

        drl.add(dr, Some(Utc::now().date())).unwrap();
    }

    #[test]
    #[should_panic(expected = "provided day records are empty")]
    fn add_someday_empty_day_record() {
        let mut drl = DayRecordsList {
            records: BTreeMap::new(),
        };
        let dr = DayRecords {
            weight: None,
            notes: None,
        };
        let date = Utc.ymd(1994, 05, 18);

        drl.add(dr, Some(date)).unwrap();
    }
}
