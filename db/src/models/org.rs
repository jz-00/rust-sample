use super::{UserId, UserRole};

id64!(pub OrgId);

#[derive(Debug)]
pub struct Org {
    pub id: OrgId,
    pub created: time::OffsetDateTime,
    pub members: Vec<OrgMember>
}

id64!(pub OrgMemberId);

#[derive(Debug)]
pub struct OrgMember {
    pub id: OrgMemberId,
    pub created: time::OffsetDateTime,
    pub org_id: OrgId,
    pub user_id: UserId,
    pub role: UserRole,
}
