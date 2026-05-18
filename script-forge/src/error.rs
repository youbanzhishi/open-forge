use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("YAML parse error: {0}")]
    YamlParse(#[from] serde_yaml::Error),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("unknown component type: {0}")]
    UnknownComponent(String),

    #[error("unknown action type: {0}")]
    UnknownAction(String),

    #[error("unknown condition type: {0}")]
    UnknownCondition(String),
}
