use crate::request::prelude::*;
use twilight_model::id::ChannelId;

/// Clear the permissions for a target ID in a channel.
///
/// The `target_id` is a `u64`, but it should point to a `RoleId` or a `UserId`.
pub struct DeleteChannelPermissionConfigured<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<String>,
    target_id: u64,
}

impl<'a> DeleteChannelPermissionConfigured<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, target_id: u64) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            reason: None,
            target_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        let mut request = Request::builder(Route::DeletePermissionOverwrite {
            channel_id: self.channel_id.0,
            target_id: self.target_id,
        });

        if let Some(reason) = &self.reason {
            request = request.headers(audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.verify(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteChannelPermissionConfigured<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteChannelPermissionConfigured<'_>, ());
