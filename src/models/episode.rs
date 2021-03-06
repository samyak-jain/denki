use crate::models::show;
use crate::schema::episodes;
use serde::Serialize;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Serialize, Associations, Debug)]
#[belongs_to(show::Show)]
#[table_name = "episodes"]
pub struct Episode {
    pub id: i32,
    pub show_id: i32,
    pub name: String,
    pub thumbnail: Option<String>,
    pub file_path: String,
    pub locator_id: Uuid,
    pub episode_number: Option<i32>,
}
