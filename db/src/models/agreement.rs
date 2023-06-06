use super::{UserId, OrgMemberId}

id64!(pub AgreementId);

#[derive(Debug)]
pub struct Agreement {
    pub id: AgreementId,
    pub created: time::OffsetDateTime,
    pub accepted: Option<time::OffsetDateTime>,
    pub user_id: UserId,
    pub org_member_id: OrgMemberId,
    pub title: String,
    pub version: i16,
}
