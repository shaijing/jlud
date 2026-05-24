use cygnus::{
    args::{Args, ArgsCommand, Parser},
    auth::auth_command_resolver,
    user::user_command_resolver,
};
use time::{UtcOffset, macros::format_description};
use tracing::{Level, error};
use tracing_subscriber::fmt::time::OffsetTime;

fn main() {
    let args = Args::parse();

    match args.command {
        ArgsCommand::User(usr_args) => {
            user_command_resolver(usr_args).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
        }
        ArgsCommand::Auth(auth_args) => {
            let log_level: Level = auth_args.log_level.clone().into();
            let offset =
                UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
            let timer = OffsetTime::new(
                offset,
                format_description!(
                    "[year]-[month]-[day] [hour]:[minute]:[second]"
                ),
            );
            let subscriber = tracing_subscriber::fmt()
                .with_timer(timer)
                .with_max_level(log_level)
                .finish();
            tracing::subscriber::set_global_default(subscriber).unwrap_or_else(
                |e| {
                    eprintln!("Failed to set default subscriber: {}", e);
                    eprintln!("App will continue without logging");
                },
            );
            auth_command_resolver(auth_args, None).unwrap_or_else(|e| {
                error!("Error when running auth command: {}", e);
                std::process::exit(1);
            });
        }
    }
}
