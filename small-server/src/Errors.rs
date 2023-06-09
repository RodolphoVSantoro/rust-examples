pub trait UnwrapPrint<T> {
    fn unwrap_print(self) -> T;
}

pub enum DatabaseConnectionError {
    VarError(std::env::VarError),
    ConnectionError(sqlx::Error),
}

impl From<std::env::VarError> for DatabaseConnectionError {
    fn from(error: std::env::VarError) -> Self {
        Self::VarError(error)
    }
}

impl From<sqlx::Error> for DatabaseConnectionError {
    fn from(error: sqlx::Error) -> Self {
        Self::ConnectionError(error)
    }
}

impl ToString for DatabaseConnectionError {
    fn to_string(&self) -> String {
        match self {
            DatabaseConnectionError::VarError(error) => {
                format!("Failed to read environment variable: {}", error)
            }
            DatabaseConnectionError::ConnectionError(error) => {
                format!("Failed to connect to database: {}", error)
            }
        }
    }
}

impl<T, E> UnwrapPrint<T> for Result<T, E>
where
    E: ToString,
{
    fn unwrap_print(self) -> T {
        return self.unwrap_or_else(|error| panic!("{}", error.to_string()));
    }
}
