#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct ShippingRateData_ {
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
    delivery_estimate_maximum_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_estimate_maximum_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_estimate_minimum_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delivery_estimate_minimum_value: std::option::Option<terrars::PrimField<f64>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_amount_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_amount_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_amount_currency_options: std::option::Option<std::vec::Vec<ShippingRate_FixedAmountCurrencyOptions>>,
    dynamic: ShippingRateDynamic,
}

struct ShippingRate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ShippingRateData_>,
}

#[derive(Clone)]
pub struct ShippingRate(std::rc::Rc<ShippingRate_>);

impl ShippingRate {
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

    #[doc = "Set the field `delivery_estimate_maximum_unit`.\nA unit of time."]
    pub fn set_delivery_estimate_maximum_unit(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().delivery_estimate_maximum_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_estimate_maximum_value`.\nMust be greater than 0."]
    pub fn set_delivery_estimate_maximum_value(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().delivery_estimate_maximum_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_estimate_minimum_unit`.\nA unit of time."]
    pub fn set_delivery_estimate_minimum_unit(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().delivery_estimate_minimum_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delivery_estimate_minimum_value`.\nMust be greater than 0."]
    pub fn set_delivery_estimate_minimum_value(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().delivery_estimate_minimum_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_amount_amount`.\nA non-negative integer in cents representing how much to charge."]
    pub fn set_fixed_amount_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().fixed_amount_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fixed_amount_currency`.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn set_fixed_amount_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fixed_amount_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_behavior`.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn set_tax_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_code`.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`."]
    pub fn set_tax_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of calculation to use on the shipping rate. Can only be `fixed_amount` for now."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fixed_amount_currency_options`.\n"]
    pub fn set_fixed_amount_currency_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ShippingRate_FixedAmountCurrencyOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fixed_amount_currency_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fixed_amount_currency_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_maximum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_maximum_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_maximum_unit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_maximum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_maximum_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_maximum_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_minimum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_minimum_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_minimum_unit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_minimum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_minimum_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_minimum_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fixed_amount_amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn fixed_amount_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fixed_amount_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn fixed_amount_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_currency", self.extract_ref()))
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
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`."]
    pub fn tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of calculation to use on the shipping rate. Can only be `fixed_amount` for now."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fixed_amount_currency_options` after provisioning.\n"]
    pub fn fixed_amount_currency_options(&self) -> terrars::ListRef<ShippingRate_FixedAmountCurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_amount_currency_options", self.extract_ref()))
    }
}

impl terrars::Referable for ShippingRate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ShippingRate { }

impl terrars::ToListMappable for ShippingRate {
    type O = terrars::ListRef<ShippingRateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ShippingRate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_shipping_rate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildShippingRate {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildShippingRate {
    pub fn build(self, stack: &mut terrars::Stack) -> ShippingRate {
        let out = ShippingRate(std::rc::Rc::new(ShippingRate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ShippingRateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delivery_estimate_maximum_unit: core::default::Default::default(),
                delivery_estimate_maximum_value: core::default::Default::default(),
                delivery_estimate_minimum_unit: core::default::Default::default(),
                delivery_estimate_minimum_value: core::default::Default::default(),
                display_name: self.display_name,
                fixed_amount_amount: core::default::Default::default(),
                fixed_amount_currency: core::default::Default::default(),
                tax_behavior: core::default::Default::default(),
                tax_code: core::default::Default::default(),
                type_: core::default::Default::default(),
                fixed_amount_currency_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ShippingRateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ShippingRateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ShippingRateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_maximum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_maximum_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_maximum_unit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_maximum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_maximum_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_maximum_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_minimum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_minimum_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_minimum_unit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `delivery_estimate_minimum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_minimum_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delivery_estimate_minimum_value", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fixed_amount_amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn fixed_amount_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fixed_amount_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn fixed_amount_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_currency", self.extract_ref()))
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
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`."]
    pub fn tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of calculation to use on the shipping rate. Can only be `fixed_amount` for now."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fixed_amount_currency_options` after provisioning.\n"]
    pub fn fixed_amount_currency_options(&self) -> terrars::ListRef<ShippingRate_FixedAmountCurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_amount_currency_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ShippingRate_FixedAmountCurrencyOptions {
    amount: terrars::PrimField<f64>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ShippingRate_FixedAmountCurrencyOptions {
    #[doc =
        "Set the field `tax_behavior`.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn set_tax_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ShippingRate_FixedAmountCurrencyOptions {
    type O = terrars::BlockAssignable<ShippingRate_FixedAmountCurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildShippingRate_FixedAmountCurrencyOptions {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: terrars::PrimField<f64>,
    #[doc = "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildShippingRate_FixedAmountCurrencyOptions {
    pub fn build(self) -> ShippingRate_FixedAmountCurrencyOptions {
        ShippingRate_FixedAmountCurrencyOptions {
            amount: self.amount,
            key: self.key,
            tax_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ShippingRate_FixedAmountCurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ShippingRate_FixedAmountCurrencyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ShippingRate_FixedAmountCurrencyOptionsRef {
        ShippingRate_FixedAmountCurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ShippingRate_FixedAmountCurrencyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.base))
    }
}

#[derive(Serialize, Default)]
struct ShippingRateDynamic {
    fixed_amount_currency_options: std
    ::option
    ::Option<terrars::DynamicBlock<ShippingRate_FixedAmountCurrencyOptions>>,
}
