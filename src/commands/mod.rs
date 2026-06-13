pub mod history;
pub mod init;
pub mod save;

pub enum Command {
    Init,
    Save { message: String },
    History,
}