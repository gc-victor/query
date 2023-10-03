use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Manage branches
    Branch(BranchArgs),
    /// Push migrations using a migration file
    /// - The migration file should be in the format of <version>_<name>_<type>.db|.sql
    /// - The version should be in the format of YYYYMMDD
    /// - The name should be in the format of <name>_<description>
    /// - The type should be up or down
    #[clap(verbatim_doc_comment)]
    Migration(MigrationArgs),
    /// Sets the initial configuration
    Settings,
    /// SQLite shell to manage the databases locally
    Shell(ShellArgs),
    /// Manage tokens without relation to a user
    Token(TokenArgs),
    /// Manage users
    User(UserArgs),
    /// Manage users tokens
    UserToken(UserTokenArgs),
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
    /// Update user password
    Password,
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
