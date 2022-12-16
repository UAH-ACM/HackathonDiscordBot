use diesel::prelude::*;

use super::super::super::models::TeamSeaking;

pub fn insert(conn: &mut PgConnection, id: &i64, name: &str, discord_name: &str, description: &str) -> TeamSeaking {
    use crate::schema::team_seaking;

    let new_seaker = TeamSeaking {
        id: * id,
		name: String::from(name),
		discordname: String::from(discord_name),
		description: String::from(description),
    };

    diesel::insert_into(team_seaking::table)
        .values(&new_seaker)
        .get_result(conn)
        .expect("Error saving new seaker")
}