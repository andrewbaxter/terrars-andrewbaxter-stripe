use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct ShippingRateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_estimate_maximum_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_estimate_maximum_value: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_estimate_minimum_unit: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_estimate_minimum_value: Option<PrimField<f64>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_amount_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_amount_currency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_amount_currency_options: Option<Vec<ShippingRateFixedAmountCurrencyOptionsEl>>,
    dynamic: ShippingRateDynamic,
}

struct ShippingRate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ShippingRateData>,
}

#[derive(Clone)]
pub struct ShippingRate(Rc<ShippingRate_>);

impl ShippingRate {
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

    #[doc= "Set the field `delivery_estimate_maximum_unit`.\nA unit of time."]
    pub fn set_delivery_estimate_maximum_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_estimate_maximum_unit = Some(v.into());
        self
    }

    #[doc= "Set the field `delivery_estimate_maximum_value`.\nMust be greater than 0."]
    pub fn set_delivery_estimate_maximum_value(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().delivery_estimate_maximum_value = Some(v.into());
        self
    }

    #[doc= "Set the field `delivery_estimate_minimum_unit`.\nA unit of time."]
    pub fn set_delivery_estimate_minimum_unit(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().delivery_estimate_minimum_unit = Some(v.into());
        self
    }

    #[doc= "Set the field `delivery_estimate_minimum_value`.\nMust be greater than 0."]
    pub fn set_delivery_estimate_minimum_value(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().delivery_estimate_minimum_value = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_amount_amount`.\nA non-negative integer in cents representing how much to charge."]
    pub fn set_fixed_amount_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().fixed_amount_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_amount_currency`.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn set_fixed_amount_currency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().fixed_amount_currency = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_behavior`.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn set_tax_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tax_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_code`.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`."]
    pub fn set_tax_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tax_code = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of calculation to use on the shipping rate. Can only be `fixed_amount` for now."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `fixed_amount_currency_options`.\n"]
    pub fn set_fixed_amount_currency_options(
        self,
        v: impl Into<BlockAssignable<ShippingRateFixedAmountCurrencyOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fixed_amount_currency_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fixed_amount_currency_options = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_maximum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_maximum_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_maximum_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_maximum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_maximum_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_maximum_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_minimum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_minimum_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_minimum_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_minimum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_minimum_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_minimum_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_amount_amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn fixed_amount_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_amount_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn fixed_amount_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`."]
    pub fn tax_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of calculation to use on the shipping rate. Can only be `fixed_amount` for now."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_amount_currency_options` after provisioning.\n"]
    pub fn fixed_amount_currency_options(&self) -> ListRef<ShippingRateFixedAmountCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_amount_currency_options", self.extract_ref()))
    }
}

impl Resource for ShippingRate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ShippingRate {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ShippingRate {
    type O = ListRef<ShippingRateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for ShippingRate_ {
    fn extract_resource_type(&self) -> String {
        "stripe_shipping_rate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildShippingRate {
    pub tf_id: String,
    #[doc= "The name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions."]
    pub display_name: PrimField<String>,
}

impl BuildShippingRate {
    pub fn build(self, stack: &mut Stack) -> ShippingRate {
        let out = ShippingRate(Rc::new(ShippingRate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ShippingRateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ShippingRateRef {
    shared: StackShared,
    base: String,
}

impl Ref for ShippingRateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ShippingRateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_maximum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_maximum_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_maximum_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_maximum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_maximum_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_maximum_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_minimum_unit` after provisioning.\nA unit of time."]
    pub fn delivery_estimate_minimum_unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_minimum_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_estimate_minimum_value` after provisioning.\nMust be greater than 0."]
    pub fn delivery_estimate_minimum_value(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_estimate_minimum_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe name of the shipping rate, meant to be displayable to the customer. This will appear on CheckoutSessions."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_amount_amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn fixed_amount_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_amount_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn fixed_amount_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fixed_amount_currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID. The Shipping tax code is `txcd_92010001`."]
    pub fn tax_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of calculation to use on the shipping rate. Can only be `fixed_amount` for now."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fixed_amount_currency_options` after provisioning.\n"]
    pub fn fixed_amount_currency_options(&self) -> ListRef<ShippingRateFixedAmountCurrencyOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.fixed_amount_currency_options", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ShippingRateFixedAmountCurrencyOptionsEl {
    amount: PrimField<f64>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<PrimField<String>>,
}

impl ShippingRateFixedAmountCurrencyOptionsEl {
    #[doc= "Set the field `tax_behavior`.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn set_tax_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tax_behavior = Some(v.into());
        self
    }
}

impl ToListMappable for ShippingRateFixedAmountCurrencyOptionsEl {
    type O = BlockAssignable<ShippingRateFixedAmountCurrencyOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildShippingRateFixedAmountCurrencyOptionsEl {
    #[doc= "A non-negative integer in cents representing how much to charge."]
    pub amount: PrimField<f64>,
    #[doc= "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: PrimField<String>,
}

impl BuildShippingRateFixedAmountCurrencyOptionsEl {
    pub fn build(self) -> ShippingRateFixedAmountCurrencyOptionsEl {
        ShippingRateFixedAmountCurrencyOptionsEl {
            amount: self.amount,
            key: self.key,
            tax_behavior: core::default::Default::default(),
        }
    }
}

pub struct ShippingRateFixedAmountCurrencyOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ShippingRateFixedAmountCurrencyOptionsElRef {
    fn new(shared: StackShared, base: String) -> ShippingRateFixedAmountCurrencyOptionsElRef {
        ShippingRateFixedAmountCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ShippingRateFixedAmountCurrencyOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the rate is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.base))
    }
}

#[derive(Serialize, Default)]
struct ShippingRateDynamic {
    fixed_amount_currency_options: Option<DynamicBlock<ShippingRateFixedAmountCurrencyOptionsEl>>,
}
