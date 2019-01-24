use structopt::StructOpt;

use notion_core::session::{ActivityKind, Session};
use notion_core::tool::ToolSpec;
use notion_core::version::VersionSpec;
use notion_fail::{ExitCode, Fallible};

use crate::command::Command;

#[derive(StructOpt)]
pub(crate) struct Fetch {
    /// The tool to install, e.g. `node` or `npm` or `yarn`
    tool: String,

    /// The version of the tool to install, e.g. `1.2.3` or `latest`
    version: String,
}

impl Command for Fetch {
    fn run(self, session: &mut Session) -> Fallible<ExitCode> {
        session.add_event_start(ActivityKind::Fetch);
        match self {
            Fetch::Help => Help::Command(CommandName::Fetch).run(session)?,
            Fetch::Tool(toolspec) => {
                // TODO: this should be matched here then...
                session.fetch(&toolspec)?;
            }
        };
        session.add_event_end(ActivityKind::Fetch, ExitCode::Success);
        Ok(ExitCode::Success)
    }
}
