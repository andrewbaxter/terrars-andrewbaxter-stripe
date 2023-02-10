use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct CouponData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_off: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    applies_to_products: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_in_months: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_redemptions: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    percent_off: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redeem_by: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_options: Option<Vec<CouponCurrencyOptionsEl>>,
    dynamic: CouponDynamic,
}

struct Coupon_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CouponData>,
}

#[derive(Clone)]
pub struct Coupon(Rc<Coupon_>);

impl Coupon {
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

    #[doc= "Set the field `amount_off`.\nA positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub fn set_amount_off(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().amount_off = Some(v.into());
        self
    }

    #[doc= "Set the field `applies_to_products`.\nA list of product IDs this coupon applies to"]
    pub fn set_applies_to_products(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().applies_to_products = Some(v.into());
        self
    }

    #[doc= "Set the field `currency`.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed)."]
    pub fn set_currency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().currency = Some(v.into());
        self
    }

    #[doc= "Set the field `duration`.\nSpecifies how long the discount will be in effect if used on a subscription. Defaults to `once`."]
    pub fn set_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().duration = Some(v.into());
        self
    }

    #[doc= "Set the field `duration_in_months`.\nRequired only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect."]
    pub fn set_duration_in_months(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().duration_in_months = Some(v.into());
        self
    }

    #[doc= "Set the field `max_redemptions`.\nA positive integer specifying the number of times the coupon can be redeemed before it's no longer valid. For example, you might have a 50% off coupon that the first 20 readers of your blog can use."]
    pub fn set_max_redemptions(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_redemptions = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nName of the coupon displayed to customers on, for instance invoices, or receipts. By default the `id` is shown if `name` is not set."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `percent_off`.\nA positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed)."]
    pub fn set_percent_off(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().percent_off = Some(v.into());
        self
    }

    #[doc= "Set the field `redeem_by`.\nUnix timestamp specifying the last time at which the coupon can be redeemed. After the redeem_by date, the coupon can no longer be applied to new customers."]
    pub fn set_redeem_by(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().redeem_by = Some(v.into());
        self
    }

    #[doc= "Set the field `currency_options`.\n"]
    pub fn set_currency_options(self, v: impl Into<BlockAssignable<CouponCurrencyOptionsEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `amount_off` after provisioning.\nA positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub fn amount_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `applies_to_products` after provisioning.\nA list of product IDs this coupon applies to"]
    pub fn applies_to_products(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.applies_to_products", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed)."]
    pub fn currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\nSpecifies how long the discount will be in effect if used on a subscription. Defaults to `once`."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `duration_in_months` after provisioning.\nRequired only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect."]
    pub fn duration_in_months(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration_in_months", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique string of your choice that will be used to identify this coupon when applying it to a customer. If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the coupon can be redeemed before it's no longer valid. For example, you might have a 50% off coupon that the first 20 readers of your blog can use."]
    pub fn max_redemptions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the coupon displayed to customers on, for instance invoices, or receipts. By default the `id` is shown if `name` is not set."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `percent_off` after provisioning.\nA positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed)."]
    pub fn percent_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_off", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redeem_by` after provisioning.\nUnix timestamp specifying the last time at which the coupon can be redeemed. After the redeem_by date, the coupon can no longer be applied to new customers."]
    pub fn redeem_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.redeem_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this coupon has been applied to a customer."]
    pub fn times_redeemed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid` after provisioning.\nTaking account of the above properties, whether this coupon can still be applied to a customer."]
    pub fn valid(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> ListRef<CouponCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }
}

impl Resource for Coupon {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Coupon {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Coupon {
    type O = ListRef<CouponRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for Coupon_ {
    fn extract_resource_type(&self) -> String {
        "stripe_coupon".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCoupon {
    pub tf_id: String,
}

impl BuildCoupon {
    pub fn build(self, stack: &mut Stack) -> Coupon {
        let out = Coupon(Rc::new(Coupon_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CouponData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CouponRef {
    shared: StackShared,
    base: String,
}

impl Ref for CouponRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CouponRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amount_off` after provisioning.\nA positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub fn amount_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `applies_to_products` after provisioning.\nA list of product IDs this coupon applies to"]
    pub fn applies_to_products(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.applies_to_products", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed)."]
    pub fn currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\nSpecifies how long the discount will be in effect if used on a subscription. Defaults to `once`."]
    pub fn duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `duration_in_months` after provisioning.\nRequired only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect."]
    pub fn duration_in_months(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration_in_months", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique string of your choice that will be used to identify this coupon when applying it to a customer. If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `max_redemptions` after provisioning.\nA positive integer specifying the number of times the coupon can be redeemed before it's no longer valid. For example, you might have a 50% off coupon that the first 20 readers of your blog can use."]
    pub fn max_redemptions(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_redemptions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the coupon displayed to customers on, for instance invoices, or receipts. By default the `id` is shown if `name` is not set."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `percent_off` after provisioning.\nA positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed)."]
    pub fn percent_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percent_off", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redeem_by` after provisioning.\nUnix timestamp specifying the last time at which the coupon can be redeemed. After the redeem_by date, the coupon can no longer be applied to new customers."]
    pub fn redeem_by(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.redeem_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `times_redeemed` after provisioning.\nNumber of times this coupon has been applied to a customer."]
    pub fn times_redeemed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.times_redeemed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid` after provisioning.\nTaking account of the above properties, whether this coupon can still be applied to a customer."]
    pub fn valid(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.valid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency_options` after provisioning.\n"]
    pub fn currency_options(&self) -> ListRef<CouponCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.currency_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CouponCurrencyOptionsEl {
    amount_off: PrimField<f64>,
    key: PrimField<String>,
}

impl CouponCurrencyOptionsEl { }

impl ToListMappable for CouponCurrencyOptionsEl {
    type O = BlockAssignable<CouponCurrencyOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCouponCurrencyOptionsEl {
    #[doc= "Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    pub amount_off: PrimField<f64>,
    #[doc= "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for CouponCurrencyOptionsElRef {
    fn new(shared: StackShared, base: String) -> CouponCurrencyOptionsElRef {
        CouponCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CouponCurrencyOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amount_off` after provisioning.\nAmount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    pub fn amount_off(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct CouponDynamic {
    currency_options: Option<DynamicBlock<CouponCurrencyOptionsEl>>,
}
