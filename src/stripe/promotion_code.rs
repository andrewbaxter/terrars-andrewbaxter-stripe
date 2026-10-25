#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PromotionCodeData_ {
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
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    coupon: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expires_at: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_redemptions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions_first_time_transaction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions_minimum_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions_minimum_amount_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrictions_currency_options: std::option::Option<std::vec::Vec<PromotionCode_RestrictionsCurrencyOptions>>,
    dynamic: PromotionCodeDynamic,
}

struct PromotionCode_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PromotionCodeData_>,
}

#[derive(Clone)]
pub struct PromotionCode(std::rc::Rc<PromotionCode_>);

impl PromotionCode {
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
        "Set the field `code`.\nThe customer-facing code. Regardless of case, this code must be unique across all active promotion codes for a specific customer. If left blank, we will generate one automatically."]
    pub fn set_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `customer`.\nThe customer that this promotion code can be used by. If not set, the promotion code can be used by all customers."]
    pub fn set_customer(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expires_at`.\nThe timestamp at which this promotion code will expire. If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`."]
    pub fn set_expires_at(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().expires_at = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_redemptions`.\nA positive integer specifying the number of times the promotion code can be redeemed. If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`."]
    pub fn set_max_redemptions(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_redemptions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restrictions_first_time_transaction`.\nA Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices"]
    pub fn set_restrictions_first_time_transaction(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().restrictions_first_time_transaction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restrictions_minimum_amount`.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn set_restrictions_minimum_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().restrictions_minimum_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restrictions_minimum_amount_currency`.\nThree-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub fn set_restrictions_minimum_amount_currency(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().restrictions_minimum_amount_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restrictions_currency_options`.\n"]
    pub fn set_restrictions_currency_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PromotionCode_RestrictionsCurrencyOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions_currency_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions_currency_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nThe customer-facing code. Regardless of case, this code must be unique across all active promotion codes for a specific customer. If left blank, we will generate one automatically."]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `coupon` after provisioning.\nThe coupon for this promotion code."]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe customer that this promotion code can be used by. If not set, the promotion code can be used by all customers."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nThe timestamp at which this promotion code will expire. If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`."]
    pub fn expires_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
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
        "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the promotion code can be redeemed. If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`."]
    pub fn max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restrictions_first_time_transaction` after provisioning.\nA Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices"]
    pub fn restrictions_first_time_transaction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrictions_first_time_transaction", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `restrictions_minimum_amount` after provisioning.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn restrictions_minimum_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrictions_minimum_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restrictions_minimum_amount_currency` after provisioning.\nThree-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub fn restrictions_minimum_amount_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrictions_minimum_amount_currency", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this promotion code has been used."]
    pub fn times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions_currency_options` after provisioning.\n"]
    pub fn restrictions_currency_options(&self) -> terrars::ListRef<PromotionCode_RestrictionsCurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions_currency_options", self.extract_ref()))
    }
}

impl terrars::Referable for PromotionCode {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PromotionCode { }

impl terrars::ToListMappable for PromotionCode {
    type O = terrars::ListRef<PromotionCodeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PromotionCode_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_promotion_code".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPromotionCode {
    pub tf_id: std::string::String,
    #[doc = "The coupon for this promotion code."]
    pub coupon: terrars::PrimField<std::string::String>,
}

impl BuildPromotionCode {
    pub fn build(self, stack: &mut terrars::Stack) -> PromotionCode {
        let out = PromotionCode(std::rc::Rc::new(PromotionCode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PromotionCodeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                code: core::default::Default::default(),
                coupon: self.coupon,
                customer: core::default::Default::default(),
                expires_at: core::default::Default::default(),
                max_redemptions: core::default::Default::default(),
                restrictions_first_time_transaction: core::default::Default::default(),
                restrictions_minimum_amount: core::default::Default::default(),
                restrictions_minimum_amount_currency: core::default::Default::default(),
                restrictions_currency_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PromotionCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PromotionCodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PromotionCodeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nThe customer-facing code. Regardless of case, this code must be unique across all active promotion codes for a specific customer. If left blank, we will generate one automatically."]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `coupon` after provisioning.\nThe coupon for this promotion code."]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe customer that this promotion code can be used by. If not set, the promotion code can be used by all customers."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nThe timestamp at which this promotion code will expire. If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`."]
    pub fn expires_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
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
        "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the promotion code can be redeemed. If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`."]
    pub fn max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restrictions_first_time_transaction` after provisioning.\nA Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices"]
    pub fn restrictions_first_time_transaction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrictions_first_time_transaction", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `restrictions_minimum_amount` after provisioning.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn restrictions_minimum_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrictions_minimum_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `restrictions_minimum_amount_currency` after provisioning.\nThree-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub fn restrictions_minimum_amount_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.restrictions_minimum_amount_currency", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this promotion code has been used."]
    pub fn times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restrictions_currency_options` after provisioning.\n"]
    pub fn restrictions_currency_options(&self) -> terrars::ListRef<PromotionCode_RestrictionsCurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restrictions_currency_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PromotionCode_RestrictionsCurrencyOptions {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_amount: std::option::Option<terrars::PrimField<f64>>,
}

impl PromotionCode_RestrictionsCurrencyOptions {
    #[doc =
        "Set the field `minimum_amount`.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn set_minimum_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_amount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PromotionCode_RestrictionsCurrencyOptions {
    type O = terrars::BlockAssignable<PromotionCode_RestrictionsCurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPromotionCode_RestrictionsCurrencyOptions {
    #[doc = "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildPromotionCode_RestrictionsCurrencyOptions {
    pub fn build(self) -> PromotionCode_RestrictionsCurrencyOptions {
        PromotionCode_RestrictionsCurrencyOptions {
            key: self.key,
            minimum_amount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PromotionCode_RestrictionsCurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PromotionCode_RestrictionsCurrencyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PromotionCode_RestrictionsCurrencyOptionsRef {
        PromotionCode_RestrictionsCurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PromotionCode_RestrictionsCurrencyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_amount` after provisioning.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn minimum_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_amount", self.base))
    }
}

#[derive(Serialize, Default)]
struct PromotionCodeDynamic {
    restrictions_currency_options: std
    ::option
    ::Option<terrars::DynamicBlock<PromotionCode_RestrictionsCurrencyOptions>>,
}
