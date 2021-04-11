#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApprovalActorIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "principalType", skip_serializing)]
    pub principal_type: Option<role_assignment_approval_actor_identity::PrincipalType>,
    #[serde(rename = "principalName", skip_serializing)]
    pub principal_name: Option<String>,
    #[serde(rename = "userPrincipalName", skip_serializing)]
    pub user_principal_name: Option<String>,
}
pub mod role_assignment_approval_actor_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrincipalType {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "servicePrincipal")]
        ServicePrincipal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApproval {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentApprovalProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApprovalProperties {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<RoleAssignmentApprovalStep>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApprovalListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentApproval>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApprovalStep {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentApprovalStepProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApprovalStepProperties {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<role_assignment_approval_step_properties::Status>,
    #[serde(rename = "assignedToMe", skip_serializing)]
    pub assigned_to_me: Option<bool>,
    #[serde(rename = "reviewedBy", skip_serializing_if = "Option::is_none")]
    pub reviewed_by: Option<RoleAssignmentApprovalActorIdentity>,
    #[serde(rename = "reviewedDateTime", skip_serializing)]
    pub reviewed_date_time: Option<String>,
    #[serde(rename = "reviewResult", skip_serializing_if = "Option::is_none")]
    pub review_result: Option<role_assignment_approval_step_properties::ReviewResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
}
pub mod role_assignment_approval_step_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        InProgress,
        Completed,
        Expired,
        Initializing,
        Escalating,
        Completing,
        Escalated,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReviewResult {
        Approve,
        Deny,
        NotReviewed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentApprovalStepListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentApprovalStep>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinitionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDefinitionProperties {
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplay {
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}