use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PromotionCodeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
    coupon: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_redemptions: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions_first_time_transaction: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions_minimum_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions_minimum_amount_currency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restrictions_currency_options: Option<Vec<PromotionCodeRestrictionsCurrencyOptionsEl>>,
    dynamic: PromotionCodeDynamic,
}

struct PromotionCode_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PromotionCodeData>,
}

#[derive(Clone)]
pub struct PromotionCode(Rc<PromotionCode_>);

impl PromotionCode {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `code`.\nThe customer-facing code. Regardless of case, this code must be unique across all active promotion codes for a specific customer. If left blank, we will generate one automatically."]
    pub fn set_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().code = Some(v.into());
        self
    }

    #[doc= "Set the field `customer`.\nThe customer that this promotion code can be used by. If not set, the promotion code can be used by all customers."]
    pub fn set_customer(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer = Some(v.into());
        self
    }

    #[doc= "Set the field `expires_at`.\nThe timestamp at which this promotion code will expire. If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`."]
    pub fn set_expires_at(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().expires_at = Some(v.into());
        self
    }

    #[doc= "Set the field `max_redemptions`.\nA positive integer specifying the number of times the promotion code can be redeemed. If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`."]
    pub fn set_max_redemptions(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_redemptions = Some(v.into());
        self
    }

    #[doc= "Set the field `restrictions_first_time_transaction`.\nA Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices"]
    pub fn set_restrictions_first_time_transaction(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().restrictions_first_time_transaction = Some(v.into());
        self
    }

    #[doc= "Set the field `restrictions_minimum_amount`.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn set_restrictions_minimum_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().restrictions_minimum_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `restrictions_minimum_amount_currency`.\nThree-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub fn set_restrictions_minimum_amount_currency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().restrictions_minimum_amount_currency = Some(v.into());
        self
    }

    #[doc= "Set the field `restrictions_currency_options`.\n"]
    pub fn set_restrictions_currency_options(
        self,
        v: impl Into<BlockAssignable<PromotionCodeRestrictionsCurrencyOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restrictions_currency_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restrictions_currency_options = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `code` after provisioning.\nThe customer-facing code. Regardless of case, this code must be unique across all active promotion codes for a specific customer. If left blank, we will generate one automatically."]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `coupon` after provisioning.\nThe coupon for this promotion code."]
    pub fn coupon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer` after provisioning.\nThe customer that this promotion code can be used by. If not set, the promotion code can be used by all customers."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nThe timestamp at which this promotion code will expire. If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`."]
    pub fn expires_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the promotion code can be redeemed. If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`."]
    pub fn max_redemptions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_first_time_transaction` after provisioning.\nA Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices"]
    pub fn restrictions_first_time_transaction(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrictions_first_time_transaction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_minimum_amount` after provisioning.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn restrictions_minimum_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrictions_minimum_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_minimum_amount_currency` after provisioning.\nThree-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub fn restrictions_minimum_amount_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrictions_minimum_amount_currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this promotion code has been used."]
    pub fn times_redeemed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_currency_options` after provisioning.\n"]
    pub fn restrictions_currency_options(&self) -> ListRef<PromotionCodeRestrictionsCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restrictions_currency_options", self.extract_ref()))
    }
}

impl Resource for PromotionCode {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for PromotionCode {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for PromotionCode {
    type O = ListRef<PromotionCodeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for PromotionCode_ {
    fn extract_resource_type(&self) -> String {
        "stripe_promotion_code".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPromotionCode {
    pub tf_id: String,
    #[doc= "The coupon for this promotion code."]
    pub coupon: PrimField<String>,
}

impl BuildPromotionCode {
    pub fn build(self, stack: &mut Stack) -> PromotionCode {
        let out = PromotionCode(Rc::new(PromotionCode_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PromotionCodeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                code: core::default::Default::default(),
                coupon: self.coupon,
                customer: core::default::Default::default(),
                expires_at: core::default::Default::default(),
                max_redemptions: core::default::Default::default(),
                restrictions_first_time_transaction: core::default::Default::default(),
                restrictions_minimum_amount: core::default::Default::default(),
                restrictions_minimum_amount_currency: core::default::Default::default(),
                restrictions_currency_options: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PromotionCodeRef {
    shared: StackShared,
    base: String,
}

impl Ref for PromotionCodeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PromotionCodeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `code` after provisioning.\nThe customer-facing code. Regardless of case, this code must be unique across all active promotion codes for a specific customer. If left blank, we will generate one automatically."]
    pub fn code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `coupon` after provisioning.\nThe coupon for this promotion code."]
    pub fn coupon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer` after provisioning.\nThe customer that this promotion code can be used by. If not set, the promotion code can be used by all customers."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expires_at` after provisioning.\nThe timestamp at which this promotion code will expire. If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`."]
    pub fn expires_at(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the promotion code can be redeemed. If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`."]
    pub fn max_redemptions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_first_time_transaction` after provisioning.\nA Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices"]
    pub fn restrictions_first_time_transaction(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrictions_first_time_transaction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_minimum_amount` after provisioning.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn restrictions_minimum_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrictions_minimum_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_minimum_amount_currency` after provisioning.\nThree-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub fn restrictions_minimum_amount_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.restrictions_minimum_amount_currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this promotion code has been used."]
    pub fn times_redeemed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `restrictions_currency_options` after provisioning.\n"]
    pub fn restrictions_currency_options(&self) -> ListRef<PromotionCodeRestrictionsCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.restrictions_currency_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PromotionCodeRestrictionsCurrencyOptionsEl {
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_amount: Option<PrimField<f64>>,
}

impl PromotionCodeRestrictionsCurrencyOptionsEl {
    #[doc= "Set the field `minimum_amount`.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn set_minimum_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_amount = Some(v.into());
        self
    }
}

impl ToListMappable for PromotionCodeRestrictionsCurrencyOptionsEl {
    type O = BlockAssignable<PromotionCodeRestrictionsCurrencyOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPromotionCodeRestrictionsCurrencyOptionsEl {
    #[doc= "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: PrimField<String>,
}

impl BuildPromotionCodeRestrictionsCurrencyOptionsEl {
    pub fn build(self) -> PromotionCodeRestrictionsCurrencyOptionsEl {
        PromotionCodeRestrictionsCurrencyOptionsEl {
            key: self.key,
            minimum_amount: core::default::Default::default(),
        }
    }
}

pub struct PromotionCodeRestrictionsCurrencyOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PromotionCodeRestrictionsCurrencyOptionsElRef {
    fn new(shared: StackShared, base: String) -> PromotionCodeRestrictionsCurrencyOptionsElRef {
        PromotionCodeRestrictionsCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PromotionCodeRestrictionsCurrencyOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `minimum_amount` after provisioning.\nMinimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub fn minimum_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_amount", self.base))
    }
}

#[derive(Serialize, Default)]
struct PromotionCodeDynamic {
    restrictions_currency_options: Option<DynamicBlock<PromotionCodeRestrictionsCurrencyOptionsEl>>,
}
