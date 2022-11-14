use chrono::NaiveDate;

#[derive(Debug)]
pub struct FilterData {
    start_date: String,
    end_date: String,
}

impl FilterData {
    pub fn parse_data_to_date( sDate: String, eDate: String) -> (NaiveDate, NaiveDate) {
        (
            NaiveDate::parse_from_str(&sDate, "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str(&eDate, "%Y-%m-%d").unwrap(),
        )
    }
}