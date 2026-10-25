#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PriceData_ {
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
    billing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    currency: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_preset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookup_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nickname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_statement_descriptor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_tax_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_data_unit_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_aggregate_usage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_interval_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_usage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_lookup_key: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_quantity_divide_by: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transform_quantity_round: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_options: std::option::Option<std::vec::Vec<Price_CurrencyOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers: std::option::Option<std::vec::Vec<Price_Tiers>>,
    dynamic: PriceDynamic,
}

struct Price_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PriceData_>,
}

#[derive(Clone)]
pub struct Price(std::rc::Rc<Price_>);

impl Price {
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
        "Set the field `billing_scheme`.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
    pub fn set_billing_scheme(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_unit_amount_enabled`.\n"]
    pub fn set_custom_unit_amount_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_unit_amount_maximum`.\nThe maximum unit amount the customer can specify for this item."]
    pub fn set_custom_unit_amount_maximum(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_unit_amount_minimum`.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn set_custom_unit_amount_minimum(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_unit_amount_preset`.\nThe starting unit amount which can be updated by the customer."]
    pub fn set_custom_unit_amount_preset(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().custom_unit_amount_preset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lookup_key`.\nA lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters."]
    pub fn set_lookup_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().lookup_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nickname`.\nA brief description of the price, hidden from customers."]
    pub fn set_nickname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nickname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product`.\nThe ID of the product that this price will belong to."]
    pub fn set_product(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_active`.\n"]
    pub fn set_product_data_active(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().product_data_active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_id`.\n"]
    pub fn set_product_data_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_data_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_metadata`.\n"]
    pub fn set_product_data_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().product_data_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_name`.\n"]
    pub fn set_product_data_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_data_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_statement_descriptor`.\n"]
    pub fn set_product_data_statement_descriptor(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().product_data_statement_descriptor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_tax_code`.\n"]
    pub fn set_product_data_tax_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_data_tax_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_data_unit_label`.\n"]
    pub fn set_product_data_unit_label(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().product_data_unit_label = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recurring_aggregate_usage`.\nSpecifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn set_recurring_aggregate_usage(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().recurring_aggregate_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recurring_interval`.\nThe frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`."]
    pub fn set_recurring_interval(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().recurring_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recurring_interval_count`.\nThe number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months."]
    pub fn set_recurring_interval_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().recurring_interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `recurring_usage_type`.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn set_recurring_usage_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().recurring_usage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_behavior`.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn set_tax_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tiers_mode`.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn set_tiers_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tiers_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_lookup_key`.\nIf set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub fn set_transfer_lookup_key(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().transfer_lookup_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transform_quantity_divide_by`.\nDivide usage by this number."]
    pub fn set_transform_quantity_divide_by(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transform_quantity_divide_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transform_quantity_round`.\nAfter division, either round the result `up` or `down`."]
    pub fn set_transform_quantity_round(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transform_quantity_round = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount`.\nA positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge. One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`."]
    pub fn set_unit_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount_decimal`.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn set_unit_amount_decimal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `currency_options`.\n"]
    pub fn set_currency_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Price_CurrencyOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().currency_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.currency_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tiers`.\n"]
    pub fn set_tiers(self, v: impl std::convert::Into<terrars::BlockAssignable<Price_Tiers>>) -> Self {
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
        "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
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

    #[doc = "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\nThe maximum unit amount the customer can specify for this item."]
    pub fn custom_unit_amount_maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn custom_unit_amount_minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\nThe starting unit amount which can be updated by the customer."]
    pub fn custom_unit_amount_preset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lookup_key` after provisioning.\nA lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters."]
    pub fn lookup_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookup_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the price, hidden from customers."]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `product` after provisioning.\nThe ID of the product that this price will belong to."]
    pub fn product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_active` after provisioning.\n"]
    pub fn product_data_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_active", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_id` after provisioning.\n"]
    pub fn product_data_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_metadata` after provisioning.\n"]
    pub fn product_data_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.product_data_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_name` after provisioning.\n"]
    pub fn product_data_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_statement_descriptor` after provisioning.\n"]
    pub fn product_data_statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_data_statement_descriptor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_data_tax_code` after provisioning.\n"]
    pub fn product_data_tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_tax_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_unit_label` after provisioning.\n"]
    pub fn product_data_unit_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_unit_label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn recurring_aggregate_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_aggregate_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_interval` after provisioning.\nThe frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`."]
    pub fn recurring_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_interval_count` after provisioning.\nThe number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months."]
    pub fn recurring_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn recurring_usage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_usage_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_lookup_key` after provisioning.\nIf set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub fn transfer_lookup_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_lookup_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_quantity_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_quantity_divide_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_divide_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_quantity_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_quantity_round(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_round", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOne of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge. One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`."]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> terrars::ListRef<Price_CurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<Price_TiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

impl terrars::Referable for Price {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Price { }

impl terrars::ToListMappable for Price {
    type O = terrars::ListRef<PriceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Price_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_price".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPrice {
    pub tf_id: std::string::String,
    #[doc =
        "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub currency: terrars::PrimField<std::string::String>,
}

impl BuildPrice {
    pub fn build(self, stack: &mut terrars::Stack) -> Price {
        let out = Price(std::rc::Rc::new(Price_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PriceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PriceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PriceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PriceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `billing_scheme` after provisioning.\nDescribes how to compute the price per period. Either `per_unit` or `tiered`. `per_unit` indicates that the fixed amount (specified in `unit_amount` or `unit_amount_decimal`) will be charged per unit in `quantity` (for prices with `usage_type=licensed`), or per unit of total usage (for prices with `usage_type=metered`). `tiered` indicates that the unit pricing will be computed using a tiering strategy as defined using the `tiers` and `tiers_mode` attributes."]
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

    #[doc = "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\nThe maximum unit amount the customer can specify for this item."]
    pub fn custom_unit_amount_maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn custom_unit_amount_minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\nThe starting unit amount which can be updated by the customer."]
    pub fn custom_unit_amount_preset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lookup_key` after provisioning.\nA lookup key used to retrieve prices dynamically from a static string. This may be up to 200 characters."]
    pub fn lookup_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookup_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nickname` after provisioning.\nA brief description of the price, hidden from customers."]
    pub fn nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nickname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `product` after provisioning.\nThe ID of the product that this price will belong to."]
    pub fn product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_active` after provisioning.\n"]
    pub fn product_data_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_active", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_id` after provisioning.\n"]
    pub fn product_data_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_metadata` after provisioning.\n"]
    pub fn product_data_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.product_data_metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_name` after provisioning.\n"]
    pub fn product_data_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_statement_descriptor` after provisioning.\n"]
    pub fn product_data_statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.product_data_statement_descriptor", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `product_data_tax_code` after provisioning.\n"]
    pub fn product_data_tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_tax_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_data_unit_label` after provisioning.\n"]
    pub fn product_data_unit_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_data_unit_label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_aggregate_usage` after provisioning.\nSpecifies a usage aggregation strategy for prices of `usage_type=metered`. Allowed values are `sum` for summing up all usage during a period, `last_during_period` for using the last usage record reported within a period, `last_ever` for using the last usage record ever (across period bounds) or `max` which uses the usage record with the maximum reported usage during a period. Defaults to `sum`."]
    pub fn recurring_aggregate_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_aggregate_usage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_interval` after provisioning.\nThe frequency at which a subscription is billed. One of `day`, `week`, `month` or `year`."]
    pub fn recurring_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_interval_count` after provisioning.\nThe number of intervals (specified in the `interval` attribute) between subscription billings. For example, `interval=month` and `interval_count=3` bills every 3 months."]
    pub fn recurring_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_interval_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_usage_type` after provisioning.\nConfigures how the quantity per period should be determined. Can be either `metered` or `licensed`. `licensed` automatically bills the `quantity` set when adding it to a subscription. `metered` aggregates the total usage based on usage records. Defaults to `licensed`."]
    pub fn recurring_usage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_usage_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tiers_mode` after provisioning.\nDefines if the tiering price should be `graduated` or `volume` based. In `volume`-based tiering, the maximum quantity within a period determines the per unit price, in `graduated` tiering pricing can successively change as the quantity grows."]
    pub fn tiers_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tiers_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_lookup_key` after provisioning.\nIf set to true, will atomically remove the lookup key from the existing price, and assign it to this price."]
    pub fn transfer_lookup_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_lookup_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_quantity_divide_by` after provisioning.\nDivide usage by this number."]
    pub fn transform_quantity_divide_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_divide_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transform_quantity_round` after provisioning.\nAfter division, either round the result `up` or `down`."]
    pub fn transform_quantity_round(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transform_quantity_round", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOne of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount` after provisioning.\nA positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge. One of `unit_amount` or `custom_unit_amount` is required, unless `billing_scheme=tiered`."]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> terrars::ListRef<Price_CurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<Price_TiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Price_CurrencyOptions_Tiers {
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

impl Price_CurrencyOptions_Tiers {
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

impl terrars::ToListMappable for Price_CurrencyOptions_Tiers {
    type O = terrars::BlockAssignable<Price_CurrencyOptions_Tiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrice_CurrencyOptions_Tiers {
    #[doc = "Up to and including to this quantity will be contained in the tier."]
    pub up_to: terrars::PrimField<std::string::String>,
}

impl BuildPrice_CurrencyOptions_Tiers {
    pub fn build(self) -> Price_CurrencyOptions_Tiers {
        Price_CurrencyOptions_Tiers {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: self.up_to,
        }
    }
}

#[allow(dead_code)]
pub struct Price_CurrencyOptions_TiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Price_CurrencyOptions_TiersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Price_CurrencyOptions_TiersRef {
        Price_CurrencyOptions_TiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Price_CurrencyOptions_TiersRef {
    #[allow(dead_code)]
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
struct Price_CurrencyOptionsDynamic {
    tiers: std::option::Option<terrars::DynamicBlock<Price_CurrencyOptions_Tiers>>,
}

#[derive(Serialize)]
pub struct Price_CurrencyOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_preset: std::option::Option<terrars::PrimField<f64>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers: std::option::Option<std::vec::Vec<Price_CurrencyOptions_Tiers>>,
    dynamic: Price_CurrencyOptionsDynamic,
}

impl Price_CurrencyOptions {
    #[doc = "Set the field `custom_unit_amount_enabled`.\n"]
    pub fn set_custom_unit_amount_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.custom_unit_amount_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_unit_amount_maximum`.\nThe maximum unit amount the customer can specify for this item."]
    pub fn set_custom_unit_amount_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_unit_amount_maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_unit_amount_minimum`.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn set_custom_unit_amount_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_unit_amount_minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_unit_amount_preset`.\nThe starting unit amount which can be updated by the customer."]
    pub fn set_custom_unit_amount_preset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_unit_amount_preset = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_behavior`.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn set_tax_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount`.\nThe unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`."]
    pub fn set_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount_decimal`.\nThe unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`."]
    pub fn set_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tiers`.\n"]
    pub fn set_tiers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Price_CurrencyOptions_Tiers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tiers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tiers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Price_CurrencyOptions {
    type O = terrars::BlockAssignable<Price_CurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrice_CurrencyOptions {
    #[doc = "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildPrice_CurrencyOptions {
    pub fn build(self) -> Price_CurrencyOptions {
        Price_CurrencyOptions {
            custom_unit_amount_enabled: core::default::Default::default(),
            custom_unit_amount_maximum: core::default::Default::default(),
            custom_unit_amount_minimum: core::default::Default::default(),
            custom_unit_amount_preset: core::default::Default::default(),
            key: self.key,
            tax_behavior: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            tiers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Price_CurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Price_CurrencyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Price_CurrencyOptionsRef {
        Price_CurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Price_CurrencyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\nThe maximum unit amount the customer can specify for this item."]
    pub fn custom_unit_amount_maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\nThe minimum unit amount the customer can specify for this item. Must be at least the minimum charge amount."]
    pub fn custom_unit_amount_minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.base))
    }

    #[doc =
        "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\nThe starting unit amount which can be updated by the customer."]
    pub fn custom_unit_amount_preset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount` after provisioning.\nThe unit amount in %s to be charged, represented as a whole integer if possible. Only set if `billing_scheme=per_unit`."]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nThe unit amount in %s to be charged, represented as a decimal string with at most 12 decimal places. Only set if `billing_scheme=per_unit`."]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<Price_CurrencyOptions_TiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Price_Tiers {
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

impl Price_Tiers {
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

impl terrars::ToListMappable for Price_Tiers {
    type O = terrars::BlockAssignable<Price_Tiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPrice_Tiers {
    #[doc = "Up to and including to this quantity will be contained in the tier."]
    pub up_to: terrars::PrimField<std::string::String>,
}

impl BuildPrice_Tiers {
    pub fn build(self) -> Price_Tiers {
        Price_Tiers {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: self.up_to,
        }
    }
}

#[allow(dead_code)]
pub struct Price_TiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Price_TiersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Price_TiersRef {
        Price_TiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Price_TiersRef {
    #[allow(dead_code)]
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
struct PriceDynamic {
    currency_options: std::option::Option<terrars::DynamicBlock<Price_CurrencyOptions>>,
    tiers: std::option::Option<terrars::DynamicBlock<Price_Tiers>>,
}
