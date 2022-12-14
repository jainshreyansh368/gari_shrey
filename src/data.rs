use chrono::NaiveDate;

#[derive(Debug)]
pub struct FilterData {
    start_date: String,
    end_date: String,
}

impl FilterData {
    pub fn parse_data_to_date( s_date: String, e_date: String) -> (NaiveDate, NaiveDate) {
        (
            NaiveDate::parse_from_str(&s_date, "%Y-%m-%d").unwrap(),
            NaiveDate::parse_from_str(&e_date, "%Y-%m-%d").unwrap(),
        )
    }
}