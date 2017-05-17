use postgres::{Connection, TlsMode};
use postgres::rows::Rows;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct Database {
    connection: Connection,
}
impl Database {

    pub fn new(user: &str, password: &str, host: &str, name: &str) -> Database {
        let connection_str = build_connection_str(user, password, host, name);
        let db_connection = Connection::connect(connection_str, TlsMode::None).unwrap();
        Database { connection : db_connection }
    }

    pub fn insert_user(&self, user: User) {
        self.connection.execute("INSERT INTO users (email, first_name, last_name) VALUES ($1, $2, $3)",
                     &[&user.email, &user.first_name, &user.last_name]).unwrap();
    }

    pub fn get_users(&self) -> Rows<> {
        self.connection.query("SELECT email, first_name, last_name FROM users", &[]).unwrap()
    }

    pub fn delete_user(&self, email: String) {
        self.connection.execute("DELETE FROM users WHERE email = $1", &[&email]).unwrap();
    }

}

fn build_connection_str(user: &str, password: &str, host: &str, name: &str) -> String {
    let mut connection_str = String::new();
    connection_str += "postgresql://";
    connection_str += user;
    connection_str += ":";
    connection_str += password;
    connection_str += "@";
    connection_str += host;
    connection_str += ":5432/";
    connection_str += name;

    return connection_str;
}
