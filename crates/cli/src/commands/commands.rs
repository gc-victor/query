use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Push all the assets without setting a path
    /// or an asset defining a file path
    #[clap(verbatim_doc_comment)]
    Asset(AssetArgs),
    /// Manage branches
    Branch(BranchArgs),
    /// Create a new project
    Create,
    /// Development experience
    Dev(DevArgs),
    /// Push all the functions without setting a path
    /// or a function defining a file path
    #[clap(verbatim_doc_comment)]
    Function(FunctionArgs),
    /// Create code automatically
    #[clap(verbatim_doc_comment)]
    Generate(GenerateArgs),
    /// Push migrations using a migration file
    /// - The migration file should be in the format of <version>_<name>_<type>.db|.sql
    /// - The version should be in the format of YYYYMMDD
    /// - The name should be in the format of <name>_<description>
    /// - The type should be up or down
    #[clap(verbatim_doc_comment)]
    Migration(MigrationArgs),
    /// Deletes all the cached functions
    Purge,
    /// Sets the initial configuration
    Settings,
    /// SQLite shell to manage the databases locally
    Shell(ShellArgs),
    /// Execution of custom commands defined in the Query.toml file
    Task(TaskArgs),
    /// Manage tokens without relation to a user
    Token(TokenArgs),
    /// Manage users
    User(UserArgs),
    /// Manage users tokens
    UserToken(UserTokenArgs),
}

#[derive(Args)]
pub struct AssetArgs {
    /// Activate status of the asset
    #[arg(short, long, default_value_t = String::from("true"))]
    pub active: String,
    /// Delete the asset
    /// It is mandatory to provide the path to the asset
    #[arg(short, long, default_value_t = false)]
    pub delete: bool,
    /// Path to the assets
    pub path: Option<String>,
}

#[derive(Args)]
pub struct BranchArgs {
    #[command(subcommand)]
    pub command: BranchCommands,
}

#[derive(Subcommand)]
pub enum BranchCommands {
    /// Create a branch
    Create,
    /// Delete a branch
    Delete,
    /// List all the branches
    List,
}

#[derive(Args)]
pub struct DevArgs {
    /// Clean assets and function databases, and dist folder
    #[arg(short, long, default_value_t = false)]
    pub clean: bool,
    /// Show all the logs
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct FunctionArgs {
    /// Path to the function definition file
    pub path: Option<String>,
    /// Delete the function definition
    #[arg(short, long, default_value_t = false)]
    pub delete: bool,
}

#[derive(Args)]
pub struct GenerateArgs {
    pub database: String,
    pub table: String,
    pub columns: Vec<String>,
}

#[derive(Args)]
pub struct MigrationArgs {
    /// Name of the database to migrate
    pub db_name: String,
    /// Path to the migration file
    pub path: String,
}

#[derive(Args)]
pub struct ShellArgs {
    /// Name of the database to open
    pub db_name: String,
}

#[derive(Args)]
pub struct TaskArgs {
    /// List all the tasks
    #[arg(short, long, default_value_t = false)]
    pub list: bool,
    /// Name of the task to execute
    pub task: Vec<String>,
    /// Confirm the execution of the task
    #[arg(short, long, default_value_t = false)]
    pub yes: bool,
}

#[derive(Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub command: TokenCommands,
}

#[derive(Subcommand)]
pub enum TokenCommands {
    /// Create a token
    Create,
    /// Delete a token
    Delete,
    /// List all the tokens
    List,
    /// Update token
    Update,
    /// Get the token value
    Value,
}

#[derive(Args)]
pub struct UserArgs {
    #[command(subcommand)]
    pub command: UserCommands,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Create a user
    Create,
    /// Delete a user
    Delete,
    /// List all the users
    List,
    /// Update user
    Update,
}

#[derive(Args)]
pub struct UserTokenArgs {
    #[command(subcommand)]
    pub command: UserTokenCommands,
}

#[derive(Subcommand)]
pub enum UserTokenCommands {
    /// Create a user token
    Create,
    /// Delete a user token
    Delete,
    /// List all the users tokens
    List,
    /// Update user token
    Update,
    /// Get the token value related to a user
    Value,
}
