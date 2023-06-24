use crate::{
    diesel::ExpressionMethods,
    dto::{
        dive_dto::{Dive, DiveFilter},
        query_dto::QueryParams,
    },
};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

pub fn get_dives_by_user(
    conn: &mut PgConnection,
    input_user_id: Uuid,
    dive_query_input: Option<DiveFilter>,
    db_query_ob: Option<QueryParams>,
) -> diesel::QueryResult<Vec<Dive>> {
    use crate::schema::dives::dsl::{dives, user_id};

    dives
        .filter(user_id.eq(&input_user_id))
        .get_results::<Dive>(conn)
}
