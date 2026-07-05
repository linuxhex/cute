//! Pricing information model.
//!
//! Note: Cloud-specific logic has been removed. Simplified stub.

use cute_graphql::billing::{
    AddonCreditsOption, OveragesPricing, PlanPricing, PricingInfo, StripeSubscriptionPlan,
};
use cuteui::{Entity, ModelContext, SingletonEntity};

/// A global model for maintaining pricing information from the server.
/// Simplified: local version has no pricing.
#[derive(Debug)]
pub struct PricingInfoModel {
    #[allow(dead_code)]
    pricing_info: Option<PricingInfo>,
}

impl PricingInfoModel {
    pub fn new() -> Self {
        Self { pricing_info: None }
    }

    #[allow(dead_code)]
    pub fn update_pricing_info(&mut self, pricing_info: PricingInfo, ctx: &mut ModelContext<Self>) {
        self.pricing_info = Some(pricing_info);
        ctx.emit(PricingInfoModelEvent::PricingInfoUpdated);
    }

    #[allow(dead_code)]
    fn overage_pricing(&self) -> Option<&OveragesPricing> {
        None
    }

    #[allow(dead_code)]
    pub fn plan_pricing(&self, _plan: &StripeSubscriptionPlan) -> Option<&PlanPricing> {
        None
    }

    #[allow(dead_code)]
    pub fn plans(&self) -> &[PlanPricing] {
        &[]
    }

    #[allow(dead_code)]
    pub fn overage_cost_dollars(&self) -> Option<f64> {
        None
    }

    #[allow(dead_code)]
    pub fn monthly_plan_cost_dollars(&self, _plan: &StripeSubscriptionPlan) -> Option<f64> {
        None
    }

    #[allow(dead_code)]
    pub fn addon_credits_options(&self) -> Option<&[AddonCreditsOption]> {
        None
    }
}

impl Default for PricingInfoModel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PricingInfoModelEvent {
    PricingInfoUpdated,
}

impl Entity for PricingInfoModel {
    type Event = PricingInfoModelEvent;
}

impl SingletonEntity for PricingInfoModel {}
