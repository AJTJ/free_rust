use crate::{apnea_forms::dto::report_dto::Report, diesel::ExpressionMethods};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_report(conn: &mut PgConnection, report_id: Uuid) -> diesel::QueryResult<Report> {
    use crate::schema::reports::dsl::*;

    reports.filter(id.eq(&report_id)).get_result::<Report>(conn)
}
