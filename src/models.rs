use crate::schema::team_seaking;
use crate::schema::teams;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Insertable, Eq, PartialEq, Debug)]
#[diesel(table_name = team_seaking)]
pub struct TeamSeaking {
    pub id: i64,
    pub name: String,
    pub discordname: String,
    pub description: String,
}

#[derive(Identifiable, Queryable, Insertable, Eq, PartialEq, Debug, AsChangeset)]
#[diesel(table_name = teams)]
pub struct Teams {
    pub id: i64,
    pub team_name: String,
    pub description: String,
    pub leader: String,
    pub members: String,
    pub points: i64,
	pub role_id: i64,
}
