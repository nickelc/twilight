use crate::request::prelude::*;
use twilight_model::{channel::GuildChannel, id::GuildId};

/// Get the channels in a guild.
pub struct GetGuildChannels<'a> {
    fut: Option<Pending<'a, Vec<GuildChannel>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildChannels<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetChannels {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildChannels<'_>, Vec<GuildChannel>);
