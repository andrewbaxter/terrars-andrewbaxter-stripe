#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct SubscriptionData_ {
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
    application_fee_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_tax_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backdate_start_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_cycle_anchor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_thresholds_amount_gte: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_thresholds_reset_billing_cycle_anchor: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cancel_at: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cancel_at_period_end: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coupon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    customer: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_until_due: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_payment_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_tax_rates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    off_session: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_behalf_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_acss_debit_verification_method: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_bancontact_preferred_language: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_mandate_options_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_mandate_options_amount_type: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_mandate_options_description: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_request_three_d_secure: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_customer_balance_bank_transfer_type: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_customer_balance_funding_type: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_us_bank_account_financial_connections_permissions: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_us_bank_account_verification_method: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_types: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_save_default_payment_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_invoice_item_interval_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_invoice_item_interval_interval_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    promotion_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proration_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_amount_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_from_plan: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_invoice_items: std::option::Option<std::vec::Vec<Subscription_AddInvoiceItems>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<Subscription_Items>>,
    dynamic: SubscriptionDynamic,
}

struct Subscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SubscriptionData_>,
}

#[derive(Clone)]
pub struct Subscription(std::rc::Rc<Subscription_>);

impl Subscription {
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
        "Set the field `application_fee_percent`.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. The request must be made by a platform account on a connected account in order to set an application fee percentage. For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions)."]
    pub fn set_application_fee_percent(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().application_fee_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_tax_enabled`.\nWhether Stripe automatically computes tax on this subscription."]
    pub fn set_automatic_tax_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_tax_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `backdate_start_date`.\nFor new subscriptions, a past timestamp to backdate the subscription's start date to. If set, the first invoice will contain a proration for the timespan between the start date and the current time. Can be combined with trials and the billing cycle anchor."]
    pub fn set_backdate_start_date(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().backdate_start_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `billing_cycle_anchor`.\nA future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format."]
    pub fn set_billing_cycle_anchor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().billing_cycle_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `billing_thresholds_amount_gte`.\nMonetary threshold that triggers the subscription to create an invoice"]
    pub fn set_billing_thresholds_amount_gte(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().billing_thresholds_amount_gte = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `billing_thresholds_reset_billing_cycle_anchor`.\nIndicates if the `billing_cycle_anchor` should be reset when a threshold is reached. If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged. This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`."]
    pub fn set_billing_thresholds_reset_billing_cycle_anchor(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().billing_thresholds_reset_billing_cycle_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cancel_at`.\nA timestamp at which the subscription should cancel. If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`. If set during a future period, this will always cause a proration for that period."]
    pub fn set_cancel_at(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cancel_at = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cancel_at_period_end`.\nBoolean indicating whether this subscription should cancel at the end of the current period."]
    pub fn set_cancel_at_period_end(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().cancel_at_period_end = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `collection_method`.\nEither `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`. Defaults to `charge_automatically`."]
    pub fn set_collection_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().collection_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `coupon`.\nThe ID of the coupon to apply to this subscription. A coupon applied to a subscription will only affect invoices created for that particular subscription."]
    pub fn set_coupon(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().coupon = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `currency`.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn set_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().currency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `days_until_due`.\nNumber of days a customer has to pay invoices generated by this subscription. Valid only for subscriptions where `collection_method` is set to `send_invoice`."]
    pub fn set_days_until_due(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().days_until_due = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_payment_method`.\nID of the default payment method for the subscription. It must belong to the customer associated with the subscription. This takes precedence over `default_source`. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source)."]
    pub fn set_default_payment_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_source`.\nID of the default payment source for the subscription. It must belong to the customer associated with the subscription and be in a chargeable state. If `default_payment_method` is also set, `default_payment_method` will take precedence. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source)."]
    pub fn set_default_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_tax_rates`.\nThe tax rates that will apply to any subscription item that does not have `tax_rates` set. Invoices created will have their `default_tax_rates` populated from the subscription."]
    pub fn set_default_tax_rates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().default_tax_rates = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nThe subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `off_session`.\nIndicates if a customer is on or off-session while an invoice payment is attempted."]
    pub fn set_off_session(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().off_session = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `on_behalf_of`.\nThe account on behalf of which to charge, for each of the subscription's invoices."]
    pub fn set_on_behalf_of(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().on_behalf_of = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_behavior`.\nOnly applies to subscriptions with `collection_method=charge_automatically`.\n\nUse `allow_incomplete` to create subscriptions with `status=incomplete` if the first invoice cannot be paid. Creating subscriptions with this status allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice. For example, SCA regulation may require 3DS authentication to complete payment. See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more. This is the default behavior.\n\nUse `default_incomplete` to create Subscriptions with `status=incomplete` when the first invoice requires payment, otherwise start as active. Subscriptions transition to `status=active` when successfully confirming the payment intent on the first invoice. This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice. Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method. If the payment intent is not confirmed within 23 hours subscriptions transition to `status=incomplete_expired`, which is a terminal state.\n\nUse `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's first invoice cannot be paid. For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not create a subscription and returns an error instead. This was the default behavior for API versions prior to 2019-03-14. See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.\n\n`pending_if_incomplete` is only used with updates and cannot be passed when creating a subscription.\n\nSubscriptions with `collection_method=send_invoice` are automatically activated regardless of the first invoice status."]
    pub fn set_payment_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().payment_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type`.\nTransaction type of the mandate."]
    pub fn set_payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_acss_debit_verification_method`.\nBank account verification method."]
    pub fn set_payment_settings_payment_method_options_acss_debit_verification_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_acss_debit_verification_method =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_bancontact_preferred_language`.\nPreferred language of the Bancontact authorization page that the customer is redirected to."]
    pub fn set_payment_settings_payment_method_options_bancontact_preferred_language(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_bancontact_preferred_language =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_card_mandate_options_amount`.\nAmount to be charged for future payments."]
    pub fn set_payment_settings_payment_method_options_card_mandate_options_amount(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_mandate_options_amount =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_card_mandate_options_amount_type`.\nOne of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param."]
    pub fn set_payment_settings_payment_method_options_card_mandate_options_amount_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_mandate_options_amount_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_card_mandate_options_description`.\nA description of the mandate or subscription that is meant to be displayed to the customer."]
    pub fn set_payment_settings_payment_method_options_card_mandate_options_description(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_mandate_options_description =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_card_network`.\nSelected network to process this Subscription on. Depends on the available networks of the card attached to the Subscription. Can be only set confirm-time."]
    pub fn set_payment_settings_payment_method_options_card_network(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_network =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_card_request_three_d_secure`.\nWe strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine."]
    pub fn set_payment_settings_payment_method_options_card_request_three_d_secure(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_request_three_d_secure =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country`.\nThe desired country code of the bank account information. Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`."]
    pub fn set_payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self
            .0
            .data
            .borrow_mut()
            .payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_customer_balance_bank_transfer_type`.\nThe bank transfer type that can be used for funding. Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`."]
    pub fn set_payment_settings_payment_method_options_customer_balance_bank_transfer_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_customer_balance_bank_transfer_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_customer_balance_funding_type`.\nThe funding method type to be used when there are not enough funds in the customer balance. Permitted values include: `bank_transfer`."]
    pub fn set_payment_settings_payment_method_options_customer_balance_funding_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_customer_balance_funding_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_us_bank_account_financial_connections_permissions`.\nThe list of permissions to request. The `payment_method` permission must be included."]
    pub fn set_payment_settings_payment_method_options_us_bank_account_financial_connections_permissions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self
            .0
            .data
            .borrow_mut()
            .payment_settings_payment_method_options_us_bank_account_financial_connections_permissions =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_us_bank_account_verification_method`.\nBank account verification method."]
    pub fn set_payment_settings_payment_method_options_us_bank_account_verification_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_us_bank_account_verification_method =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_types`.\nThe list of payment method types to provide to every invoice created by the subscription. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice)."]
    pub fn set_payment_settings_payment_method_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_save_default_payment_method`.\nEither `off`, or `on_subscription`. With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds."]
    pub fn set_payment_settings_save_default_payment_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_save_default_payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pending_invoice_item_interval_interval`.\nSpecifies invoicing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn set_pending_invoice_item_interval_interval(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().pending_invoice_item_interval_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pending_invoice_item_interval_interval_count`.\nThe number of intervals between invoices. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn set_pending_invoice_item_interval_interval_count(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().pending_invoice_item_interval_interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `promotion_code`.\nThe API ID of a promotion code to apply to this subscription. A promotion code applied to a subscription will only affect invoices created for that particular subscription."]
    pub fn set_promotion_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().promotion_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `proration_behavior`.\nDetermines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`. If no value is passed, the default is `create_prorations`."]
    pub fn set_proration_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().proration_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_data_amount_percent`.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account. By default, the entire amount is transferred to the destination."]
    pub fn set_transfer_data_amount_percent(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transfer_data_amount_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_data_destination`.\nThe account where funds from the payment will be transferred to upon payment success."]
    pub fn set_transfer_data_destination(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transfer_data_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trial_end`.\nUnix timestamp representing the end of the trial period the customer will get before being charged for the first time. This will always overwrite any trials that might apply via a subscribed plan. If set, trial_end will override the default trial period of the plan the customer is being subscribed to. The special value `now` can be provided to end the customer's trial immediately. Can be at most two years from `billing_cycle_anchor`. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn set_trial_end(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().trial_end = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trial_from_plan`.\nIndicates if a plan's `trial_period_days` should be applied to the subscription. Setting `trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn set_trial_from_plan(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().trial_from_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trial_period_days`.\nInteger representing the number of trial period days before the customer is charged for the first time. This will always overwrite any trials that might apply via a subscribed plan. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn set_trial_period_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().trial_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `add_invoice_items`.\n"]
    pub fn set_add_invoice_items(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Subscription_AddInvoiceItems>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().add_invoice_items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.add_invoice_items = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(self, v: impl std::convert::Into<terrars::BlockAssignable<Subscription_Items>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the subscription."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_percent` after provisioning.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. The request must be made by a platform account on a connected account in order to set an application fee percentage. For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions)."]
    pub fn application_fee_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_percent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\nWhether Stripe automatically computes tax on this subscription."]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backdate_start_date` after provisioning.\nFor new subscriptions, a past timestamp to backdate the subscription's start date to. If set, the first invoice will contain a proration for the timespan between the start date and the current time. Can be combined with trials and the billing cycle anchor."]
    pub fn backdate_start_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backdate_start_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_cycle_anchor` after provisioning.\nA future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format."]
    pub fn billing_cycle_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_cycle_anchor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_thresholds_amount_gte` after provisioning.\nMonetary threshold that triggers the subscription to create an invoice"]
    pub fn billing_thresholds_amount_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_thresholds_amount_gte", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `billing_thresholds_reset_billing_cycle_anchor` after provisioning.\nIndicates if the `billing_cycle_anchor` should be reset when a threshold is reached. If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged. This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`."]
    pub fn billing_thresholds_reset_billing_cycle_anchor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_thresholds_reset_billing_cycle_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cancel_at` after provisioning.\nA timestamp at which the subscription should cancel. If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`. If set during a future period, this will always cause a proration for that period."]
    pub fn cancel_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cancel_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cancel_at_period_end` after provisioning.\nBoolean indicating whether this subscription should cancel at the end of the current period."]
    pub fn cancel_at_period_end(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cancel_at_period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `canceled_at` after provisioning.\nIf the subscription has been canceled, the date of that cancellation. If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will reflect the time of the most recent update request, not the end of the subscription period when the subscription is automatically moved to a canceled state."]
    pub fn canceled_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canceled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_method` after provisioning.\nEither `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`. Defaults to `charge_automatically`."]
    pub fn collection_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `coupon` after provisioning.\nThe ID of the coupon to apply to this subscription. A coupon applied to a subscription will only affect invoices created for that particular subscription."]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
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
        "Get a reference to the value of field `current_period_end` after provisioning.\nEnd of the current period that the subscription has been invoiced for. At the end of this period, a new invoice will be created."]
    pub fn current_period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_period_start` after provisioning.\nStart of the current period that the subscription has been invoiced for."]
    pub fn current_period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe identifier of the customer to subscribe."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `days_until_due` after provisioning.\nNumber of days a customer has to pay invoices generated by this subscription. Valid only for subscriptions where `collection_method` is set to `send_invoice`."]
    pub fn days_until_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_until_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_payment_method` after provisioning.\nID of the default payment method for the subscription. It must belong to the customer associated with the subscription. This takes precedence over `default_source`. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source)."]
    pub fn default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_payment_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_source` after provisioning.\nID of the default payment source for the subscription. It must belong to the customer associated with the subscription and be in a chargeable state. If `default_payment_method` is also set, `default_payment_method` will take precedence. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source)."]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_tax_rates` after provisioning.\nThe tax rates that will apply to any subscription item that does not have `tax_rates` set. Invoices created will have their `default_tax_rates` populated from the subscription."]
    pub fn default_tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_tax_rates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_checkout_session` after provisioning.\nThe Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode."]
    pub fn discount_checkout_session(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_checkout_session", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_amount_off` after provisioning.\nAmount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    pub fn discount_coupon_amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_amount_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_applies_to_products` after provisioning.\nA list of product IDs this coupon applies to"]
    pub fn discount_coupon_applies_to_products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.discount_coupon_applies_to_products", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn discount_coupon_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_currency` after provisioning.\nIf `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off."]
    pub fn discount_coupon_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_currency_options` after provisioning.\nCoupons defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn discount_coupon_currency_options(&self) -> terrars::ListRef<Subscription_DiscountCouponCurrencyOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.discount_coupon_currency_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_duration` after provisioning.\nOne of `forever`, `once`, and `repeating`. Describes how long a customer who applies this coupon will get the discount."]
    pub fn discount_coupon_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_duration_in_months` after provisioning.\nIf `duration` is `repeating`, the number of months the coupon applies. Null if coupon `duration` is `forever` or `once`."]
    pub fn discount_coupon_duration_in_months(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.discount_coupon_duration_in_months", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_id` after provisioning.\nUnique identifier for the object."]
    pub fn discount_coupon_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn discount_coupon_livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_max_redemptions` after provisioning.\nMaximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid."]
    pub fn discount_coupon_max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.discount_coupon_max_redemptions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_metadata` after provisioning.\nSet of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format."]
    pub fn discount_coupon_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.discount_coupon_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_name` after provisioning.\nName of the coupon displayed to customers on for instance invoices or receipts."]
    pub fn discount_coupon_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn discount_coupon_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_percent_off` after provisioning.\nPercent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon. For example, a coupon with percent_off of 50 will make a %s100 invoice %s50 instead."]
    pub fn discount_coupon_percent_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_percent_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_redeem_by` after provisioning.\nDate after which the coupon can no longer be redeemed."]
    pub fn discount_coupon_redeem_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_redeem_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_times_redeemed` after provisioning.\nNumber of times this coupon has been applied to a customer."]
    pub fn discount_coupon_times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.discount_coupon_times_redeemed", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_valid` after provisioning.\nTaking account of the above properties, whether this coupon can still be applied to a customer."]
    pub fn discount_coupon_valid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_valid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_customer` after provisioning.\nThe ID of the customer associated with this discount."]
    pub fn discount_customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_end` after provisioning.\nIf the coupon has a duration of `repeating`, the date that this discount will end. If the coupon has a duration of `once` or `forever`, this attribute will be null."]
    pub fn discount_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_id` after provisioning.\nThe ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array."]
    pub fn discount_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_invoice` after provisioning.\nThe invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice."]
    pub fn discount_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_invoice", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_invoice_item` after provisioning.\nThe invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item."]
    pub fn discount_invoice_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_invoice_item", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn discount_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_promotion_code` after provisioning.\nThe promotion code applied to create this discount."]
    pub fn discount_promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_promotion_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_start` after provisioning.\nDate that the coupon was applied."]
    pub fn discount_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_subscription` after provisioning.\nThe subscription that this coupon is applied to, if it is applied to a particular subscription."]
    pub fn discount_subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_subscription", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ended_at` after provisioning.\nIf the subscription has ended, the date the subscription ended."]
    pub fn ended_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ended_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_invoice` after provisioning.\nThe most recent invoice this subscription has generated."]
    pub fn latest_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_invoice", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_pending_invoice_item_invoice` after provisioning.\nSpecifies the approximate timestamp on which any pending invoice items will be billed according to the schedule provided at `pending_invoice_item_interval`."]
    pub fn next_pending_invoice_item_invoice(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_pending_invoice_item_invoice", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `off_session` after provisioning.\nIndicates if a customer is on or off-session while an invoice payment is attempted."]
    pub fn off_session(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.off_session", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_behalf_of` after provisioning.\nThe account on behalf of which to charge, for each of the subscription's invoices."]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pause_collection_behavior` after provisioning.\nThe payment collection behavior for this subscription while paused. One of `keep_as_draft`, `mark_uncollectible`, or `void`."]
    pub fn pause_collection_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_collection_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pause_collection_resumes_at` after provisioning.\nThe time after which the subscription will resume collecting payments."]
    pub fn pause_collection_resumes_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_collection_resumes_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_behavior` after provisioning.\nOnly applies to subscriptions with `collection_method=charge_automatically`.\n\nUse `allow_incomplete` to create subscriptions with `status=incomplete` if the first invoice cannot be paid. Creating subscriptions with this status allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice. For example, SCA regulation may require 3DS authentication to complete payment. See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more. This is the default behavior.\n\nUse `default_incomplete` to create Subscriptions with `status=incomplete` when the first invoice requires payment, otherwise start as active. Subscriptions transition to `status=active` when successfully confirming the payment intent on the first invoice. This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice. Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method. If the payment intent is not confirmed within 23 hours subscriptions transition to `status=incomplete_expired`, which is a terminal state.\n\nUse `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's first invoice cannot be paid. For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not create a subscription and returns an error instead. This was the default behavior for API versions prior to 2019-03-14. See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.\n\n`pending_if_incomplete` is only used with updates and cannot be passed when creating a subscription.\n\nSubscriptions with `collection_method=send_invoice` are automatically activated regardless of the first invoice status."]
    pub fn payment_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type` after provisioning.\nTransaction type of the mandate."]
    pub fn payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_acss_debit_verification_method` after provisioning.\nBank account verification method."]
    pub fn payment_settings_payment_method_options_acss_debit_verification_method(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_acss_debit_verification_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_bancontact_preferred_language` after provisioning.\nPreferred language of the Bancontact authorization page that the customer is redirected to."]
    pub fn payment_settings_payment_method_options_bancontact_preferred_language(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_bancontact_preferred_language", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_amount` after provisioning.\nAmount to be charged for future payments."]
    pub fn payment_settings_payment_method_options_card_mandate_options_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_amount", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_amount_type` after provisioning.\nOne of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param."]
    pub fn payment_settings_payment_method_options_card_mandate_options_amount_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_amount_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_description` after provisioning.\nA description of the mandate or subscription that is meant to be displayed to the customer."]
    pub fn payment_settings_payment_method_options_card_mandate_options_description(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_description", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_network` after provisioning.\nSelected network to process this Subscription on. Depends on the available networks of the card attached to the Subscription. Can be only set confirm-time."]
    pub fn payment_settings_payment_method_options_card_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_network", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_request_three_d_secure` after provisioning.\nWe strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine."]
    pub fn payment_settings_payment_method_options_card_request_three_d_secure(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_request_three_d_secure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country` after provisioning.\nThe desired country code of the bank account information. Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`."]
    pub fn payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_bank_transfer_type` after provisioning.\nThe bank transfer type that can be used for funding. Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`."]
    pub fn payment_settings_payment_method_options_customer_balance_bank_transfer_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_customer_balance_bank_transfer_type",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_funding_type` after provisioning.\nThe funding method type to be used when there are not enough funds in the customer balance. Permitted values include: `bank_transfer`."]
    pub fn payment_settings_payment_method_options_customer_balance_funding_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_customer_balance_funding_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_us_bank_account_financial_connections_permissions` after provisioning.\nThe list of permissions to request. The `payment_method` permission must be included."]
    pub fn payment_settings_payment_method_options_us_bank_account_financial_connections_permissions(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_us_bank_account_financial_connections_permissions",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_us_bank_account_verification_method` after provisioning.\nBank account verification method."]
    pub fn payment_settings_payment_method_options_us_bank_account_verification_method(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_us_bank_account_verification_method",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_types` after provisioning.\nThe list of payment method types to provide to every invoice created by the subscription. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice)."]
    pub fn payment_settings_payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_save_default_payment_method` after provisioning.\nEither `off`, or `on_subscription`. With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds."]
    pub fn payment_settings_save_default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_save_default_payment_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_item_interval_interval` after provisioning.\nSpecifies invoicing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn pending_invoice_item_interval_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_item_interval_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_item_interval_interval_count` after provisioning.\nThe number of intervals between invoices. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn pending_invoice_item_interval_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_item_interval_interval_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_setup_intent` after provisioning.\nYou can use this [SetupIntent](https://stripe.com/docs/api/setup_intents) to collect user authentication when creating a subscription without immediate payment or updating a subscription's payment method, allowing you to optimize for off-session payments. Learn more in the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication#scenario-2)."]
    pub fn pending_setup_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_setup_intent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pending_update_billing_cycle_anchor` after provisioning.\nIf the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format."]
    pub fn pending_update_billing_cycle_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_update_billing_cycle_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_update_expires_at` after provisioning.\nThe point after which the changes reflected by this update will be discarded and no longer applied."]
    pub fn pending_update_expires_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_expires_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pending_update_subscription_items` after provisioning.\nList of subscription items, each with an attached plan, that will be set if the update is applied."]
    pub fn pending_update_subscription_items(&self) -> terrars::ListRef<Subscription_PendingUpdateSubscriptionItemsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.pending_update_subscription_items", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_update_trial_end` after provisioning.\nUnix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied."]
    pub fn pending_update_trial_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_trial_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pending_update_trial_from_plan` after provisioning.\nIndicates if a plan's `trial_period_days` should be applied to the subscription. Setting `trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn pending_update_trial_from_plan(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_update_trial_from_plan", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `promotion_code` after provisioning.\nThe API ID of a promotion code to apply to this subscription. A promotion code applied to a subscription will only affect invoices created for that particular subscription."]
    pub fn promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promotion_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proration_behavior` after provisioning.\nDetermines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`. If no value is passed, the default is `create_prorations`."]
    pub fn proration_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proration_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nThe schedule attached to the subscription"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_date` after provisioning.\nDate when the subscription was first created. The date might differ from the `created` date due to backdating."]
    pub fn start_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nPossible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`. \n\nFor `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails. A subscription in this state can only have metadata and default_source updated. Once the first invoice is paid, the subscription moves into an `active` state. If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`. This is a terminal state, the open invoice will be voided and no further invoices will be generated. \n\nA subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over. \n\nIf subscription `collection_method=charge_automatically` it becomes `past_due` when payment to renew it fails and `canceled` or `unpaid` (depending on your subscriptions settings) when Stripe has exhausted all payment retry attempts. \n\nIf subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that. Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed). After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this subscription belongs to."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_amount_percent` after provisioning.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account. By default, the entire amount is transferred to the destination."]
    pub fn transfer_data_amount_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount_percent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_destination` after provisioning.\nThe account where funds from the payment will be transferred to upon payment success."]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_end` after provisioning.\nUnix timestamp representing the end of the trial period the customer will get before being charged for the first time. This will always overwrite any trials that might apply via a subscribed plan. If set, trial_end will override the default trial period of the plan the customer is being subscribed to. The special value `now` can be provided to end the customer's trial immediately. Can be at most two years from `billing_cycle_anchor`. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn trial_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_from_plan` after provisioning.\nIndicates if a plan's `trial_period_days` should be applied to the subscription. Setting `trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn trial_from_plan(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_from_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_period_days` after provisioning.\nInteger representing the number of trial period days before the customer is charged for the first time. This will always overwrite any trials that might apply via a subscribed plan. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn trial_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_start` after provisioning.\nIf the subscription has a trial, the beginning of that trial."]
    pub fn trial_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `add_invoice_items` after provisioning.\n"]
    pub fn add_invoice_items(&self) -> terrars::ListRef<Subscription_AddInvoiceItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_invoice_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<Subscription_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }
}

impl terrars::Referable for Subscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Subscription { }

impl terrars::ToListMappable for Subscription {
    type O = terrars::ListRef<SubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Subscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSubscription {
    pub tf_id: std::string::String,
    #[doc = "The identifier of the customer to subscribe."]
    pub customer: terrars::PrimField<std::string::String>,
}

impl BuildSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> Subscription {
        let out = Subscription(std::rc::Rc::new(Subscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_fee_percent: core::default::Default::default(),
                automatic_tax_enabled: core::default::Default::default(),
                backdate_start_date: core::default::Default::default(),
                billing_cycle_anchor: core::default::Default::default(),
                billing_thresholds_amount_gte: core::default::Default::default(),
                billing_thresholds_reset_billing_cycle_anchor: core::default::Default::default(),
                cancel_at: core::default::Default::default(),
                cancel_at_period_end: core::default::Default::default(),
                collection_method: core::default::Default::default(),
                coupon: core::default::Default::default(),
                currency: core::default::Default::default(),
                customer: self.customer,
                days_until_due: core::default::Default::default(),
                default_payment_method: core::default::Default::default(),
                default_source: core::default::Default::default(),
                default_tax_rates: core::default::Default::default(),
                description: core::default::Default::default(),
                off_session: core::default::Default::default(),
                on_behalf_of: core::default::Default::default(),
                payment_behavior: core::default::Default::default(),
                payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_acss_debit_verification_method: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_bancontact_preferred_language: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_card_mandate_options_amount: core::default::Default::default(),
                payment_settings_payment_method_options_card_mandate_options_amount_type: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_card_mandate_options_description: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_card_network: core::default::Default::default(),
                payment_settings_payment_method_options_card_request_three_d_secure: core::default::Default::default(),
                payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_customer_balance_bank_transfer_type: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_customer_balance_funding_type: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_us_bank_account_financial_connections_permissions: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_us_bank_account_verification_method: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_types: core::default::Default::default(),
                payment_settings_save_default_payment_method: core::default::Default::default(),
                pending_invoice_item_interval_interval: core::default::Default::default(),
                pending_invoice_item_interval_interval_count: core::default::Default::default(),
                promotion_code: core::default::Default::default(),
                proration_behavior: core::default::Default::default(),
                transfer_data_amount_percent: core::default::Default::default(),
                transfer_data_destination: core::default::Default::default(),
                trial_end: core::default::Default::default(),
                trial_from_plan: core::default::Default::default(),
                trial_period_days: core::default::Default::default(),
                add_invoice_items: core::default::Default::default(),
                items: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the subscription."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_percent` after provisioning.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. The request must be made by a platform account on a connected account in order to set an application fee percentage. For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions)."]
    pub fn application_fee_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_percent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\nWhether Stripe automatically computes tax on this subscription."]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `backdate_start_date` after provisioning.\nFor new subscriptions, a past timestamp to backdate the subscription's start date to. If set, the first invoice will contain a proration for the timespan between the start date and the current time. Can be combined with trials and the billing cycle anchor."]
    pub fn backdate_start_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backdate_start_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_cycle_anchor` after provisioning.\nA future timestamp to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle). This is used to determine the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format."]
    pub fn billing_cycle_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_cycle_anchor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_thresholds_amount_gte` after provisioning.\nMonetary threshold that triggers the subscription to create an invoice"]
    pub fn billing_thresholds_amount_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_thresholds_amount_gte", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `billing_thresholds_reset_billing_cycle_anchor` after provisioning.\nIndicates if the `billing_cycle_anchor` should be reset when a threshold is reached. If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged. This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`."]
    pub fn billing_thresholds_reset_billing_cycle_anchor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_thresholds_reset_billing_cycle_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `cancel_at` after provisioning.\nA timestamp at which the subscription should cancel. If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`. If set during a future period, this will always cause a proration for that period."]
    pub fn cancel_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cancel_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cancel_at_period_end` after provisioning.\nBoolean indicating whether this subscription should cancel at the end of the current period."]
    pub fn cancel_at_period_end(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cancel_at_period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `canceled_at` after provisioning.\nIf the subscription has been canceled, the date of that cancellation. If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will reflect the time of the most recent update request, not the end of the subscription period when the subscription is automatically moved to a canceled state."]
    pub fn canceled_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canceled_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_method` after provisioning.\nEither `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer. When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`. Defaults to `charge_automatically`."]
    pub fn collection_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `coupon` after provisioning.\nThe ID of the coupon to apply to this subscription. A coupon applied to a subscription will only affect invoices created for that particular subscription."]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
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
        "Get a reference to the value of field `current_period_end` after provisioning.\nEnd of the current period that the subscription has been invoiced for. At the end of this period, a new invoice will be created."]
    pub fn current_period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_period_start` after provisioning.\nStart of the current period that the subscription has been invoiced for."]
    pub fn current_period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe identifier of the customer to subscribe."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `days_until_due` after provisioning.\nNumber of days a customer has to pay invoices generated by this subscription. Valid only for subscriptions where `collection_method` is set to `send_invoice`."]
    pub fn days_until_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_until_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_payment_method` after provisioning.\nID of the default payment method for the subscription. It must belong to the customer associated with the subscription. This takes precedence over `default_source`. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source)."]
    pub fn default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_payment_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_source` after provisioning.\nID of the default payment source for the subscription. It must belong to the customer associated with the subscription and be in a chargeable state. If `default_payment_method` is also set, `default_payment_method` will take precedence. If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source)."]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_tax_rates` after provisioning.\nThe tax rates that will apply to any subscription item that does not have `tax_rates` set. Invoices created will have their `default_tax_rates` populated from the subscription."]
    pub fn default_tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_tax_rates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_checkout_session` after provisioning.\nThe Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode."]
    pub fn discount_checkout_session(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_checkout_session", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_amount_off` after provisioning.\nAmount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer."]
    pub fn discount_coupon_amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_amount_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_applies_to_products` after provisioning.\nA list of product IDs this coupon applies to"]
    pub fn discount_coupon_applies_to_products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.discount_coupon_applies_to_products", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn discount_coupon_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_currency` after provisioning.\nIf `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off."]
    pub fn discount_coupon_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_currency_options` after provisioning.\nCoupons defined in each available currency option. Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn discount_coupon_currency_options(&self) -> terrars::ListRef<Subscription_DiscountCouponCurrencyOptionsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.discount_coupon_currency_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_duration` after provisioning.\nOne of `forever`, `once`, and `repeating`. Describes how long a customer who applies this coupon will get the discount."]
    pub fn discount_coupon_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_duration_in_months` after provisioning.\nIf `duration` is `repeating`, the number of months the coupon applies. Null if coupon `duration` is `forever` or `once`."]
    pub fn discount_coupon_duration_in_months(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.discount_coupon_duration_in_months", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_id` after provisioning.\nUnique identifier for the object."]
    pub fn discount_coupon_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn discount_coupon_livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_max_redemptions` after provisioning.\nMaximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid."]
    pub fn discount_coupon_max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.discount_coupon_max_redemptions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_metadata` after provisioning.\nSet of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format."]
    pub fn discount_coupon_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.discount_coupon_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_name` after provisioning.\nName of the coupon displayed to customers on for instance invoices or receipts."]
    pub fn discount_coupon_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn discount_coupon_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_percent_off` after provisioning.\nPercent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon. For example, a coupon with percent_off of 50 will make a %s100 invoice %s50 instead."]
    pub fn discount_coupon_percent_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_percent_off", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_redeem_by` after provisioning.\nDate after which the coupon can no longer be redeemed."]
    pub fn discount_coupon_redeem_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_redeem_by", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_times_redeemed` after provisioning.\nNumber of times this coupon has been applied to a customer."]
    pub fn discount_coupon_times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.discount_coupon_times_redeemed", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `discount_coupon_valid` after provisioning.\nTaking account of the above properties, whether this coupon can still be applied to a customer."]
    pub fn discount_coupon_valid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_valid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_customer` after provisioning.\nThe ID of the customer associated with this discount."]
    pub fn discount_customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_end` after provisioning.\nIf the coupon has a duration of `repeating`, the date that this discount will end. If the coupon has a duration of `once` or `forever`, this attribute will be null."]
    pub fn discount_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_id` after provisioning.\nThe ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array."]
    pub fn discount_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_invoice` after provisioning.\nThe invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice."]
    pub fn discount_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_invoice", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_invoice_item` after provisioning.\nThe invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item."]
    pub fn discount_invoice_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_invoice_item", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn discount_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_promotion_code` after provisioning.\nThe promotion code applied to create this discount."]
    pub fn discount_promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_promotion_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_start` after provisioning.\nDate that the coupon was applied."]
    pub fn discount_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discount_subscription` after provisioning.\nThe subscription that this coupon is applied to, if it is applied to a particular subscription."]
    pub fn discount_subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_subscription", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ended_at` after provisioning.\nIf the subscription has ended, the date the subscription ended."]
    pub fn ended_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ended_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_invoice` after provisioning.\nThe most recent invoice this subscription has generated."]
    pub fn latest_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_invoice", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_pending_invoice_item_invoice` after provisioning.\nSpecifies the approximate timestamp on which any pending invoice items will be billed according to the schedule provided at `pending_invoice_item_interval`."]
    pub fn next_pending_invoice_item_invoice(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.next_pending_invoice_item_invoice", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `off_session` after provisioning.\nIndicates if a customer is on or off-session while an invoice payment is attempted."]
    pub fn off_session(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.off_session", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_behalf_of` after provisioning.\nThe account on behalf of which to charge, for each of the subscription's invoices."]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pause_collection_behavior` after provisioning.\nThe payment collection behavior for this subscription while paused. One of `keep_as_draft`, `mark_uncollectible`, or `void`."]
    pub fn pause_collection_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_collection_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pause_collection_resumes_at` after provisioning.\nThe time after which the subscription will resume collecting payments."]
    pub fn pause_collection_resumes_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_collection_resumes_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_behavior` after provisioning.\nOnly applies to subscriptions with `collection_method=charge_automatically`.\n\nUse `allow_incomplete` to create subscriptions with `status=incomplete` if the first invoice cannot be paid. Creating subscriptions with this status allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice. For example, SCA regulation may require 3DS authentication to complete payment. See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more. This is the default behavior.\n\nUse `default_incomplete` to create Subscriptions with `status=incomplete` when the first invoice requires payment, otherwise start as active. Subscriptions transition to `status=active` when successfully confirming the payment intent on the first invoice. This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice. Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method. If the payment intent is not confirmed within 23 hours subscriptions transition to `status=incomplete_expired`, which is a terminal state.\n\nUse `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's first invoice cannot be paid. For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not create a subscription and returns an error instead. This was the default behavior for API versions prior to 2019-03-14. See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.\n\n`pending_if_incomplete` is only used with updates and cannot be passed when creating a subscription.\n\nSubscriptions with `collection_method=send_invoice` are automatically activated regardless of the first invoice status."]
    pub fn payment_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type` after provisioning.\nTransaction type of the mandate."]
    pub fn payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_acss_debit_verification_method` after provisioning.\nBank account verification method."]
    pub fn payment_settings_payment_method_options_acss_debit_verification_method(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_acss_debit_verification_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_bancontact_preferred_language` after provisioning.\nPreferred language of the Bancontact authorization page that the customer is redirected to."]
    pub fn payment_settings_payment_method_options_bancontact_preferred_language(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_bancontact_preferred_language", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_amount` after provisioning.\nAmount to be charged for future payments."]
    pub fn payment_settings_payment_method_options_card_mandate_options_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_amount", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_amount_type` after provisioning.\nOne of `fixed` or `maximum`. If `fixed`, the `amount` param refers to the exact amount to be charged in future payments. If `maximum`, the amount charged can be up to the value passed for the `amount` param."]
    pub fn payment_settings_payment_method_options_card_mandate_options_amount_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_amount_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_description` after provisioning.\nA description of the mandate or subscription that is meant to be displayed to the customer."]
    pub fn payment_settings_payment_method_options_card_mandate_options_description(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_description", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_network` after provisioning.\nSelected network to process this Subscription on. Depends on the available networks of the card attached to the Subscription. Can be only set confirm-time."]
    pub fn payment_settings_payment_method_options_card_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_network", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_request_three_d_secure` after provisioning.\nWe strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication). However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option. Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine."]
    pub fn payment_settings_payment_method_options_card_request_three_d_secure(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_request_three_d_secure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country` after provisioning.\nThe desired country code of the bank account information. Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`."]
    pub fn payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_bank_transfer_type` after provisioning.\nThe bank transfer type that can be used for funding. Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`."]
    pub fn payment_settings_payment_method_options_customer_balance_bank_transfer_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_customer_balance_bank_transfer_type",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_funding_type` after provisioning.\nThe funding method type to be used when there are not enough funds in the customer balance. Permitted values include: `bank_transfer`."]
    pub fn payment_settings_payment_method_options_customer_balance_funding_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_customer_balance_funding_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_us_bank_account_financial_connections_permissions` after provisioning.\nThe list of permissions to request. The `payment_method` permission must be included."]
    pub fn payment_settings_payment_method_options_us_bank_account_financial_connections_permissions(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_us_bank_account_financial_connections_permissions",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_us_bank_account_verification_method` after provisioning.\nBank account verification method."]
    pub fn payment_settings_payment_method_options_us_bank_account_verification_method(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_us_bank_account_verification_method",
                self.extract_ref()
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_types` after provisioning.\nThe list of payment method types to provide to every invoice created by the subscription. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice)."]
    pub fn payment_settings_payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_save_default_payment_method` after provisioning.\nEither `off`, or `on_subscription`. With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds."]
    pub fn payment_settings_save_default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_save_default_payment_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_item_interval_interval` after provisioning.\nSpecifies invoicing frequency. Either `day`, `week`, `month` or `year`."]
    pub fn pending_invoice_item_interval_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_item_interval_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_item_interval_interval_count` after provisioning.\nThe number of intervals between invoices. For example, `interval=month` and `interval_count=3` bills every 3 months. Maximum of one year interval allowed (1 year, 12 months, or 52 weeks)."]
    pub fn pending_invoice_item_interval_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_item_interval_interval_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_setup_intent` after provisioning.\nYou can use this [SetupIntent](https://stripe.com/docs/api/setup_intents) to collect user authentication when creating a subscription without immediate payment or updating a subscription's payment method, allowing you to optimize for off-session payments. Learn more in the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication#scenario-2)."]
    pub fn pending_setup_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_setup_intent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pending_update_billing_cycle_anchor` after provisioning.\nIf the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices. The timestamp is in UTC format."]
    pub fn pending_update_billing_cycle_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_update_billing_cycle_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_update_expires_at` after provisioning.\nThe point after which the changes reflected by this update will be discarded and no longer applied."]
    pub fn pending_update_expires_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_expires_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pending_update_subscription_items` after provisioning.\nList of subscription items, each with an attached plan, that will be set if the update is applied."]
    pub fn pending_update_subscription_items(&self) -> terrars::ListRef<Subscription_PendingUpdateSubscriptionItemsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.pending_update_subscription_items", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_update_trial_end` after provisioning.\nUnix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied."]
    pub fn pending_update_trial_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_trial_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pending_update_trial_from_plan` after provisioning.\nIndicates if a plan's `trial_period_days` should be applied to the subscription. Setting `trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn pending_update_trial_from_plan(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_update_trial_from_plan", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `promotion_code` after provisioning.\nThe API ID of a promotion code to apply to this subscription. A promotion code applied to a subscription will only affect invoices created for that particular subscription."]
    pub fn promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promotion_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proration_behavior` after provisioning.\nDetermines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`. If no value is passed, the default is `create_prorations`."]
    pub fn proration_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proration_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nThe schedule attached to the subscription"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `start_date` after provisioning.\nDate when the subscription was first created. The date might differ from the `created` date due to backdating."]
    pub fn start_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nPossible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`. \n\nFor `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails. A subscription in this state can only have metadata and default_source updated. Once the first invoice is paid, the subscription moves into an `active` state. If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`. This is a terminal state, the open invoice will be voided and no further invoices will be generated. \n\nA subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over. \n\nIf subscription `collection_method=charge_automatically` it becomes `past_due` when payment to renew it fails and `canceled` or `unpaid` (depending on your subscriptions settings) when Stripe has exhausted all payment retry attempts. \n\nIf subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that. Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed). After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this subscription belongs to."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_amount_percent` after provisioning.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account. By default, the entire amount is transferred to the destination."]
    pub fn transfer_data_amount_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount_percent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_destination` after provisioning.\nThe account where funds from the payment will be transferred to upon payment success."]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_end` after provisioning.\nUnix timestamp representing the end of the trial period the customer will get before being charged for the first time. This will always overwrite any trials that might apply via a subscribed plan. If set, trial_end will override the default trial period of the plan the customer is being subscribed to. The special value `now` can be provided to end the customer's trial immediately. Can be at most two years from `billing_cycle_anchor`. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn trial_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_from_plan` after provisioning.\nIndicates if a plan's `trial_period_days` should be applied to the subscription. Setting `trial_end` per subscription is preferred, and this defaults to `false`. Setting this flag to `true` together with `trial_end` is not allowed. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn trial_from_plan(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_from_plan", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_period_days` after provisioning.\nInteger representing the number of trial period days before the customer is charged for the first time. This will always overwrite any trials that might apply via a subscribed plan. See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more."]
    pub fn trial_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_period_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `trial_start` after provisioning.\nIf the subscription has a trial, the beginning of that trial."]
    pub fn trial_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `add_invoice_items` after provisioning.\n"]
    pub fn add_invoice_items(&self) -> terrars::ListRef<Subscription_AddInvoiceItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_invoice_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<Subscription_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Subscription_DiscountCouponCurrencyOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Subscription_DiscountCouponCurrencyOptions {
    #[doc = "Set the field `amount_off`.\n"]
    pub fn set_amount_off(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount_off = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_DiscountCouponCurrencyOptions {
    type O = terrars::BlockAssignable<Subscription_DiscountCouponCurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_DiscountCouponCurrencyOptions {}

impl BuildSubscription_DiscountCouponCurrencyOptions {
    pub fn build(self) -> Subscription_DiscountCouponCurrencyOptions {
        Subscription_DiscountCouponCurrencyOptions {
            amount_off: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_DiscountCouponCurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_DiscountCouponCurrencyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Subscription_DiscountCouponCurrencyOptionsRef {
        Subscription_DiscountCouponCurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_DiscountCouponCurrencyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount_off` after provisioning.\n"]
    pub fn amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_off", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    up_to: std::option::Option<terrars::PrimField<f64>>,
}

impl Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {
    #[doc = "Set the field `flat_amount`.\n"]
    pub fn set_flat_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.flat_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flat_amount_decimal`.\n"]
    pub fn set_flat_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flat_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount`.\n"]
    pub fn set_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount_decimal`.\n"]
    pub fn set_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `up_to`.\n"]
    pub fn set_up_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.up_to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {
    type O = terrars::BlockAssignable<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {}

impl BuildSubscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {
    pub fn build(self) -> Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {
        Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_TiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_TiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_TiersRef {
        Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_TiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_TiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flat_amount` after provisioning.\n"]
    pub fn flat_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flat_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `flat_amount_decimal` after provisioning.\n"]
    pub fn flat_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flat_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount` after provisioning.\n"]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount_decimal` after provisioning.\n"]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `up_to` after provisioning.\n"]
    pub fn up_to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.up_to", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_preset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers: std
    ::option
    ::Option<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {
    #[doc = "Set the field `custom_unit_amount_maximum`.\n"]
    pub fn set_custom_unit_amount_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_unit_amount_maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_unit_amount_minimum`.\n"]
    pub fn set_custom_unit_amount_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_unit_amount_minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_unit_amount_preset`.\n"]
    pub fn set_custom_unit_amount_preset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.custom_unit_amount_preset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_behavior`.\n"]
    pub fn set_tax_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tiers`.\n"]
    pub fn set_tiers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_Tiers>,
                    >,
    ) -> Self {
        self.tiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount`.\n"]
    pub fn set_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount_decimal`.\n"]
    pub fn set_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {
    type O = terrars::BlockAssignable<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {}

impl BuildSubscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {
    pub fn build(self) -> Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {
        Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions {
            custom_unit_amount_maximum: core::default::Default::default(),
            custom_unit_amount_minimum: core::default::Default::default(),
            custom_unit_amount_preset: core::default::Default::default(),
            key: core::default::Default::default(),
            tax_behavior: core::default::Default::default(),
            tiers: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptionsRef {
        Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_unit_amount_maximum` after provisioning.\n"]
    pub fn custom_unit_amount_maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_maximum", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_unit_amount_minimum` after provisioning.\n"]
    pub fn custom_unit_amount_minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_unit_amount_preset` after provisioning.\n"]
    pub fn custom_unit_amount_preset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_preset", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_behavior` after provisioning.\n"]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions_TiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount` after provisioning.\n"]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount_decimal` after provisioning.\n"]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_PendingUpdateSubscriptionItems_PriceTiers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    up_to: std::option::Option<terrars::PrimField<f64>>,
}

impl Subscription_PendingUpdateSubscriptionItems_PriceTiers {
    #[doc = "Set the field `flat_amount`.\n"]
    pub fn set_flat_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.flat_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flat_amount_decimal`.\n"]
    pub fn set_flat_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flat_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount`.\n"]
    pub fn set_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount_decimal`.\n"]
    pub fn set_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `up_to`.\n"]
    pub fn set_up_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.up_to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_PendingUpdateSubscriptionItems_PriceTiers {
    type O = terrars::BlockAssignable<Subscription_PendingUpdateSubscriptionItems_PriceTiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_PendingUpdateSubscriptionItems_PriceTiers {}

impl BuildSubscription_PendingUpdateSubscriptionItems_PriceTiers {
    pub fn build(self) -> Subscription_PendingUpdateSubscriptionItems_PriceTiers {
        Subscription_PendingUpdateSubscriptionItems_PriceTiers {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_PendingUpdateSubscriptionItems_PriceTiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_PendingUpdateSubscriptionItems_PriceTiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Subscription_PendingUpdateSubscriptionItems_PriceTiersRef {
        Subscription_PendingUpdateSubscriptionItems_PriceTiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_PendingUpdateSubscriptionItems_PriceTiersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flat_amount` after provisioning.\n"]
    pub fn flat_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flat_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `flat_amount_decimal` after provisioning.\n"]
    pub fn flat_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flat_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount` after provisioning.\n"]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount_decimal` after provisioning.\n"]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `up_to` after provisioning.\n"]
    pub fn up_to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.up_to", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_PendingUpdateSubscriptionItems_TaxRates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jurisdiction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Subscription_PendingUpdateSubscriptionItems_TaxRates {
    #[doc = "Set the field `active`.\n"]
    pub fn set_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusive`.\n"]
    pub fn set_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jurisdiction`.\n"]
    pub fn set_jurisdiction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jurisdiction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `livemode`.\n"]
    pub fn set_livemode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.livemode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\n"]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_type`.\n"]
    pub fn set_tax_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_PendingUpdateSubscriptionItems_TaxRates {
    type O = terrars::BlockAssignable<Subscription_PendingUpdateSubscriptionItems_TaxRates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_PendingUpdateSubscriptionItems_TaxRates {}

impl BuildSubscription_PendingUpdateSubscriptionItems_TaxRates {
    pub fn build(self) -> Subscription_PendingUpdateSubscriptionItems_TaxRates {
        Subscription_PendingUpdateSubscriptionItems_TaxRates {
            active: core::default::Default::default(),
            country: core::default::Default::default(),
            created: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            inclusive: core::default::Default::default(),
            jurisdiction: core::default::Default::default(),
            livemode: core::default::Default::default(),
            metadata: core::default::Default::default(),
            object: core::default::Default::default(),
            percentage: core::default::Default::default(),
            state: core::default::Default::default(),
            tax_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_PendingUpdateSubscriptionItems_TaxRatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_PendingUpdateSubscriptionItems_TaxRatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Subscription_PendingUpdateSubscriptionItems_TaxRatesRef {
        Subscription_PendingUpdateSubscriptionItems_TaxRatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_PendingUpdateSubscriptionItems_TaxRatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusive` after provisioning.\n"]
    pub fn inclusive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `jurisdiction` after provisioning.\n"]
    pub fn jurisdiction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.base))
    }

    #[doc = "Get a reference to the value of field `livemode` after provisioning.\n"]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `percentage` after provisioning.\n"]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_type` after provisioning.\n"]
    pub fn tax_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_type", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_PendingUpdateSubscriptionItems {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_thresholds_usage_gte: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_billing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_currency_options: std
    ::option
    ::Option<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_custom_unit_amount_maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_custom_unit_amount_minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_custom_unit_amount_preset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_lookup_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_nickname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_recurring_aggregate_usage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_recurring_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_recurring_interval_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_recurring_usage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_tiers: std::option::Option<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_PriceTiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_tiers_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_transform_quantity_divide_by: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_transform_quantity_round: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rates: std::option::Option<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_TaxRates>>,
}

impl Subscription_PendingUpdateSubscriptionItems {
    #[doc = "Set the field `billing_thresholds_usage_gte`.\n"]
    pub fn set_billing_thresholds_usage_gte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.billing_thresholds_usage_gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_active`.\n"]
    pub fn set_price_active(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.price_active = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_billing_scheme`.\n"]
    pub fn set_price_billing_scheme(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_billing_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_created`.\n"]
    pub fn set_price_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_currency`.\n"]
    pub fn set_price_currency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_currency_options`.\n"]
    pub fn set_price_currency_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptions>>,
    ) -> Self {
        self.price_currency_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_custom_unit_amount_maximum`.\n"]
    pub fn set_price_custom_unit_amount_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_custom_unit_amount_maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_custom_unit_amount_minimum`.\n"]
    pub fn set_price_custom_unit_amount_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_custom_unit_amount_minimum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_custom_unit_amount_preset`.\n"]
    pub fn set_price_custom_unit_amount_preset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_custom_unit_amount_preset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_id`.\n"]
    pub fn set_price_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_livemode`.\n"]
    pub fn set_price_livemode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.price_livemode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_lookup_key`.\n"]
    pub fn set_price_lookup_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_lookup_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_metadata`.\n"]
    pub fn set_price_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.price_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_nickname`.\n"]
    pub fn set_price_nickname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_nickname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_object`.\n"]
    pub fn set_price_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_product`.\n"]
    pub fn set_price_product(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_recurring_aggregate_usage`.\n"]
    pub fn set_price_recurring_aggregate_usage(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_recurring_aggregate_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_recurring_interval`.\n"]
    pub fn set_price_recurring_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_recurring_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_recurring_interval_count`.\n"]
    pub fn set_price_recurring_interval_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_recurring_interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_recurring_usage_type`.\n"]
    pub fn set_price_recurring_usage_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_recurring_usage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_tax_behavior`.\n"]
    pub fn set_price_tax_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_tiers`.\n"]
    pub fn set_price_tiers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_PriceTiers>>,
    ) -> Self {
        self.price_tiers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_tiers_mode`.\n"]
    pub fn set_price_tiers_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_tiers_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_transform_quantity_divide_by`.\n"]
    pub fn set_price_transform_quantity_divide_by(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.price_transform_quantity_divide_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_transform_quantity_round`.\n"]
    pub fn set_price_transform_quantity_round(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_transform_quantity_round = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_type`.\n"]
    pub fn set_price_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_unit_amount`.\n"]
    pub fn set_price_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_unit_amount_decimal`.\n"]
    pub fn set_price_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quantity`.\n"]
    pub fn set_quantity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.quantity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscription`.\n"]
    pub fn set_subscription(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_rates`.\n"]
    pub fn set_tax_rates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Subscription_PendingUpdateSubscriptionItems_TaxRates>>,
    ) -> Self {
        self.tax_rates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_PendingUpdateSubscriptionItems {
    type O = terrars::BlockAssignable<Subscription_PendingUpdateSubscriptionItems>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_PendingUpdateSubscriptionItems {}

impl BuildSubscription_PendingUpdateSubscriptionItems {
    pub fn build(self) -> Subscription_PendingUpdateSubscriptionItems {
        Subscription_PendingUpdateSubscriptionItems {
            billing_thresholds_usage_gte: core::default::Default::default(),
            created: core::default::Default::default(),
            id: core::default::Default::default(),
            metadata: core::default::Default::default(),
            object: core::default::Default::default(),
            price_active: core::default::Default::default(),
            price_billing_scheme: core::default::Default::default(),
            price_created: core::default::Default::default(),
            price_currency: core::default::Default::default(),
            price_currency_options: core::default::Default::default(),
            price_custom_unit_amount_maximum: core::default::Default::default(),
            price_custom_unit_amount_minimum: core::default::Default::default(),
            price_custom_unit_amount_preset: core::default::Default::default(),
            price_id: core::default::Default::default(),
            price_livemode: core::default::Default::default(),
            price_lookup_key: core::default::Default::default(),
            price_metadata: core::default::Default::default(),
            price_nickname: core::default::Default::default(),
            price_object: core::default::Default::default(),
            price_product: core::default::Default::default(),
            price_recurring_aggregate_usage: core::default::Default::default(),
            price_recurring_interval: core::default::Default::default(),
            price_recurring_interval_count: core::default::Default::default(),
            price_recurring_usage_type: core::default::Default::default(),
            price_tax_behavior: core::default::Default::default(),
            price_tiers: core::default::Default::default(),
            price_tiers_mode: core::default::Default::default(),
            price_transform_quantity_divide_by: core::default::Default::default(),
            price_transform_quantity_round: core::default::Default::default(),
            price_type: core::default::Default::default(),
            price_unit_amount: core::default::Default::default(),
            price_unit_amount_decimal: core::default::Default::default(),
            quantity: core::default::Default::default(),
            subscription: core::default::Default::default(),
            tax_rates: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_PendingUpdateSubscriptionItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_PendingUpdateSubscriptionItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Subscription_PendingUpdateSubscriptionItemsRef {
        Subscription_PendingUpdateSubscriptionItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_PendingUpdateSubscriptionItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `billing_thresholds_usage_gte` after provisioning.\n"]
    pub fn billing_thresholds_usage_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_thresholds_usage_gte", self.base))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `price_active` after provisioning.\n"]
    pub fn price_active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_active", self.base))
    }

    #[doc = "Get a reference to the value of field `price_billing_scheme` after provisioning.\n"]
    pub fn price_billing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_billing_scheme", self.base))
    }

    #[doc = "Get a reference to the value of field `price_created` after provisioning.\n"]
    pub fn price_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_created", self.base))
    }

    #[doc = "Get a reference to the value of field `price_currency` after provisioning.\n"]
    pub fn price_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_currency", self.base))
    }

    #[doc = "Get a reference to the value of field `price_currency_options` after provisioning.\n"]
    pub fn price_currency_options(
        &self,
    ) -> terrars::ListRef<Subscription_PendingUpdateSubscriptionItems_PriceCurrencyOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.price_currency_options", self.base))
    }

    #[doc = "Get a reference to the value of field `price_custom_unit_amount_maximum` after provisioning.\n"]
    pub fn price_custom_unit_amount_maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_custom_unit_amount_maximum", self.base))
    }

    #[doc = "Get a reference to the value of field `price_custom_unit_amount_minimum` after provisioning.\n"]
    pub fn price_custom_unit_amount_minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_custom_unit_amount_minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `price_custom_unit_amount_preset` after provisioning.\n"]
    pub fn price_custom_unit_amount_preset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_custom_unit_amount_preset", self.base))
    }

    #[doc = "Get a reference to the value of field `price_id` after provisioning.\n"]
    pub fn price_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_id", self.base))
    }

    #[doc = "Get a reference to the value of field `price_livemode` after provisioning.\n"]
    pub fn price_livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_livemode", self.base))
    }

    #[doc = "Get a reference to the value of field `price_lookup_key` after provisioning.\n"]
    pub fn price_lookup_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_lookup_key", self.base))
    }

    #[doc = "Get a reference to the value of field `price_metadata` after provisioning.\n"]
    pub fn price_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.price_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `price_nickname` after provisioning.\n"]
    pub fn price_nickname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_nickname", self.base))
    }

    #[doc = "Get a reference to the value of field `price_object` after provisioning.\n"]
    pub fn price_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_object", self.base))
    }

    #[doc = "Get a reference to the value of field `price_product` after provisioning.\n"]
    pub fn price_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_product", self.base))
    }

    #[doc = "Get a reference to the value of field `price_recurring_aggregate_usage` after provisioning.\n"]
    pub fn price_recurring_aggregate_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_recurring_aggregate_usage", self.base))
    }

    #[doc = "Get a reference to the value of field `price_recurring_interval` after provisioning.\n"]
    pub fn price_recurring_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_recurring_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `price_recurring_interval_count` after provisioning.\n"]
    pub fn price_recurring_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_recurring_interval_count", self.base))
    }

    #[doc = "Get a reference to the value of field `price_recurring_usage_type` after provisioning.\n"]
    pub fn price_recurring_usage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_recurring_usage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `price_tax_behavior` after provisioning.\n"]
    pub fn price_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_tax_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `price_tiers` after provisioning.\n"]
    pub fn price_tiers(&self) -> terrars::ListRef<Subscription_PendingUpdateSubscriptionItems_PriceTiersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.price_tiers", self.base))
    }

    #[doc = "Get a reference to the value of field `price_tiers_mode` after provisioning.\n"]
    pub fn price_tiers_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_tiers_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `price_transform_quantity_divide_by` after provisioning.\n"]
    pub fn price_transform_quantity_divide_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_transform_quantity_divide_by", self.base))
    }

    #[doc = "Get a reference to the value of field `price_transform_quantity_round` after provisioning.\n"]
    pub fn price_transform_quantity_round(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_transform_quantity_round", self.base))
    }

    #[doc = "Get a reference to the value of field `price_type` after provisioning.\n"]
    pub fn price_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_type", self.base))
    }

    #[doc = "Get a reference to the value of field `price_unit_amount` after provisioning.\n"]
    pub fn price_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `price_unit_amount_decimal` after provisioning.\n"]
    pub fn price_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription` after provisioning.\n"]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_rates` after provisioning.\n"]
    pub fn tax_rates(&self) -> terrars::ListRef<Subscription_PendingUpdateSubscriptionItems_TaxRatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_AddInvoiceItems {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Subscription_AddInvoiceItems {
    #[doc = "Set the field `price`.\n"]
    pub fn set_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_currency`.\n"]
    pub fn set_price_data_currency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_product`.\n"]
    pub fn set_price_data_product(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_tax_behavior`.\n"]
    pub fn set_price_data_tax_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_unit_amount`.\n"]
    pub fn set_price_data_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_data_unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_unit_amount_decimal`.\n"]
    pub fn set_price_data_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quantity`.\n"]
    pub fn set_quantity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.quantity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_rates`.\n"]
    pub fn set_tax_rates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tax_rates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_AddInvoiceItems {
    type O = terrars::BlockAssignable<Subscription_AddInvoiceItems>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_AddInvoiceItems {}

impl BuildSubscription_AddInvoiceItems {
    pub fn build(self) -> Subscription_AddInvoiceItems {
        Subscription_AddInvoiceItems {
            price: core::default::Default::default(),
            price_data_currency: core::default::Default::default(),
            price_data_product: core::default::Default::default(),
            price_data_tax_behavior: core::default::Default::default(),
            price_data_unit_amount: core::default::Default::default(),
            price_data_unit_amount_decimal: core::default::Default::default(),
            quantity: core::default::Default::default(),
            tax_rates: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_AddInvoiceItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_AddInvoiceItemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Subscription_AddInvoiceItemsRef {
        Subscription_AddInvoiceItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_AddInvoiceItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\n"]
    pub fn price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_currency` after provisioning.\n"]
    pub fn price_data_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_currency", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_product` after provisioning.\n"]
    pub fn price_data_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_product", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_tax_behavior` after provisioning.\n"]
    pub fn price_data_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_tax_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount` after provisioning.\n"]
    pub fn price_data_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn price_data_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_rates` after provisioning.\n"]
    pub fn tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.base))
    }
}

#[derive(Serialize)]
pub struct Subscription_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_thresholds_usage_gte: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_recurring_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_recurring_interval_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_data_unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Subscription_Items {
    #[doc = "Set the field `billing_thresholds_usage_gte`.\n"]
    pub fn set_billing_thresholds_usage_gte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.billing_thresholds_usage_gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price`.\n"]
    pub fn set_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_currency`.\n"]
    pub fn set_price_data_currency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_product`.\n"]
    pub fn set_price_data_product(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_recurring_interval`.\n"]
    pub fn set_price_data_recurring_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_recurring_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_recurring_interval_count`.\n"]
    pub fn set_price_data_recurring_interval_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.price_data_recurring_interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_tax_behavior`.\n"]
    pub fn set_price_data_tax_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_unit_amount`.\n"]
    pub fn set_price_data_unit_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_data_unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_unit_amount_decimal`.\n"]
    pub fn set_price_data_unit_amount_decimal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.price_data_unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quantity`.\n"]
    pub fn set_quantity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.quantity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_rates`.\n"]
    pub fn set_tax_rates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tax_rates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Subscription_Items {
    type O = terrars::BlockAssignable<Subscription_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubscription_Items {}

impl BuildSubscription_Items {
    pub fn build(self) -> Subscription_Items {
        Subscription_Items {
            billing_thresholds_usage_gte: core::default::Default::default(),
            metadata: core::default::Default::default(),
            price: core::default::Default::default(),
            price_data_currency: core::default::Default::default(),
            price_data_product: core::default::Default::default(),
            price_data_recurring_interval: core::default::Default::default(),
            price_data_recurring_interval_count: core::default::Default::default(),
            price_data_tax_behavior: core::default::Default::default(),
            price_data_unit_amount: core::default::Default::default(),
            price_data_unit_amount_decimal: core::default::Default::default(),
            quantity: core::default::Default::default(),
            tax_rates: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Subscription_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Subscription_ItemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Subscription_ItemsRef {
        Subscription_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Subscription_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `billing_thresholds_usage_gte` after provisioning.\n"]
    pub fn billing_thresholds_usage_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_thresholds_usage_gte", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\n"]
    pub fn price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_currency` after provisioning.\n"]
    pub fn price_data_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_currency", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_product` after provisioning.\n"]
    pub fn price_data_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_product", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_recurring_interval` after provisioning.\n"]
    pub fn price_data_recurring_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_recurring_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_recurring_interval_count` after provisioning.\n"]
    pub fn price_data_recurring_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_recurring_interval_count", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_tax_behavior` after provisioning.\n"]
    pub fn price_data_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_tax_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount` after provisioning.\n"]
    pub fn price_data_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn price_data_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_rates` after provisioning.\n"]
    pub fn tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.base))
    }
}

#[derive(Serialize, Default)]
struct SubscriptionDynamic {
    add_invoice_items: std::option::Option<terrars::DynamicBlock<Subscription_AddInvoiceItems>>,
    items: std::option::Option<terrars::DynamicBlock<Subscription_Items>>,
}
