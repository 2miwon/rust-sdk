#[derive(Debug)]
pub struct AwsConfig {
    pub region: &'static str,
    pub access_key_id: &'static str,
    pub secret_access_key: &'static str,
}

impl Default for AwsConfig {
    fn default() -> Self {
        AwsConfig {
            region: option_env!("AWS_REGION").expect("You must set AWS_REGION"),
            access_key_id: option_env!("AWS_ACCESS_KEY_ID")
                .expect("You must set AWS_ACCESS_KEY_ID"),
            secret_access_key: option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required"),
        }
    }
}

#[derive(Debug)]
pub enum DatabaseConfig {
    DynamoDb { table_name: &'static str },
    Postgres { url: &'static str },
    Sqlite { url: &'static str },
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        match option_env!("DATABASE_TYPE")
            .expect("You must set DATABASE_TYPE")
            .to_lowercase()
            .as_str()
        {
            "dynamo" | "dynamodb" => DatabaseConfig::DynamoDb {
                table_name: option_env!("TABLE_NAME").expect("You must set TABLE_NAME"),
            },
            "rds" | "postgres" => DatabaseConfig::Postgres {
                url: option_env!("DATABASE_URL").expect("You must set DATABASE_URL"),
            },
            "sqlite" => DatabaseConfig::Sqlite {
                url: option_env!("DATABASE_URL").expect("You must set DATABASE_URL"),
            },
            _ => panic!("DATABASE_TYPE must be dynamodb or postgres"),
        }
    }
}
