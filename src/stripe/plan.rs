use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PlanData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aggregate_usage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_scheme: Option<PrimField<String>>,
    currency: PrimField<String>,
    interval: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_usage_divide_by: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_usage_round: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trial_period_days: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<Vec<PlanTiersEl>>,
    dynamic: PlanDynamic,
}

struct Plan_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PlanData>,
}

#[derive(Clone)]
pub struct Plan(Rc<Plan_>);

impl Plan {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderStripe) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `aggregate_usage`.\nSpecifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn set_aggregate_usage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().aggregate_usage = Some(v.into());
        self
    }

    #[doc= "Set the field `amount`.\nA positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis."]
    pub fn set_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().amount = Some(v.into());
        self
    }

    #[doc= "Set the field `amount_decimal`.\nSame as `amount`, but accepts a decimal value with at most 12 decimal places. Only one of `amount` and `amount_decimal` can be set."]
    pub fn set_amount_decimal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().amount_decimal = Some(v.into());
        self
    }

    #[doc= "Set the field `billing_scheme`.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn set_billing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_scheme = Some(v.into());
        self
    }

    #[doc= "Set the field `interval_count`.\nThe number of intervals between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn set_interval_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interval_count = Some(v.into());
        self
    }

    #[doc= "Set the field `nickname`.\nA brief description of the plan, hidden from customers."]
    pub fn set_nickname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nickname = Some(v.into());
        self
    }

    #[doc= "Set the field `product`.\nThe product whose pricing this plan determines."]
    pub fn set_product(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product = Some(v.into());
        self
    }

    #[doc= "Set the field `tiers_mode`.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn set_tiers_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tiers_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `transform_usage_divide_by`.\nDivide usage by this number."]
    pub fn set_transform_usage_divide_by(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transform_usage_divide_by = Some(v.into());
        self
    }

    #[doc= "Set the field `transform_usage_round`.\nAfter division, either round the result `up` or `down`."]
    pub fn set_transform_usage_round(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transform_usage_round = Some(v.into());
        self
    }

    #[doc= "Set the field `trial_period_days`.\nDefault number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan)."]
    pub fn set_trial_period_days(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().trial_period_days = Some(v.into());
        self
    }

    #[doc= "Set the field `usage_type`.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn set_usage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().usage_type = Some(v.into());
        self
    }

    #[doc= "Set the field `tiers`.\n"]
    pub fn set_tiers(self, v: impl Into<BlockAssignable<PlanTiersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiers = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn aggregate_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aggregate_usage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis."]
    pub fn amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `amount_decimal` after provisioning.\nSame as `amount`, but accepts a decimal value with at most 12 decimal places. Only one of `amount` and `amount_decimal` can be set."]
    pub fn amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn billing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_scheme", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nAn identifier randomly generated by Stripe. Used to identify this plan when subscribing a customer. You can optionally override this ID, but the ID must be unique across all plans in your Stripe account. You can, however, use the same plan ID in both live and test modes."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nSpecifies billing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval_count` after provisioning.\nThe number of intervals between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn interval_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the plan, hidden from customers."]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product` after provisioning.\nThe product whose pricing this plan determines."]
    pub fn product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_usage_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_usage_divide_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_divide_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_usage_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_usage_round(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_round", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `trial_period_days` after provisioning.\nDefault number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan)."]
    pub fn trial_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.trial_period_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn usage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<PlanTiersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

impl Referable for Plan {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Plan { }

impl ToListMappable for Plan {
    type O = ListRef<PlanRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Plan_ {
    fn extract_resource_type(&self) -> String {
        "stripe_plan".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPlan {
    pub tf_id: String,
    #[doc= "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub currency: PrimField<String>,
    #[doc= "Specifies billing frequency. Either `day`, `week`, `month` or `year`."]
    pub interval: PrimField<String>,
}

impl BuildPlan {
    pub fn build(self, stack: &mut Stack) -> Plan {
        let out = Plan(Rc::new(Plan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PlanData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                aggregate_usage: core::default::Default::default(),
                amount: core::default::Default::default(),
                amount_decimal: core::default::Default::default(),
                billing_scheme: core::default::Default::default(),
                currency: self.currency,
                interval: self.interval,
                interval_count: core::default::Default::default(),
                nickname: core::default::Default::default(),
                product: core::default::Default::default(),
                tiers_mode: core::default::Default::default(),
                transform_usage_divide_by: core::default::Default::default(),
                transform_usage_round: core::default::Default::default(),
                trial_period_days: core::default::Default::default(),
                usage_type: core::default::Default::default(),
                tiers: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PlanRef {
    shared: StackShared,
    base: String,
}

impl Ref for PlanRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PlanRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn aggregate_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aggregate_usage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis."]
    pub fn amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `amount_decimal` after provisioning.\nSame as `amount`, but accepts a decimal value with at most 12 decimal places. Only one of `amount` and `amount_decimal` can be set."]
    pub fn amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn billing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_scheme", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nAn identifier randomly generated by Stripe. Used to identify this plan when subscribing a customer. You can optionally override this ID, but the ID must be unique across all plans in your Stripe account. You can, however, use the same plan ID in both live and test modes."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval` after provisioning.\nSpecifies billing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `interval_count` after provisioning.\nThe number of intervals between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn interval_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the plan, hidden from customers."]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product` after provisioning.\nThe product whose pricing this plan determines."]
    pub fn product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_usage_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_usage_divide_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_divide_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_usage_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_usage_round(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_round", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `trial_period_days` after provisioning.\nDefault number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan)."]
    pub fn trial_period_days(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.trial_period_days", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn usage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.usage_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<PlanTiersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PlanTiersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flat_amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<PrimField<String>>,
    up_to: PrimField<String>,
}

impl PlanTiersEl {
    #[doc= "Set the field `flat_amount`.\nPrice for the entire tier."]
    pub fn set_flat_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.flat_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `flat_amount_decimal`.\nSame as `flat_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn set_flat_amount_decimal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.flat_amount_decimal = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount`.\nPer unit price for units relevant to the tier."]
    pub fn set_unit_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unit_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount_decimal`.\nSame as `unit_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn set_unit_amount_decimal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit_amount_decimal = Some(v.into());
        self
    }
}

impl ToListMappable for PlanTiersEl {
    type O = BlockAssignable<PlanTiersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPlanTiersEl {
    #[doc= "Up to and including to this quantity will be contained in the tier."]
    pub up_to: PrimField<String>,
}

impl BuildPlanTiersEl {
    pub fn build(self) -> PlanTiersEl {
        PlanTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: self.up_to,
        }
    }
}

pub struct PlanTiersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PlanTiersElRef {
    fn new(shared: StackShared, base: String) -> PlanTiersElRef {
        PlanTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PlanTiersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `flat_amount` after provisioning.\nPrice for the entire tier."]
    pub fn flat_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.flat_amount", self.base))
    }

    #[doc= "Get a reference to the value of field `flat_amount_decimal` after provisioning.\nSame as `flat_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn flat_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flat_amount_decimal", self.base))
    }

    #[doc= "Get a reference to the value of field `unit_amount` after provisioning.\nPer unit price for units relevant to the tier."]
    pub fn unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc= "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc= "Get a reference to the value of field `up_to` after provisioning.\nUp to and including to this quantity will be contained in the tier."]
    pub fn up_to(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.up_to", self.base))
    }
}

#[derive(Serialize, Default)]
struct PlanDynamic {
    tiers: Option<DynamicBlock<PlanTiersEl>>,
}
