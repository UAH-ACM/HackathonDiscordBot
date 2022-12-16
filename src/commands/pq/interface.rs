use super::super::super::schema::team_seaking::dsl::*;
use diesel::prelude::*;

use super::super::super::models::TeamSeaking;

pub fn insert(conn: &mut PgConnection, id_loc: &i64, name_loc: &str, discord_name: &str, description_loc: &str) -> TeamSeaking {
    use crate::schema::team_seaking;

    let new_seaker = TeamSeaking {
        id: * id_loc,
		name: String::from(name_loc),
		discordname: String::from(discord_name),
		description: String::from(description_loc),
    };

    diesel::insert_into(team_seaking::table)
        .values(&new_seaker)
        .get_result(conn)
        .expect("Error saving new seaker")
}

pub fn delete_row(conn: &mut PgConnection, search: String) -> usize {
    diesel::delete(team_seaking.filter(discordname.like(search)))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn delete_table(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(team_seaking).execute(conn)
}