//! Pricing information model.
//!
//! Note: Cloud-specific logic has been removed. Only type definitions remain.

use warp_graphql::billing::{
    AddonCreditsOption, OveragesPricing, PlanPricing, PricingInfo, StripeSubscriptionPlan,
};
use warpui::{Entity, ModelContext, SingletonEntity};

/// A global model for maintaining pricing information from the server.
#[derive(Debug)]
pub struct PricingInfoModel {
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
        self.pricing_info.as_ref().map(|info| &info.overages)
    }

    #[allow(dead_code)]
    pub fn plan_pricing(&self, plan: &StripeSubscriptionPlan) -> Option<&PlanPricing> {
        self.pricing_info
            .as_ref()?
            .plans
            .iter()
            .find(|p| &p.plan == plan)
    }

    #[allow(dead_code)]
    pub fn plans(&self) -> &[PlanPricing] {
        self.pricing_info
            .as_ref()
            .map(|info| info.plans.as_slice())
            .unwrap_or(&[])
    }

    #[allow(dead_code)]
    pub fn overage_cost_dollars(&self) -> Option<f64> {
        self.overage_pricing()
            .map(|overages| overages.price_per_request_usd_cents as f64 / 100.0)
    }

    #[allow(dead_code)]
    pub fn monthly_plan_cost_dollars(&self, plan: &StripeSubscriptionPlan) -> Option<f64> {
        self.plan_pricing(plan)
            .map(|pricing| pricing.monthly_plan_price_per_month_usd_cents as f64 / 100.0)
    }

    #[allow(dead_code)]
    pub fn addon_credits_options(&self) -> Option<&[AddonCreditsOption]> {
        self.pricing_info
            .as_ref()
            .map(|info| info.addon_credits_options.as_slice())
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
