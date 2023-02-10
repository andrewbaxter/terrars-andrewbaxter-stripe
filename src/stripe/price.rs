use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PriceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_scheme: Option<PrimField<String>>,
    currency: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_preset: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lookup_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_active: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_statement_descriptor: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_tax_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_data_unit_label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_aggregate_usage: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_interval_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring_usage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_lookup_key: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_quantity_divide_by: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transform_quantity_round: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_options: Option<Vec<PriceCurrencyOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<Vec<PriceTiersEl>>,
    dynamic: PriceDynamic,
}

struct Price_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PriceData>,
}

#[derive(Clone)]
pub struct Price(Rc<Price_>);

impl Price {
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

    #[doc= "Set the field `billing_scheme`.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn set_billing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().billing_scheme = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_enabled`.\n"]
    pub fn set_custom_unit_amount_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_maximum`.\nThe maximum unit amount the customer can specify for this item."]
    pub fn set_custom_unit_amount_maximum(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_maximum = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_minimum`.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn set_custom_unit_amount_minimum(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_minimum = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_preset`.\nThe starting unit amount which can be updated by the customer."]
    pub fn set_custom_unit_amount_preset(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_preset = Some(v.into());
        self
    }

    #[doc= "Set the field `lookup_key`.\nA lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters."]
    pub fn set_lookup_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().lookup_key = Some(v.into());
        self
    }

    #[doc= "Set the field `nickname`.\nA brief description of the price, hidden from customers."]
    pub fn set_nickname(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().nickname = Some(v.into());
        self
    }

    #[doc= "Set the field `product`.\nThe ID of the product that this price will belong to."]
    pub fn set_product(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_active`.\n"]
    pub fn set_product_data_active(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().product_data_active = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_id`.\n"]
    pub fn set_product_data_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_data_id = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_metadata`.\n"]
    pub fn set_product_data_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().product_data_metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_name`.\n"]
    pub fn set_product_data_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_data_name = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_statement_descriptor`.\n"]
    pub fn set_product_data_statement_descriptor(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_data_statement_descriptor = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_tax_code`.\n"]
    pub fn set_product_data_tax_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_data_tax_code = Some(v.into());
        self
    }

    #[doc= "Set the field `product_data_unit_label`.\n"]
    pub fn set_product_data_unit_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().product_data_unit_label = Some(v.into());
        self
    }

    #[doc= "Set the field `recurring_aggregate_usage`.\nSpecifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn set_recurring_aggregate_usage(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().recurring_aggregate_usage = Some(v.into());
        self
    }

    #[doc= "Set the field `recurring_interval`.\nThe frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`."]
    pub fn set_recurring_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().recurring_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `recurring_interval_count`.\nThe number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months."]
    pub fn set_recurring_interval_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().recurring_interval_count = Some(v.into());
        self
    }

    #[doc= "Set the field `recurring_usage_type`.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn set_recurring_usage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().recurring_usage_type = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_behavior`.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn set_tax_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tax_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `tiers_mode`.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn set_tiers_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tiers_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `transfer_lookup_key`.\nIf set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub fn set_transfer_lookup_key(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transfer_lookup_key = Some(v.into());
        self
    }

    #[doc= "Set the field `transform_quantity_divide_by`.\nDivide usage by this number."]
    pub fn set_transform_quantity_divide_by(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transform_quantity_divide_by = Some(v.into());
        self
    }

    #[doc= "Set the field `transform_quantity_round`.\nAfter division, either round the result `up` or `down`."]
    pub fn set_transform_quantity_round(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().transform_quantity_round = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount`.\nA positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge. One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`."]
    pub fn set_unit_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unit_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount_decimal`.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn set_unit_amount_decimal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().unit_amount_decimal = Some(v.into());
        self
    }

    #[doc= "Set the field `currency_options`.\n"]
    pub fn set_currency_options(self, v: impl Into<BlockAssignable<PriceCurrencyOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().currency_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.currency_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `tiers`.\n"]
    pub fn set_tiers(self, v: impl Into<BlockAssignable<PriceTiersEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
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

    #[doc= "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\nThe maximum unit amount the customer can specify for this item."]
    pub fn custom_unit_amount_maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn custom_unit_amount_minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\nThe starting unit amount which can be updated by the customer."]
    pub fn custom_unit_amount_preset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lookup_key` after provisioning.\nA lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters."]
    pub fn lookup_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lookup_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the price, hidden from customers."]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product` after provisioning.\nThe ID of the product that this price will belong to."]
    pub fn product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_active` after provisioning.\n"]
    pub fn product_data_active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_id` after provisioning.\n"]
    pub fn product_data_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_metadata` after provisioning.\n"]
    pub fn product_data_metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.product_data_metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_name` after provisioning.\n"]
    pub fn product_data_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_statement_descriptor` after provisioning.\n"]
    pub fn product_data_statement_descriptor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_statement_descriptor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_tax_code` after provisioning.\n"]
    pub fn product_data_tax_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_tax_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_unit_label` after provisioning.\n"]
    pub fn product_data_unit_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_unit_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn recurring_aggregate_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_aggregate_usage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_interval` after provisioning.\nThe frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`."]
    pub fn recurring_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_interval_count` after provisioning.\nThe number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months."]
    pub fn recurring_interval_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn recurring_usage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_usage_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transfer_lookup_key` after provisioning.\nIf set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub fn transfer_lookup_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.transfer_lookup_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_quantity_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_quantity_divide_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_divide_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_quantity_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_quantity_round(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_round", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nOne of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge. One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`."]
    pub fn unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> ListRef<PriceCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<PriceTiersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

impl Referable for Price {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Price { }

impl ToListMappable for Price {
    type O = ListRef<PriceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Price_ {
    fn extract_resource_type(&self) -> String {
        "stripe_price".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrice {
    pub tf_id: String,
    #[doc= "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub currency: PrimField<String>,
}

impl BuildPrice {
    pub fn build(self, stack: &mut Stack) -> Price {
        let out = Price(Rc::new(Price_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PriceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_scheme: core::default::Default::default(),
                currency: self.currency,
                custom_unit_amount_enabled: core::default::Default::default(),
                custom_unit_amount_maximum: core::default::Default::default(),
                custom_unit_amount_minimum: core::default::Default::default(),
                custom_unit_amount_preset: core::default::Default::default(),
                lookup_key: core::default::Default::default(),
                nickname: core::default::Default::default(),
                product: core::default::Default::default(),
                product_data_active: core::default::Default::default(),
                product_data_id: core::default::Default::default(),
                product_data_metadata: core::default::Default::default(),
                product_data_name: core::default::Default::default(),
                product_data_statement_descriptor: core::default::Default::default(),
                product_data_tax_code: core::default::Default::default(),
                product_data_unit_label: core::default::Default::default(),
                recurring_aggregate_usage: core::default::Default::default(),
                recurring_interval: core::default::Default::default(),
                recurring_interval_count: core::default::Default::default(),
                recurring_usage_type: core::default::Default::default(),
                tax_behavior: core::default::Default::default(),
                tiers_mode: core::default::Default::default(),
                transfer_lookup_key: core::default::Default::default(),
                transform_quantity_divide_by: core::default::Default::default(),
                transform_quantity_round: core::default::Default::default(),
                unit_amount: core::default::Default::default(),
                unit_amount_decimal: core::default::Default::default(),
                currency_options: core::default::Default::default(),
                tiers: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PriceRef {
    shared: StackShared,
    base: String,
}

impl Ref for PriceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PriceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
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

    #[doc= "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\nThe maximum unit amount the customer can specify for this item."]
    pub fn custom_unit_amount_maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn custom_unit_amount_minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\nThe starting unit amount which can be updated by the customer."]
    pub fn custom_unit_amount_preset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lookup_key` after provisioning.\nA lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters."]
    pub fn lookup_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.lookup_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the price, hidden from customers."]
    pub fn nickname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product` after provisioning.\nThe ID of the product that this price will belong to."]
    pub fn product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_active` after provisioning.\n"]
    pub fn product_data_active(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_active", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_id` after provisioning.\n"]
    pub fn product_data_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_metadata` after provisioning.\n"]
    pub fn product_data_metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.product_data_metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_name` after provisioning.\n"]
    pub fn product_data_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_statement_descriptor` after provisioning.\n"]
    pub fn product_data_statement_descriptor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_statement_descriptor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_tax_code` after provisioning.\n"]
    pub fn product_data_tax_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_tax_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `product_data_unit_label` after provisioning.\n"]
    pub fn product_data_unit_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product_data_unit_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn recurring_aggregate_usage(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_aggregate_usage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_interval` after provisioning.\nThe frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`."]
    pub fn recurring_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_interval_count` after provisioning.\nThe number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months."]
    pub fn recurring_interval_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recurring_usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn recurring_usage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.recurring_usage_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transfer_lookup_key` after provisioning.\nIf set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub fn transfer_lookup_key(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.transfer_lookup_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_quantity_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_quantity_divide_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_divide_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `transform_quantity_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_quantity_round(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_round", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nOne of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge. One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`."]
    pub fn unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> ListRef<PriceCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<PriceTiersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PriceCurrencyOptionsElTiersEl {
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

impl PriceCurrencyOptionsElTiersEl {
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

impl ToListMappable for PriceCurrencyOptionsElTiersEl {
    type O = BlockAssignable<PriceCurrencyOptionsElTiersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPriceCurrencyOptionsElTiersEl {
    #[doc= "Up to and including to this quantity will be contained in the tier."]
    pub up_to: PrimField<String>,
}

impl BuildPriceCurrencyOptionsElTiersEl {
    pub fn build(self) -> PriceCurrencyOptionsElTiersEl {
        PriceCurrencyOptionsElTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: self.up_to,
        }
    }
}

pub struct PriceCurrencyOptionsElTiersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PriceCurrencyOptionsElTiersElRef {
    fn new(shared: StackShared, base: String) -> PriceCurrencyOptionsElTiersElRef {
        PriceCurrencyOptionsElTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PriceCurrencyOptionsElTiersElRef {
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
struct PriceCurrencyOptionsElDynamic {
    tiers: Option<DynamicBlock<PriceCurrencyOptionsElTiersEl>>,
}

#[derive(Serialize)]
pub struct PriceCurrencyOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_maximum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_minimum: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_unit_amount_preset: Option<PrimField<f64>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tiers: Option<Vec<PriceCurrencyOptionsElTiersEl>>,
    dynamic: PriceCurrencyOptionsElDynamic,
}

impl PriceCurrencyOptionsEl {
    #[doc= "Set the field `custom_unit_amount_enabled`.\n"]
    pub fn set_custom_unit_amount_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.custom_unit_amount_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_maximum`.\nThe maximum unit amount the customer can specify for this item."]
    pub fn set_custom_unit_amount_maximum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_unit_amount_maximum = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_minimum`.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn set_custom_unit_amount_minimum(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_unit_amount_minimum = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_unit_amount_preset`.\nThe starting unit amount which can be updated by the customer."]
    pub fn set_custom_unit_amount_preset(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.custom_unit_amount_preset = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_behavior`.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn set_tax_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tax_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount`.\nThe unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`."]
    pub fn set_unit_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unit_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount_decimal`.\nThe unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`."]
    pub fn set_unit_amount_decimal(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit_amount_decimal = Some(v.into());
        self
    }

    #[doc= "Set the field `tiers`.\n"]
    pub fn set_tiers(mut self, v: impl Into<BlockAssignable<PriceCurrencyOptionsElTiersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.tiers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.tiers = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PriceCurrencyOptionsEl {
    type O = BlockAssignable<PriceCurrencyOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPriceCurrencyOptionsEl {
    #[doc= "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: PrimField<String>,
}

impl BuildPriceCurrencyOptionsEl {
    pub fn build(self) -> PriceCurrencyOptionsEl {
        PriceCurrencyOptionsEl {
            custom_unit_amount_enabled: core::default::Default::default(),
            custom_unit_amount_maximum: core::default::Default::default(),
            custom_unit_amount_minimum: core::default::Default::default(),
            custom_unit_amount_preset: core::default::Default::default(),
            key: self.key,
            tax_behavior: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            tiers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct PriceCurrencyOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PriceCurrencyOptionsElRef {
    fn new(shared: StackShared, base: String) -> PriceCurrencyOptionsElRef {
        PriceCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PriceCurrencyOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\nThe maximum unit amount the customer can specify for this item."]
    pub fn custom_unit_amount_maximum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn custom_unit_amount_minimum(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.base))
    }

    #[doc= "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\nThe starting unit amount which can be updated by the customer."]
    pub fn custom_unit_amount_preset(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.base))
    }

    #[doc= "Get a reference to the value of field `unit_amount` after provisioning.\nThe unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`."]
    pub fn unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc= "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nThe unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`."]
    pub fn unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc= "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> ListRef<PriceCurrencyOptionsElTiersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tiers", self.base))
    }
}

#[derive(Serialize)]
pub struct PriceTiersEl {
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

impl PriceTiersEl {
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

impl ToListMappable for PriceTiersEl {
    type O = BlockAssignable<PriceTiersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPriceTiersEl {
    #[doc= "Up to and including to this quantity will be contained in the tier."]
    pub up_to: PrimField<String>,
}

impl BuildPriceTiersEl {
    pub fn build(self) -> PriceTiersEl {
        PriceTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: self.up_to,
        }
    }
}

pub struct PriceTiersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PriceTiersElRef {
    fn new(shared: StackShared, base: String) -> PriceTiersElRef {
        PriceTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PriceTiersElRef {
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
struct PriceDynamic {
    currency_options: Option<DynamicBlock<PriceCurrencyOptionsEl>>,
    tiers: Option<DynamicBlock<PriceTiersEl>>,
}
