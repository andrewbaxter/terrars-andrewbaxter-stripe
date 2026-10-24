use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct CouponData {
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
    amount_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    applies_to_products: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration_in_months: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_redemptions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redeem_by: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_options: std::option::Option<std::vec::Vec<CouponCurrencyOptionsEl>>,
    dynamic: CouponDynamic,
}

struct Coupon_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CouponData>,
}

#[derive(Clone)]
pub struct Coupon(std::rc::Rc<Coupon_>);

impl Coupon {
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
        "Set the field `amount_off`.\nA positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub fn set_amount_off(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().amount_off = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `applies_to_products`.\nA list of product IDs this coupon applies to"]
    pub fn set_applies_to_products(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().applies_to_products = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `currency`.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed)."]
    pub fn set_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().currency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `duration`.\nSpecifies how long the discount will be in effect if used on a subscription. Defaults to `once`."]
    pub fn set_duration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `duration_in_months`.\nRequired only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect."]
    pub fn set_duration_in_months(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().duration_in_months = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_redemptions`.\nA positive integer specifying the number of times the coupon can be redeemed before it's no longer valid. For example, you might have a 50% off coupon that the first 20 readers of your blog can use."]
    pub fn set_max_redemptions(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_redemptions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nName of the coupon displayed to customers on, for instance invoices, or receipts. By default the `id` is shown if `name` is not set."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percent_off`.\nA positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed)."]
    pub fn set_percent_off(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().percent_off = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redeem_by`.\nUnix timestamp specifying the last time at which the coupon can be redeemed. After the redeem_by date, the coupon can no longer be applied to new customers."]
    pub fn set_redeem_by(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().redeem_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `currency_options`.\n"]
    pub fn set_currency_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CouponCurrencyOptionsEl>>,
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

    #[doc =
        "Get a reference to the value of field `amount_off` after provisioning.\nA positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub fn amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_products` after provisioning.\nA list of product IDs this coupon applies to"]
    pub fn applies_to_products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applies_to_products", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed)."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nSpecifies how long the discount will be in effect if used on a subscription. Defaults to `once`."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration_in_months` after provisioning.\nRequired only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect."]
    pub fn duration_in_months(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_months", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique string of your choice that will be used to identify this coupon when applying it to a customer. If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the coupon can be redeemed before it's no longer valid. For example, you might have a 50% off coupon that the first 20 readers of your blog can use."]
    pub fn max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the coupon displayed to customers on, for instance invoices, or receipts. By default the `id` is shown if `name` is not set."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_off` after provisioning.\nA positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed)."]
    pub fn percent_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redeem_by` after provisioning.\nUnix timestamp specifying the last time at which the coupon can be redeemed. After the redeem_by date, the coupon can no longer be applied to new customers."]
    pub fn redeem_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redeem_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this coupon has been applied to a customer."]
    pub fn times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valid` after provisioning.\nTaking account of the above properties, whether this coupon can still be applied to a customer."]
    pub fn valid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> terrars::ListRef<CouponCurrencyOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }
}

impl terrars::Referable for Coupon {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Coupon { }

impl terrars::ToListMappable for Coupon {
    type O = terrars::ListRef<CouponRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Coupon_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_coupon".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCoupon {
    pub tf_id: std::string::String,
}

impl BuildCoupon {
    pub fn build(self, stack: &mut terrars::Stack) -> Coupon {
        let out = Coupon(std::rc::Rc::new(Coupon_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CouponData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                amount_off: core::default::Default::default(),
                applies_to_products: core::default::Default::default(),
                currency: core::default::Default::default(),
                duration: core::default::Default::default(),
                duration_in_months: core::default::Default::default(),
                max_redemptions: core::default::Default::default(),
                name: core::default::Default::default(),
                percent_off: core::default::Default::default(),
                redeem_by: core::default::Default::default(),
                currency_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CouponRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CouponRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CouponRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `amount_off` after provisioning.\nA positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub fn amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `applies_to_products` after provisioning.\nA list of product IDs this coupon applies to"]
    pub fn applies_to_products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applies_to_products", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed)."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration` after provisioning.\nSpecifies how long the discount will be in effect if used on a subscription. Defaults to `once`."]
    pub fn duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `duration_in_months` after provisioning.\nRequired only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect."]
    pub fn duration_in_months(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_in_months", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique string of your choice that will be used to identify this coupon when applying it to a customer. If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the coupon can be redeemed before it's no longer valid. For example, you might have a 50% off coupon that the first 20 readers of your blog can use."]
    pub fn max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the coupon displayed to customers on, for instance invoices, or receipts. By default the `id` is shown if `name` is not set."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `percent_off` after provisioning.\nA positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed)."]
    pub fn percent_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redeem_by` after provisioning.\nUnix timestamp specifying the last time at which the coupon can be redeemed. After the redeem_by date, the coupon can no longer be applied to new customers."]
    pub fn redeem_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redeem_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this coupon has been applied to a customer."]
    pub fn times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valid` after provisioning.\nTaking account of the above properties, whether this coupon can still be applied to a customer."]
    pub fn valid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> terrars::ListRef<CouponCurrencyOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CouponCurrencyOptionsEl {
    amount_off: terrars::PrimField<f64>,
    key: terrars::PrimField<std::string::String>,
}

impl CouponCurrencyOptionsEl { }

impl terrars::ToListMappable for CouponCurrencyOptionsEl {
    type O = terrars::BlockAssignable<CouponCurrencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCouponCurrencyOptionsEl {
    #[doc =
        "Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    pub amount_off: terrars::PrimField<f64>,
    #[doc = "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildCouponCurrencyOptionsEl {
    pub fn build(self) -> CouponCurrencyOptionsEl {
        CouponCurrencyOptionsEl {
            amount_off: self.amount_off,
            key: self.key,
        }
    }
}

pub struct CouponCurrencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CouponCurrencyOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CouponCurrencyOptionsElRef {
        CouponCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CouponCurrencyOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `amount_off` after provisioning.\nAmount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    pub fn amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct CouponDynamic {
    currency_options: std::option::Option<terrars::DynamicBlock<CouponCurrencyOptionsEl>>,
}
