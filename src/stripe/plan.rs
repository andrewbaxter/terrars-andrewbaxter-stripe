use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PlanData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregate_usage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    currency: terrars::PrimField<std::string::String>,
    interval: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nickname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_usage_divide_by: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_usage_round: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers: std::option::Option<std::vec::Vec<PlanTiersEl>>,
    dynamic: PlanDynamic,
}

struct Plan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PlanData>,
}

#[derive(Clone)]
pub struct Plan(std::rc::Rc<Plan_>);

impl Plan {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderStripe) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `aggregate_usage`.\nSpecifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn set_aggregate_usage(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aggregate_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `amount`.\nA positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis."]
    pub fn set_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `amount_decimal`.\nSame as `amount`, but accepts a decimal value with at most 12 decimal places. Only one of `amount` and `amount_decimal` can be set."]
    pub fn set_amount_decimal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `billing_scheme`.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn set_billing_scheme(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interval_count`.\nThe number of intervals between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn set_interval_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nickname`.\nA brief description of the plan, hidden from customers."]
    pub fn set_nickname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nickname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product`.\nThe product whose pricing this plan determines."]
    pub fn set_product(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tiers_mode`.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn set_tiers_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tiers_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transform_usage_divide_by`.\nDivide usage by this number."]
    pub fn set_transform_usage_divide_by(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transform_usage_divide_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transform_usage_round`.\nAfter division, either round the result `up` or `down`."]
    pub fn set_transform_usage_round(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transform_usage_round = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trial_period_days`.\nDefault number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan)."]
    pub fn set_trial_period_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().trial_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `usage_type`.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn set_usage_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().usage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tiers`.\n"]
    pub fn set_tiers(self, v: impl std::convert::Into<terrars::BlockAssignable<PlanTiersEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tiers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tiers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn aggregate_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregate_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis."]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_decimal` after provisioning.\nSame as `amount`, but accepts a decimal value with at most 12 decimal places. Only one of `amount` and `amount_decimal` can be set."]
    pub fn amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_decimal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn billing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_scheme", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAn identifier randomly generated by Stripe. Used to identify this plan when subscribing a customer. You can optionally override this ID, but the ID must be unique across all plans in your Stripe account. You can, however, use the same plan ID in both live and test modes."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nSpecifies billing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval_count` after provisioning.\nThe number of intervals between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the plan, hidden from customers."]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `product` after provisioning.\nThe product whose pricing this plan determines."]
    pub fn product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_usage_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_usage_divide_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_divide_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_usage_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_usage_round(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_round", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_period_days` after provisioning.\nDefault number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan)."]
    pub fn trial_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn usage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<PlanTiersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

impl terrars::Referable for Plan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Plan { }

impl terrars::ToListMappable for Plan {
    type O = terrars::ListRef<PlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Plan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPlan {
    pub tf_id: std::string::String,
    #[doc =
        "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub currency: terrars::PrimField<std::string::String>,
    #[doc = "Specifies billing frequency. Either `day`, `week`, `month` or `year`."]
    pub interval: terrars::PrimField<std::string::String>,
}

impl BuildPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> Plan {
        let out = Plan(std::rc::Rc::new(Plan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PlanData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for plans of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn aggregate_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregate_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free plan) representing how much to charge on a recurring basis."]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_decimal` after provisioning.\nSame as `amount`, but accepts a decimal value with at most 12 decimal places. Only one of `amount` and `amount_decimal` can be set."]
    pub fn amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_decimal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `amount`) will be charged per unit in `quantity` (for plans with `usage_type=licensed`), or per unit of total usage (for plans with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn billing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_scheme", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAn identifier randomly generated by Stripe. Used to identify this plan when subscribing a customer. You can optionally override this ID, but the ID must be unique across all plans in your Stripe account. You can, however, use the same plan ID in both live and test modes."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nSpecifies billing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `interval_count` after provisioning.\nThe number of intervals between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the plan, hidden from customers."]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `product` after provisioning.\nThe product whose pricing this plan determines."]
    pub fn product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_usage_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_usage_divide_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_divide_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_usage_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_usage_round(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_usage_round", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_period_days` after provisioning.\nDefault number of trial days when subscribing a customer to this plan using [`trial_from_plan=true`](https://stripe.com/docs/api#create_subscription-trial_from_plan)."]
    pub fn trial_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn usage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<PlanTiersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PlanTiersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    up_to: terrars::PrimField<std::string::String>,
}

impl PlanTiersEl {
    #[doc = "Set the field `flat_amount`.\nPrice for the entire tier."]
    pub fn set_flat_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.flat_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `flat_amount_decimal`.\nSame as `flat_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn set_flat_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flat_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount`.\nPer unit price for units relevant to the tier."]
    pub fn set_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount_decimal`.\nSame as `unit_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn set_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PlanTiersEl {
    type O = terrars::BlockAssignable<PlanTiersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPlanTiersEl {
    #[doc = "Up to and including to this quantity will be contained in the tier."]
    pub up_to: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PlanTiersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PlanTiersElRef {
        PlanTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PlanTiersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flat_amount` after provisioning.\nPrice for the entire tier."]
    pub fn flat_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flat_amount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `flat_amount_decimal` after provisioning.\nSame as `flat_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn flat_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flat_amount_decimal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount` after provisioning.\nPer unit price for units relevant to the tier."]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but contains a decimal value with at most 12 decimal places."]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc =
        "Get a reference to the value of field `up_to` after provisioning.\nUp to and including to this quantity will be contained in the tier."]
    pub fn up_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.up_to", self.base))
    }
}

#[derive(Serialize, Default)]
struct PlanDynamic {
    tiers: std::option::Option<terrars::DynamicBlock<PlanTiersEl>>,
}
