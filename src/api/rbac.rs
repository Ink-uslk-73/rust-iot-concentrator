//! src/api/rbac.rs - RBAC middleware (P3.43).

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Engineer,
    Auditor,
    OnDuty,
    Llm,
}

impl Role {
    pub fn from_header(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "engineer" => Some(Role::Engineer),
            "auditor" => Some(Role::Auditor),
            "onduty" => Some(Role::OnDuty),
            "llm" => Some(Role::Llm),
            _ => None,
        }
    }
    pub fn can_write(self) -> bool {
        matches!(self, Role::Engineer | Role::OnDuty)
    }
}

pub async fn require_role(req: Request, next: Next) -> Result<Response, StatusCode> {
    let role = req
        .headers()
        .get("X-Role")
        .and_then(|v| v.to_str().ok())
        .and_then(Role::from_header)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if !role.can_write() {
        return Err(StatusCode::FORBIDDEN);
    }
    let mut req = req;
    req.extensions_mut().insert(role);
    Ok(next.run(req).await)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_header_known() {
        assert_eq!(Role::from_header("engineer"), Some(Role::Engineer));
        assert_eq!(Role::from_header("Engineer"), Some(Role::Engineer));
        assert_eq!(Role::from_header("auditor"), Some(Role::Auditor));
        assert_eq!(Role::from_header("onduty"), Some(Role::OnDuty));
        assert_eq!(Role::from_header("llm"), Some(Role::Llm));
    }

    #[test]
    fn from_header_unknown() {
        assert_eq!(Role::from_header(""), None);
        assert_eq!(Role::from_header("admin"), None);
        assert_eq!(Role::from_header("root"), None);
    }

    #[test]
    fn can_write_matrix() {
        assert!(Role::Engineer.can_write());
        assert!(Role::OnDuty.can_write());
        assert!(!Role::Auditor.can_write());
        assert!(!Role::Llm.can_write());
    }
}