//! Stub implementation for warp_graphql crate
//! This provides minimal types to satisfy compilation without actual GraphQL functionality

use serde::{Deserialize, Serialize};

pub mod client {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default)]
    pub struct RequestOptions {
        pub path_prefix: Option<String>,
    }

    pub fn get_request_context() -> Option<String> {
        None
    }

    pub fn get_user_facing_error_message(_error: &GraphQLError) -> String {
        "GraphQL error".to_string()
    }

    #[derive(Debug, Clone)]
    pub struct GraphQLError {
        pub message: String,
    }

    pub trait Operation<T> {
        fn build(_variables: T) -> Self;
    }
}

pub mod scalars {
    pub mod time {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ServerTimestamp(pub String);
    }

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Time(pub String);
}

pub mod billing {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StripeSubscriptionPlan {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PlanPricing {
        pub amount: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddonCreditsOption {
        pub credits: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiAutonomyPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AmbientAgentsPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BillingCycleUsageHistory {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BillingMetadata {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BonusGrant {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ByoApiKeyPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CodebaseContextPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum CustomerType {
        Individual,
        Team,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DelinquencyStatus {
        None,
        PastDue,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EnterpriseCreditsAutoReloadPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EnterprisePayAsYouGoPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum InstanceShape {
        Small,
        Medium,
        Large,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MultiAdminPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PurchaseAddOnCreditsPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ServiceAgreementType {
        None,
        Bsl,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SessionSharingPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SharedNotebooksPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SharedWorkflowsPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TeamSizePolicy {
        pub max: u32,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TelemetryDataCollectionPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Pro,
        Team,
        Enterprise,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UgcDataCollectionPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UsageBasedPricingPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum UsageVisibilityGranularity {
        None,
        Daily,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UsageVisibilityPolicy {
        pub granularity: UsageVisibilityGranularity,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WarpAiPolicy {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddonCreditAutoReloadStatus {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceAgreement {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AiCreditsUsageAndCostSubjectType {
        User,
        Workspace,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AiCreditsUsageAndCostType {
        Usage,
        Cost,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiCreditsUsageBucket {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AiCreditsUsageSource {
        Api,
        Ui,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PricingInfo {
        pub id: String,
    }
}

pub mod error {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BudgetExceededError {
        pub message: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserFacingErrorInterface {
        pub message: String,
    }
}

pub mod object_permissions {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AccessLevel {
        pub level: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum OwnerType {
        User,
        Team,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Owner {
        pub id: String,
    }
}

pub mod experiment {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Experiment {
        pub id: String,
    }
}

pub mod generic_string_object {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GenericStringObjectFormat {
        pub format: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GenericStringObject {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GenericStringObjectInput {
        pub id: String,
    }
}

pub mod mutations {
    use serde::{Deserialize, Serialize};

    pub mod create_anonymous_user {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AnonymousUserType {
            pub type_: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum AnonymousUserExpirationType {
            Default,
            Extended,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAnonymousUser {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAnonymousUserInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAnonymousUserResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAnonymousUserVariables {
            pub id: String,
        }
    }

    pub mod update_generic_string_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateGenericStringObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateGenericStringObjectPayload {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenericStringObjectUpdate {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateGenericStringObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateGenericStringObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateGenericStringObjectVariables {
            pub id: String,
        }
    }

    pub mod add_invite_link_domain_restriction {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddInviteLinkDomainRestrictionInput {
            pub domain: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddInviteLinkDomainRestriction {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddInviteLinkDomainRestrictionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddInviteLinkDomainRestrictionVariables {
            pub id: String,
        }
    }

    pub mod create_team {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateTeamInput {
            pub name: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateTeam {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateTeamVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum CloudObjectEventEntrypoint {
            Unknown,
        }
    }

    pub mod delete_invite_link_domain_restriction {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteInviteLinkDomainRestrictionInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteInviteLinkDomainRestriction {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteInviteLinkDomainRestrictionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteInviteLinkDomainRestrictionVariables {
            pub id: String,
        }
    }

    pub mod delete_team_invite {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteTeamInviteInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteTeamInvite {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteTeamInviteResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteTeamInviteVariables {
            pub id: String,
        }
    }

    pub mod join_team_with_team_discovery {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct JoinTeamWithTeamDiscoveryInput {
            pub team_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct JoinTeamWithTeamDiscovery {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct JoinTeamWithTeamDiscoveryResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct JoinTeamWithTeamDiscoveryVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum TeamDiscoveryEntrypoint {
            None,
        }
    }

    pub mod remove_user_from_team {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveUserFromTeamInput {
            pub user_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveUserFromTeam {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveUserFromTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveUserFromTeamVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum CloudObjectEventEntrypoint {
            Unknown,
        }
    }

    pub mod rename_team {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RenameTeamInput {
            pub name: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RenameTeam {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RenameTeamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RenameTeamVariables {
            pub id: String,
        }
    }

    pub mod reset_invite_links {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ResetInviteLinksInput {
            pub team_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ResetInviteLinks {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ResetInviteLinksResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ResetInviteLinksVariables {
            pub id: String,
        }
    }

    pub mod send_team_invite_email {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendTeamInviteEmailInput {
            pub email: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendTeamInviteEmail {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendTeamInviteEmailResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendTeamInviteEmailVariables {
            pub id: String,
        }
    }

    pub mod set_is_invite_link_enabled {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetIsInviteLinkEnabledInput {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetIsInviteLinkEnabled {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetIsInviteLinkEnabledResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetIsInviteLinkEnabledVariables {
            pub id: String,
        }
    }

    pub mod share_block {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ShareBlockInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ShareBlockResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DisplaySetting {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BlockInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ShareBlock {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ShareBlockVariables {
            pub id: String,
        }
    }

    pub mod confirm_file_artifact_upload {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConfirmFileArtifactUploadInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConfirmFileArtifactUploadResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConfirmFileArtifactUpload {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConfirmFileArtifactUploadVariables {
            pub id: String,
        }
    }

    pub mod create_agent_task {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAgentTaskInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAgentTaskResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAgentTask {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateAgentTaskVariables {
            pub id: String,
        }
    }

    pub mod create_file_artifact_upload_target {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFileArtifactUploadTargetInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFileArtifactUploadTargetResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFileArtifactUploadTarget {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFileArtifactUploadTargetVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileArtifactUploadFieldValue {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileArtifact {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FileArtifactUploadField {
            pub id: String,
        }
    }

    pub mod delete_ai_conversation {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteAIConversationInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteAIConversationResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteAIConversation {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteAIConversationVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteConversationInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteConversationResult {
            pub success: bool,
        }
    }

    pub mod generate_code_embeddings {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCodeEmbeddingsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCodeEmbeddingsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCodeEmbeddings {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCodeEmbeddingsVariables {
            pub id: String,
        }
    }

    pub mod generate_commands {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCommandsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCommandsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCommands {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateCommandsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum GenerateCommandsStatus {
            Pending,
            Complete,
        }
    }

    pub mod generate_dialogue {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateDialogueInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateDialogueResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateDialogue {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateDialogueVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum GenerateDialogueStatus {
            Pending,
            Complete,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TranscriptPart {
            pub text: String,
        }
    }

    pub mod generate_metadata_for_command {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateMetadataForCommandInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateMetadataForCommandResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateMetadataForCommand {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateMetadataForCommandVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum GenerateMetadataForCommandStatus {
            Pending,
            Complete,
        }
    }

    pub mod populate_merkle_tree_cache {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PopulateMerkleTreeCacheInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PopulateMerkleTreeCacheResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PopulateMerkleTreeCache {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PopulateMerkleTreeCacheVariables {
            pub id: String,
        }
    }

    pub mod request_bonus {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RequestBonusInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RequestBonusResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ProvideNegativeFeedbackResponseForAiConversation {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ProvideNegativeFeedbackResponseForAiConversationInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ProvideNegativeFeedbackResponseForAiConversationVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RequestsRefundedResult {
            pub refunded: bool,
        }
    }

    pub mod update_agent_task {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateAgentTaskInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateAgentTaskResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateAgentTask {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateAgentTaskVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AgentTaskStatusMessageInput {
            pub message: String,
        }
    }

    pub mod update_merkle_tree {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateMerkleTreeInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateMerkleTreeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateMerkleTree {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateMerkleTreeVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MerkleTreeNode {
            pub hash: String,
        }
    }

    pub mod expire_api_key {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ExpireApiKeyInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ExpireApiKeyResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ExpireApiKey {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ExpireApiKeyVariables {
            pub id: String,
        }
    }

    pub mod generate_api_key {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateApiKeyInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateApiKeyResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateApiKey {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GenerateApiKeyVariables {
            pub id: String,
        }
    }

    pub mod mint_custom_token {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MintCustomToken {
            pub token: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MintCustomTokenInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MintCustomTokenResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MintCustomTokenVariables {
            pub id: String,
        }
    }

    pub mod set_user_is_onboarded {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetUserIsOnboardedInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetUserIsOnboardedResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetUserIsOnboarded {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetUserIsOnboardedVariables {
            pub id: String,
        }
    }

    pub mod update_user_settings {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateUserSettingsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateUserSettingsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateUserSettings {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateUserSettingsVariables {
            pub id: String,
        }
    }

    pub mod unshare_block {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UnshareBlockInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UnshareBlockResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UnshareBlock {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UnshareBlockVariables {
            pub id: String,
        }
    }

    pub mod create_simple_integration {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateSimpleIntegrationInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateSimpleIntegrationResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateSimpleIntegration {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateSimpleIntegrationOutput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateSimpleIntegrationVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIntegrationConfig {
            pub id: String,
        }
    }

    pub mod create_managed_secret {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateManagedSecretInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateManagedSecretResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateManagedSecret {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateManagedSecretVariables {
            pub id: String,
        }
    }

    pub mod delete_managed_secret {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteManagedSecretInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteManagedSecretResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteManagedSecret {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteManagedSecretVariables {
            pub id: String,
        }
    }

    pub mod update_managed_secret {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateManagedSecretInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateManagedSecretResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateManagedSecret {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateManagedSecretVariables {
            pub id: String,
        }
    }

    pub mod issue_task_identity_token {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct IssueTaskIdentityTokenInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct IssueTaskIdentityTokenResult {
            pub token: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct IssueTaskIdentityToken {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct IssueTaskIdentityTokenVariables {
            pub id: String,
        }
    }

    pub mod add_object_guests {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddObjectGuestsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddObjectGuestsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddObjectGuests {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddObjectGuestsVariables {
            pub id: String,
        }
    }

    pub mod remove_object_guests {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuestsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuestsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuests {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuestsVariables {
            pub id: String,
        }
    }

    pub mod bulk_create_objects {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkCreateObjectsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkCreateObjectsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkCreateGenericStringObjectsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkCreateObjects {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct BulkCreateObjectsVariables {
            pub id: String,
        }
    }

    pub mod create_folder {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFolderInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFolderResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFolder {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateFolderVariables {
            pub id: String,
        }
    }

    pub mod create_generic_string_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateGenericStringObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateGenericStringObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateGenericStringObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateGenericStringObjectVariables {
            pub id: String,
        }
    }

    pub mod create_notebook {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateNotebookInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateNotebookResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateNotebook {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateNotebookVariables {
            pub id: String,
        }
    }

    pub mod create_workflow {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateWorkflowInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateWorkflowResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateWorkflow {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CreateWorkflowVariables {
            pub id: String,
        }
    }

    pub mod delete_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct DeleteObjectVariables {
            pub id: String,
        }
    }

    pub mod empty_trash {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EmptyTrashInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EmptyTrashResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EmptyTrash {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct EmptyTrashVariables {
            pub id: String,
        }
    }

    pub mod give_up_notebook_edit_access {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GiveUpNotebookEditAccessInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GiveUpNotebookEditAccessResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GiveUpNotebookEditAccess {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GiveUpNotebookEditAccessVariables {
            pub id: String,
        }
    }

    pub mod grab_notebook_edit_access {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GrabNotebookEditAccessInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GrabNotebookEditAccessResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GrabNotebookEditAccess {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GrabNotebookEditAccessVariables {
            pub id: String,
        }
    }

    pub mod leave_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LeaveObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LeaveObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LeaveObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LeaveObjectVariables {
            pub id: String,
        }
    }

    pub mod move_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MoveObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MoveObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MoveObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct MoveObjectVariables {
            pub id: String,
        }
    }

    pub mod record_object_action {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RecordObjectActionInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RecordObjectActionResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RecordObjectAction {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RecordObjectActionVariables {
            pub id: String,
        }
    }

    pub mod transfer_generic_string_object_owner {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferGenericStringObjectOwnerInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferGenericStringObjectOwnerResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferGenericStringObjectOwner {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferGenericStringObjectOwnerVariables {
            pub id: String,
        }
    }

    pub mod transfer_notebook_owner {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferNotebookOwnerInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferNotebookOwnerResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferNotebookOwner {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferNotebookOwnerVariables {
            pub id: String,
        }
    }

    pub mod transfer_workflow_owner {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferWorkflowOwnerInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferWorkflowOwnerResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferWorkflowOwner {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferWorkflowOwnerVariables {
            pub id: String,
        }
    }

    pub mod trash_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TrashObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TrashObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TrashObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TrashObjectVariables {
            pub id: String,
        }
    }

    pub mod untrash_object {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UntrashObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UntrashObjectResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UntrashObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UntrashObjectVariables {
            pub id: String,
        }
    }

    pub mod update_folder {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateFolderInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateFolderResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateFolder {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateFolderVariables {
            pub id: String,
        }
    }

    pub mod update_notebook {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct NotebookUpdate {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateNotebook {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateNotebookVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateNotebookInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateNotebookResult {
            pub success: bool,
        }
    }

    pub mod update_object_guests {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateObjectGuests {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateObjectGuestsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateObjectGuestsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateObjectGuestsResult {
            pub success: bool,
        }
    }

    pub mod update_workflow {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct WorkflowUpdate {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkflow {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkflowVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkflowInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkflowResult {
            pub success: bool,
        }
    }

    pub mod remove_object_guest {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuest {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuestVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuestInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectGuestResult {
            pub success: bool,
        }
    }

    pub mod remove_object_link_permissions {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectLinkPermissions {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectLinkPermissionsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectLinkPermissionsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RemoveObjectLinkPermissionsResult {
            pub success: bool,
        }
    }

    pub mod set_object_link_permissions {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetObjectLinkPermissions {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetObjectLinkPermissionsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetObjectLinkPermissionsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetObjectLinkPermissionsResult {
            pub success: bool,
        }
    }

    pub mod send_referral_invite_emails {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendReferralInviteEmails {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendReferralInviteEmailsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendReferralInviteEmailsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SendReferralInviteEmailsResult {
            pub success: bool,
        }
    }

    pub mod set_team_discoverability {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamDiscoverability {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamDiscoverabilityVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamDiscoverabilityInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamDiscoverabilityResult {
            pub success: bool,
        }
    }

    pub mod set_team_member_role {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamMemberRole {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamMemberRoleVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamMemberRoleInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SetTeamMemberRoleResult {
            pub success: bool,
        }
    }

    pub mod transfer_team_ownership {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferTeamOwnership {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferTeamOwnershipVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferTeamOwnershipInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TransferTeamOwnershipResult {
            pub success: bool,
        }
    }

    pub mod purchase_addon_credits {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PurchaseAddonCredits {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PurchaseAddonCreditsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PurchaseAddonCreditsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PurchaseAddonCreditsResult {
            pub success: bool,
        }
    }

    pub mod stripe_billing_portal {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StripeBillingPortal {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StripeBillingPortalVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StripeBillingPortalInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct StripeBillingPortalResult {
            pub success: bool,
        }
    }

    pub mod update_workspace_settings {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkspaceSettings {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkspaceSettingsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkspaceSettingsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdateWorkspaceSettingsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AddonCreditsSettingsInput {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UsageBasedPricingSettingsInput {
            pub enabled: bool,
        }
    }
}

pub mod object {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ObjectUpdateSuccess {
        pub success: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CloudObjectWithDescendants {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum CloudObject {
        AIConversation(CloudAIConversation),
        Folder(super::folder::Folder),
        Notebook(CloudNotebook),
        Workflow(CloudWorkflow),
        EnvVarCollection(CloudEnvVarCollection),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CloudAIConversation {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CloudNotebook {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CloudWorkflow {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CloudEnvVarCollection {
        pub id: String,
    }
}

pub mod folder {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Folder {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FolderItem {
        pub id: String,
    }
}

pub mod workspace {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DisableReason {
        AdminDisabled,
        OutOfRequests,
        Other(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum LlmProvider {
        Openai,
        Anthropic,
        Google,
        Xai,
        Unknown,
        Other(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LlmModelHost {
        pub host: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LlmHostSettings {
        pub settings: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AddonCreditsSettings {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AdminEnablementSetting {
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AiAutonomyValue {
        None,
        Limited,
        Full,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AiPermissionsSettings {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ComputerUseAutonomyValue {
        None,
        Limited,
        Full,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmailInvite {
        pub email: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum HostEnablementSetting {
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InviteLinkDomainRestriction {
        pub domain: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum MembershipRole {
        Owner,
        Admin,
        Member,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Team {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TeamMember {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum UgcCollectionEnablementSetting {
        Enabled,
        Disabled,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Workspace {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WorkspaceMember {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WorkspaceMemberUsageInfo {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WorkspaceSettings {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum WriteToPtyAutonomyValue {
        None,
        Limited,
        Full,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum FeatureModelChoice {
        Default,
        Custom,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LlmInfo {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AvailableLlms {
        pub llms: Vec<LlmInfo>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LlmSpec {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LlmUsageMetadata {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RoutingHostConfig {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LlmSettings {
        pub id: String,
    }
}

pub mod queries {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetWorkspaceQuery {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetFeatureModelChoices {
        pub choices: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GetFeatureModelChoicesVariables {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum UserResult {
        UserOutput(String),
        Unknown,
    }

    pub mod get_feature_model_choices {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum DisableReason {
            AdminDisabled,
            OutOfRequests,
            Other(String),
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum LlmModelHost {
            DirectApi,
            AwsBedrock,
            Other(String),
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetFeatureModelChoices {
            pub choices: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetFeatureModelChoicesVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum LlmProvider {
            Openai,
            Anthropic,
            Google,
            Xai,
            Unknown,
            Other(String),
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LlmInfo {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AvailableLlms {
            pub llms: Vec<LlmInfo>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LlmSpec {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct LlmUsageMetadata {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RoutingHostConfig {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FeatureModelChoice {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct User {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserOutput {
            pub id: String,
        }
    }

    pub mod get_request_limit_info {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetRequestLimitInfo {
            pub limit: u32,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetRequestLimitInfoVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_available_harnesses {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAvailableHarnesses {
            pub harnesses: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAvailableHarnessesVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_relevant_fragments {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetRelevantFragmentsQuery {
            pub fragments: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetRelevantFragmentsResult {
            pub result: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetRelevantFragmentsVariables {
            pub id: String,
        }
    }

    pub mod get_scheduled_agent_history {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetScheduledAgentHistory {
            pub history: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetScheduledAgentHistoryVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ScheduledAgentHistory {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ScheduledAgentHistoryInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ScheduledAgentHistoryResult {
            pub result: String,
        }
    }

    pub mod rerank_fragments {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RerankFragments {
            pub fragments: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RerankFragmentsResult {
            pub result: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RerankFragmentsVariables {
            pub id: String,
        }
    }

    pub mod sync_merkle_tree {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SyncMerkleTree {
            pub tree: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SyncMerkleTreeInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SyncMerkleTreeResult {
            pub result: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SyncMerkleTreeVariables {
            pub id: String,
        }
    }

    pub mod task_attachments {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Task {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskResult {
            pub result: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskVariables {
            pub id: String,
        }
    }

    pub mod task_git_credentials {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskGitCredentials {
            pub credentials: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskGitCredentialsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskGitCredentialsResult {
            pub result: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskGitCredentialsVariables {
            pub id: String,
        }
    }

    pub mod get_available_models {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAvailableModels {
            pub models: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAvailableModelsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FreeAvailableModels {
            pub models: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FreeAvailableModelsVariables {
            pub id: String,
        }
    }

    pub mod codebase_context_config {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CodebaseContextConfig {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CodebaseContextConfigVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CodebaseContextConfigQuery {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CodebaseContextConfigResult {
            pub id: String,
        }
    }

    pub mod api_keys {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ApiKeys {
            pub keys: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ApiKeysVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ApiKeyProperties {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ApiKeyPropertiesResult {
            pub keys: Vec<String>,
        }
    }

    pub mod get_conversation_usage {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetConversationUsage {
            pub usage: u64,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetConversationUsageVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ConversationUsage {
            pub usage: u64,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_user {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetUser {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetUserVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserResult {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserOutput {
            pub id: String,
        }
    }

    pub mod free_available_models {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FreeAvailableModels {
            pub models: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FreeAvailableModelsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FreeAvailableModelsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct FreeAvailableModelsResult {
            pub models: Vec<String>,
        }
    }

    pub mod get_user_settings {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetUserSettings {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetUserSettingsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserSettings {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_blocks_for_user {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetBlocksForUser {
            pub blocks: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetBlocksForUserVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Block {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_integrations_using_environment {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetIntegrationsUsingEnvironment {
            pub integrations: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetIntegrationsUsingEnvironmentVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetIntegrationsUsingEnvironmentInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetIntegrationsUsingEnvironmentOutput {
            pub integrations: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetIntegrationsUsingEnvironmentResult {
            pub integrations: Vec<String>,
        }
    }

    pub mod get_oauth_connect_tx_status {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetOAuthConnectTxStatus {
            pub status: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetOAuthConnectTxStatusVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetOAuthConnectTxStatusInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetOAuthConnectTxStatusResult {
            pub status: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum OauthConnectTxStatus {
            Pending,
            Complete,
        }
    }

    pub mod get_simple_integrations {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetSimpleIntegrations {
            pub integrations: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetSimpleIntegrationsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIntegrations {
            pub integrations: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIntegrationsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIntegrationsOutput {
            pub integrations: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIntegrationsResult {
            pub integrations: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SimpleIntegrationsVariables {
            pub id: String,
        }
    }

    pub mod get_cloud_environments {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudEnvironments {
            pub environments: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudEnvironmentsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudEnvironmentsQuery {
            pub environments: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudEnvironmentsQueryVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudEnvironmentsResult {
            pub environments: Vec<String>,
        }
    }

    pub mod get_cloud_object {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudObject {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetCloudObjectVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CloudObjectInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct CloudObjectResult {
            pub id: String,
        }
    }

    pub mod get_updated_cloud_objects {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetUpdatedCloudObjects {
            pub objects: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetUpdatedCloudObjectsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdatedCloudObjectsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UpdatedCloudObjectsResult {
            pub objects: Vec<String>,
        }
    }

    pub mod get_referral_info {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetReferralInfo {
            pub info: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetReferralInfoVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_discoverable_teams {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetDiscoverableTeams {
            pub teams: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetDiscoverableTeamsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_workspaces_metadata_for_user {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetWorkspacesMetadataForUser {
            pub workspaces: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetWorkspacesMetadataForUserVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct PricingInfoResult {
            pub info: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct User {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod get_ai_overages_for_workspace {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAiOveragesForWorkspace {
            pub overages: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAiOveragesForWorkspaceVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod suggest_cloud_environment_image {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SuggestCloudEnvironmentImage {
            pub image: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SuggestCloudEnvironmentImageVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RepoInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SuggestCloudEnvironmentImageInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct SuggestCloudEnvironmentImageResult {
            pub image: String,
        }
    }

    pub mod list_harness_auth_secrets {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListHarnessAuthSecrets {
            pub secrets: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListHarnessAuthSecretsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListHarnessAuthSecretsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListHarnessAuthSecretsResult {
            pub secrets: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct HarnessAuthSecretsResult {
            pub secrets: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AgentHarnessInput {
            pub id: String,
        }
    }

    pub mod list_managed_secrets {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListManagedSecrets {
            pub secrets: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListManagedSecretsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretsResult {
            pub secrets: Vec<String>,
        }
    }

    pub mod managed_secret_config {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretConfig {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretConfigVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetManagedSecretConfig {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetManagedSecretConfigVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretConfigInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretConfigResult {
            pub config: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum UserResult {
            UserOutput(String),
            UserFacingError(String),
            Unknown,
        }
    }

    pub mod task_secrets {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskSecrets {
            pub secrets: Vec<String>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskSecretsVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ManagedSecretValue {
            pub value: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskSecretsInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct TaskSecretsResult {
            pub secrets: Vec<String>,
        }
    }

    pub mod user_github_info {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserGithubInfo {
            pub info: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserGithubInfoVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GithubAuthRequiredOutput {
            pub required: bool,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserGithubInfoResult {
            pub info: String,
        }
    }

    pub mod user_repo_auth_status {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserRepoAuthStatus {
            pub status: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserRepoAuthStatusVariables {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct RepoInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserRepoAuthStatusInput {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserRepoAuthStatusOutput {
            pub status: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct UserRepoAuthStatusResult {
            pub status: String,
        }
    }

    pub mod list_ai_conversations {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AIConversation {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListAIConversations {
            pub conversations: Vec<AIConversation>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListAIConversationsInput {
            pub conversation_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListAIConversationsVariables {
            pub input: ListAIConversationsInput,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum ListAIConversationsResult {
            Success(ListAIConversations),
            Error(String),
        }

        impl ListAIConversations {
            pub fn build(_vars: ListAIConversationsVariables) -> Self {
                Self { conversations: vec![] }
            }
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListAIConversationMetadata {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AIConversationMetadata {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct ListAIConversationMetadataVariables {
            pub conversation_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum ListAIConversationMetadataResult {
            Success(ListAIConversationMetadata),
            Error(String),
        }
    }

    pub mod get_ai_conversation_format {
        use super::*;

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct AIConversationFormat {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAIConversationFormat {
            pub format: Option<AIConversationFormat>,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAIConversationFormatInput {
            pub conversation_id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetAIConversationFormatVariables {
            pub input: GetAIConversationFormatInput,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum GetAIConversationFormatResult {
            Success(GetAIConversationFormat),
            Error(String),
        }

        impl GetAIConversationFormat {
            pub fn build(_vars: GetAIConversationFormatVariables) -> Self {
                Self { format: None }
            }
        }
    }
}

pub mod ai {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AIConversation {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AIConversationFormat {
        Default,
        Compact,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AgentHarness {
        Oz,
        ClaudeCode,
        Gemini,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AgentTaskState {
        Pending,
        Running,
        Complete,
        Failed,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum PlatformErrorCode {
        Unknown,
        Timeout,
        RateLimited,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SerializedBlockFormat {
        JsonV1,
        Unknown,
    }
}

pub mod mcp_gallery_template {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct McpGalleryTemplate {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MCPGalleryTemplate {
        pub id: String,
    }
}

pub mod managed_secrets {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ManagedSecret {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ManagedSecretType {
        EnvironmentVariable,
        File,
    }
}

pub mod notebook {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Notebook {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateNotebookEditAccessInput {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UpdateNotebookEditAccessResult {
        pub success: bool,
    }
}

pub mod subscriptions {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Subscription {
        pub id: String,
    }

    pub mod get_warp_drive_updates {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct GetWarpDriveUpdates {
            pub id: String,
        }

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum WarpDriveUpdate {
            None,
        }
    }

    pub fn get_warp_drive_updates() -> Option<get_warp_drive_updates::GetWarpDriveUpdates> { None }
    pub fn start_graphql_streaming_operation() -> Option<String> { None }
}

pub mod user {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserProfile {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DiscoverableTeamData {
        pub id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PublicUserProfile {
        pub id: String,
    }
}

pub mod workflow {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Workflow {
        pub id: String,
    }
}
