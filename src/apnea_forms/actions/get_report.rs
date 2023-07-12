use crate::apnea_forms::dto::report_dto::{Report, ReportRetrievalData};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};

pub fn get_report(
    conn: &mut PgConnection,
    report_retrieval: ReportRetrievalData,
) -> diesel::QueryResult<Option<Report>> {
    use crate::schema::reports::dsl::*;

    match report_retrieval {
        ReportRetrievalData::ReportId(report_id) => reports
            .filter(id.eq(&report_id))
            .get_result::<Report>(conn)
            .optional(),
        ReportRetrievalData::SessionId(inc_session_id) => reports
            .filter(session_id.eq(&inc_session_id))
            .get_result::<Report>(conn)
            .optional(),
    }
}
