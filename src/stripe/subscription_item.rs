use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct SubscriptionItemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing_thresholds_usage_gte: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_currency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_product: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_recurring_interval: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_recurring_interval_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_tax_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_unit_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_unit_amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proration_date: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<PrimField<f64>>,
    subscription: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<ListField<PrimField<String>>>,
}

struct SubscriptionItem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SubscriptionItemData>,
}

#[derive(Clone)]
pub struct SubscriptionItem(Rc<SubscriptionItem_>);

impl SubscriptionItem {
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

    #[doc= "Set the field `billing_thresholds_usage_gte`.\nUsage threshold that triggers the subscription to create an invoice"]
    pub fn set_billing_thresholds_usage_gte(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().billing_thresholds_usage_gte = Some(v.into());
        self
    }

    #[doc= "Set the field `payment_behavior`.\nUse `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid. This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice. For example, SCA regulation may require 3DS authentication to complete payment. See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more. This is the default behavior.\n\nUse `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent. This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice. Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.\n\nUse `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates). When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).\n\nUse `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid. For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead. This was the default behavior for API versions prior to 2019-03-14. See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more."]
    pub fn set_payment_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().payment_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `price`.\nThe ID of the price object."]
    pub fn set_price(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_currency`.\n"]
    pub fn set_price_data_currency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price_data_currency = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_product`.\n"]
    pub fn set_price_data_product(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price_data_product = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_recurring_interval`.\n"]
    pub fn set_price_data_recurring_interval(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price_data_recurring_interval = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_recurring_interval_count`.\n"]
    pub fn set_price_data_recurring_interval_count(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().price_data_recurring_interval_count = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_tax_behavior`.\n"]
    pub fn set_price_data_tax_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price_data_tax_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_unit_amount`.\n"]
    pub fn set_price_data_unit_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().price_data_unit_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `price_data_unit_amount_decimal`.\n"]
    pub fn set_price_data_unit_amount_decimal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().price_data_unit_amount_decimal = Some(v.into());
        self
    }

    #[doc= "Set the field `proration_behavior`.\nDetermines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes."]
    pub fn set_proration_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().proration_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `proration_date`.\nIf set, the proration will be calculated as though the subscription was updated at the given time. This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint."]
    pub fn set_proration_date(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().proration_date = Some(v.into());
        self
    }

    #[doc= "Set the field `quantity`.\nThe quantity you'd like to apply to the subscription item you're creating."]
    pub fn set_quantity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().quantity = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_rates`.\nA list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids. These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription. When updating, pass an empty string to remove previously-defined tax rates."]
    pub fn set_tax_rates(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tax_rates = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `billing_thresholds_usage_gte` after provisioning.\nUsage threshold that triggers the subscription to create an invoice"]
    pub fn billing_thresholds_usage_gte(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_thresholds_usage_gte", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `payment_behavior` after provisioning.\nUse `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid. This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice. For example, SCA regulation may require 3DS authentication to complete payment. See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more. This is the default behavior.\n\nUse `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent. This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice. Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.\n\nUse `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates). When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).\n\nUse `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid. For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead. This was the default behavior for API versions prior to 2019-03-14. See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more."]
    pub fn payment_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payment_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price` after provisioning.\nThe ID of the price object."]
    pub fn price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_currency` after provisioning.\n"]
    pub fn price_data_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_product` after provisioning.\n"]
    pub fn price_data_product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_product", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_recurring_interval` after provisioning.\n"]
    pub fn price_data_recurring_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_recurring_interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_recurring_interval_count` after provisioning.\n"]
    pub fn price_data_recurring_interval_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_recurring_interval_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_tax_behavior` after provisioning.\n"]
    pub fn price_data_tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_unit_amount` after provisioning.\n"]
    pub fn price_data_unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn price_data_unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proration_behavior` after provisioning.\nDetermines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes."]
    pub fn proration_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proration_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proration_date` after provisioning.\nIf set, the proration will be calculated as though the subscription was updated at the given time. This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint."]
    pub fn proration_date(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.proration_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quantity` after provisioning.\nThe quantity you'd like to apply to the subscription item you're creating."]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription` after provisioning.\nThe identifier of the subscription to modify."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_rates` after provisioning.\nA list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids. These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription. When updating, pass an empty string to remove previously-defined tax rates."]
    pub fn tax_rates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.extract_ref()))
    }
}

impl Resource for SubscriptionItem {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for SubscriptionItem {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for SubscriptionItem {
    type O = ListRef<SubscriptionItemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SubscriptionItem_ {
    fn extract_resource_type(&self) -> String {
        "stripe_subscription_item".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSubscriptionItem {
    pub tf_id: String,
    #[doc= "The identifier of the subscription to modify."]
    pub subscription: PrimField<String>,
}

impl BuildSubscriptionItem {
    pub fn build(self, stack: &mut Stack) -> SubscriptionItem {
        let out = SubscriptionItem(Rc::new(SubscriptionItem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SubscriptionItemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                billing_thresholds_usage_gte: core::default::Default::default(),
                payment_behavior: core::default::Default::default(),
                price: core::default::Default::default(),
                price_data_currency: core::default::Default::default(),
                price_data_product: core::default::Default::default(),
                price_data_recurring_interval: core::default::Default::default(),
                price_data_recurring_interval_count: core::default::Default::default(),
                price_data_tax_behavior: core::default::Default::default(),
                price_data_unit_amount: core::default::Default::default(),
                price_data_unit_amount_decimal: core::default::Default::default(),
                proration_behavior: core::default::Default::default(),
                proration_date: core::default::Default::default(),
                quantity: core::default::Default::default(),
                subscription: self.subscription,
                tax_rates: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SubscriptionItemRef {
    shared: StackShared,
    base: String,
}

impl Ref for SubscriptionItemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SubscriptionItemRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `billing_thresholds_usage_gte` after provisioning.\nUsage threshold that triggers the subscription to create an invoice"]
    pub fn billing_thresholds_usage_gte(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.billing_thresholds_usage_gte", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `payment_behavior` after provisioning.\nUse `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid. This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice. For example, SCA regulation may require 3DS authentication to complete payment. See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more. This is the default behavior.\n\nUse `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent. This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice. Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.\n\nUse `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates). When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).\n\nUse `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid. For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead. This was the default behavior for API versions prior to 2019-03-14. See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more."]
    pub fn payment_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payment_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price` after provisioning.\nThe ID of the price object."]
    pub fn price(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_currency` after provisioning.\n"]
    pub fn price_data_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_product` after provisioning.\n"]
    pub fn price_data_product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_product", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_recurring_interval` after provisioning.\n"]
    pub fn price_data_recurring_interval(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_recurring_interval", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_recurring_interval_count` after provisioning.\n"]
    pub fn price_data_recurring_interval_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_recurring_interval_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_tax_behavior` after provisioning.\n"]
    pub fn price_data_tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_unit_amount` after provisioning.\n"]
    pub fn price_data_unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn price_data_unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proration_behavior` after provisioning.\nDetermines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes."]
    pub fn proration_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.proration_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `proration_date` after provisioning.\nIf set, the proration will be calculated as though the subscription was updated at the given time. This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint."]
    pub fn proration_date(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.proration_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quantity` after provisioning.\nThe quantity you'd like to apply to the subscription item you're creating."]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription` after provisioning.\nThe identifier of the subscription to modify."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_rates` after provisioning.\nA list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids. These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription. When updating, pass an empty string to remove previously-defined tax rates."]
    pub fn tax_rates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.extract_ref()))
    }
}
