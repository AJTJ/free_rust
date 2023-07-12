use crate::{
    apnea_forms::dto::report_dto::{Report, ReportsRetrievalData},
    utility::errors::{BigError, DieselQuerySnafu},
};
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use snafu::ResultExt;
use uuid::Uuid;

pub fn get_reports(
    conn: &mut PgConnection,
    retrieval_ids: Vec<ReportsRetrievalData>,
) -> Result<Option<Vec<Report>>, BigError> {
    use crate::schema::reports::dsl::{reports, session_id, user_id as forms_user_id};

    let mut session_ids: Vec<Uuid> = vec![];
    let mut user_ids: Vec<Uuid> = vec![];
    for variant in retrieval_ids.into_iter() {
        match variant {
            ReportsRetrievalData::SessionId(inner_id) => session_ids.push(inner_id),
            ReportsRetrievalData::UserId(inner_id) => user_ids.push(inner_id),
        }
    }

    let query = reports
        .filter(session_id.eq_any(session_ids))
        .or_filter(forms_user_id.eq_any(user_ids))
        .into_boxed();

    let my_reports = query
        .get_results::<Report>(conn)
        .optional()
        .context(DieselQuerySnafu);

    my_reports
}
