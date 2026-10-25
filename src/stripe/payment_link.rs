#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct PaymentLinkData_ {
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
    after_completion_hosted_confirmation_custom_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_completion_redirect_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_completion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_promotion_codes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_fee_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_fee_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_tax_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_address_collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consent_collection_promotions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consent_collection_terms_of_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_text_shipping_address_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_text_submit_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_creation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_behalf_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_intent_data_capture_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_intent_data_setup_future_usage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_method_collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_method_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number_collection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_collection_allowed_countries: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    submit_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription_data_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription_data_trial_period_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_id_collection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    line_items: std::option::Option<std::vec::Vec<PaymentLink_LineItems>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_options: std::option::Option<std::vec::Vec<PaymentLink_ShippingOptions>>,
    dynamic: PaymentLinkDynamic,
}

struct PaymentLink_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PaymentLinkData_>,
}

#[derive(Clone)]
pub struct PaymentLink(std::rc::Rc<PaymentLink_>);

impl PaymentLink {
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
        "Set the field `after_completion_hosted_confirmation_custom_message`.\nThe custom message that is displayed to the customer after the purchase is complete."]
    pub fn set_after_completion_hosted_confirmation_custom_message(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().after_completion_hosted_confirmation_custom_message =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `after_completion_redirect_url`.\nThe URL the customer will be redirected to after the purchase is complete."]
    pub fn set_after_completion_redirect_url(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().after_completion_redirect_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `after_completion_type`.\nThe specified behavior after the purchase is complete."]
    pub fn set_after_completion_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().after_completion_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow_promotion_codes`.\nEnables user redeemable promotion codes."]
    pub fn set_allow_promotion_codes(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_promotion_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `application_fee_amount`.\nThe amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account. Can only be applied when there are no line items with recurring prices."]
    pub fn set_application_fee_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().application_fee_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `application_fee_percent`.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. There must be at least 1 line item with a recurring price to use this field."]
    pub fn set_application_fee_percent(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().application_fee_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `automatic_tax_enabled`.\nIf `true`, tax will be calculated automatically using the customer's location."]
    pub fn set_automatic_tax_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_tax_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_address_collection`.\nConfiguration for collecting the customer's billing address."]
    pub fn set_billing_address_collection(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().billing_address_collection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consent_collection_promotions`.\nIf set to `auto`, enables the collection of customer consent for promotional communications."]
    pub fn set_consent_collection_promotions(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().consent_collection_promotions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `consent_collection_terms_of_service`.\nIf set to `required`, it requires cutomers to accept the terms of service before being able to pay. If set to `none`, customers won't be shown a checkbox to accept the terms of service."]
    pub fn set_consent_collection_terms_of_service(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().consent_collection_terms_of_service = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `currency`.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price."]
    pub fn set_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_text_shipping_address_message`.\nText may be up to 500 characters in length."]
    pub fn set_custom_text_shipping_address_message(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_text_shipping_address_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_text_submit_message`.\nText may be up to 500 characters in length."]
    pub fn set_custom_text_submit_message(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_text_submit_message = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `customer_creation`.\nConfigures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers)."]
    pub fn set_customer_creation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_behalf_of`.\nThe account on behalf of which to charge."]
    pub fn set_on_behalf_of(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().on_behalf_of = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_intent_data_capture_method`.\nIndicates when the funds will be captured from the customer's account."]
    pub fn set_payment_intent_data_capture_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_intent_data_capture_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_intent_data_setup_future_usage`.\nIndicates that you intend to make future payments with the payment method collected during checkout."]
    pub fn set_payment_intent_data_setup_future_usage(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_intent_data_setup_future_usage = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_method_collection`.\nSpecify whether Checkout should collect a payment method. When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.\n\nCan only be set in `subscription` mode.\n\nIf you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials)."]
    pub fn set_payment_method_collection(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_method_collection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_method_types`.\nThe list of payment method types that customers can use. If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support))."]
    pub fn set_payment_method_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_method_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `phone_number_collection_enabled`.\nIf `true`, a phone number will be collected during checkout."]
    pub fn set_phone_number_collection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().phone_number_collection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shipping_address_collection_allowed_countries`.\nAn array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations. Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`."]
    pub fn set_shipping_address_collection_allowed_countries(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().shipping_address_collection_allowed_countries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `submit_type`.\nDescribes the type of transaction being performed in order to customize relevant text on the page, such as the submit button. Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`)."]
    pub fn set_submit_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().submit_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subscription_data_description`.\nThe subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription."]
    pub fn set_subscription_data_description(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().subscription_data_description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subscription_data_trial_period_days`.\nInteger representing the number of trial period days before the customer is charged for the first time."]
    pub fn set_subscription_data_trial_period_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().subscription_data_trial_period_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_id_collection_enabled`.\nIndicates whether tax ID collection is enabled for the session."]
    pub fn set_tax_id_collection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tax_id_collection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_data_amount`.\nThe amount in %s that will be transferred to the destination account. By default, the entire amount is transferred to the destination."]
    pub fn set_transfer_data_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transfer_data_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_data_destination`.\nThe connected account receiving the transfer."]
    pub fn set_transfer_data_destination(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().transfer_data_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `line_items`.\n"]
    pub fn set_line_items(self, v: impl std::convert::Into<terrars::BlockAssignable<PaymentLink_LineItems>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().line_items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.line_items = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shipping_options`.\n"]
    pub fn set_shipping_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PaymentLink_ShippingOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shipping_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shipping_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `after_completion_hosted_confirmation_custom_message` after provisioning.\nThe custom message that is displayed to the customer after the purchase is complete."]
    pub fn after_completion_hosted_confirmation_custom_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_completion_hosted_confirmation_custom_message", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `after_completion_redirect_url` after provisioning.\nThe URL the customer will be redirected to after the purchase is complete."]
    pub fn after_completion_redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_completion_redirect_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `after_completion_type` after provisioning.\nThe specified behavior after the purchase is complete."]
    pub fn after_completion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after_completion_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_promotion_codes` after provisioning.\nEnables user redeemable promotion codes."]
    pub fn allow_promotion_codes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_promotion_codes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_amount` after provisioning.\nThe amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account. Can only be applied when there are no line items with recurring prices."]
    pub fn application_fee_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_percent` after provisioning.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. There must be at least 1 line item with a recurring price to use this field."]
    pub fn application_fee_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_percent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\nIf `true`, tax will be calculated automatically using the customer's location."]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_address_collection` after provisioning.\nConfiguration for collecting the customer's billing address."]
    pub fn billing_address_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_address_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consent_collection_promotions` after provisioning.\nIf set to `auto`, enables the collection of customer consent for promotional communications."]
    pub fn consent_collection_promotions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.consent_collection_promotions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `consent_collection_terms_of_service` after provisioning.\nIf set to `required`, it requires cutomers to accept the terms of service before being able to pay. If set to `none`, customers won't be shown a checkbox to accept the terms of service."]
    pub fn consent_collection_terms_of_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.consent_collection_terms_of_service", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_text_shipping_address_message` after provisioning.\nText may be up to 500 characters in length."]
    pub fn custom_text_shipping_address_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_text_shipping_address_message", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_text_submit_message` after provisioning.\nText may be up to 500 characters in length."]
    pub fn custom_text_submit_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_text_submit_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_creation` after provisioning.\nConfigures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers)."]
    pub fn customer_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_creation", self.extract_ref()))
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
        "Get a reference to the value of field `on_behalf_of` after provisioning.\nThe account on behalf of which to charge."]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_intent_data_capture_method` after provisioning.\nIndicates when the funds will be captured from the customer's account."]
    pub fn payment_intent_data_capture_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_intent_data_capture_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_intent_data_setup_future_usage` after provisioning.\nIndicates that you intend to make future payments with the payment method collected during checkout."]
    pub fn payment_intent_data_setup_future_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_intent_data_setup_future_usage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_method_collection` after provisioning.\nSpecify whether Checkout should collect a payment method. When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.\n\nCan only be set in `subscription` mode.\n\nIf you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials)."]
    pub fn payment_method_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_method_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_method_types` after provisioning.\nThe list of payment method types that customers can use. If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support))."]
    pub fn payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payment_method_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phone_number_collection_enabled` after provisioning.\nIf `true`, a phone number will be collected during checkout."]
    pub fn phone_number_collection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number_collection_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_collection_allowed_countries` after provisioning.\nAn array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations. Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`."]
    pub fn shipping_address_collection_allowed_countries(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.shipping_address_collection_allowed_countries", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `submit_type` after provisioning.\nDescribes the type of transaction being performed in order to customize relevant text on the page, such as the submit button. Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`)."]
    pub fn submit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.submit_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_data_description` after provisioning.\nThe subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription."]
    pub fn subscription_data_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_data_description", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `subscription_data_trial_period_days` after provisioning.\nInteger representing the number of trial period days before the customer is charged for the first time."]
    pub fn subscription_data_trial_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_data_trial_period_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tax_id_collection_enabled` after provisioning.\nIndicates whether tax ID collection is enabled for the session."]
    pub fn tax_id_collection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_id_collection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_amount` after provisioning.\nThe amount in %s that will be transferred to the destination account. By default, the entire amount is transferred to the destination."]
    pub fn transfer_data_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_destination` after provisioning.\nThe connected account receiving the transfer."]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe public URL that can be shared with customers."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `line_items` after provisioning.\n"]
    pub fn line_items(&self) -> terrars::ListRef<PaymentLink_LineItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.line_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shipping_options` after provisioning.\n"]
    pub fn shipping_options(&self) -> terrars::ListRef<PaymentLink_ShippingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shipping_options", self.extract_ref()))
    }
}

impl terrars::Referable for PaymentLink {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PaymentLink { }

impl terrars::ToListMappable for PaymentLink {
    type O = terrars::ListRef<PaymentLinkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PaymentLink_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_payment_link".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPaymentLink {
    pub tf_id: std::string::String,
}

impl BuildPaymentLink {
    pub fn build(self, stack: &mut terrars::Stack) -> PaymentLink {
        let out = PaymentLink(std::rc::Rc::new(PaymentLink_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PaymentLinkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                after_completion_hosted_confirmation_custom_message: core::default::Default::default(),
                after_completion_redirect_url: core::default::Default::default(),
                after_completion_type: core::default::Default::default(),
                allow_promotion_codes: core::default::Default::default(),
                application_fee_amount: core::default::Default::default(),
                application_fee_percent: core::default::Default::default(),
                automatic_tax_enabled: core::default::Default::default(),
                billing_address_collection: core::default::Default::default(),
                consent_collection_promotions: core::default::Default::default(),
                consent_collection_terms_of_service: core::default::Default::default(),
                currency: core::default::Default::default(),
                custom_text_shipping_address_message: core::default::Default::default(),
                custom_text_submit_message: core::default::Default::default(),
                customer_creation: core::default::Default::default(),
                on_behalf_of: core::default::Default::default(),
                payment_intent_data_capture_method: core::default::Default::default(),
                payment_intent_data_setup_future_usage: core::default::Default::default(),
                payment_method_collection: core::default::Default::default(),
                payment_method_types: core::default::Default::default(),
                phone_number_collection_enabled: core::default::Default::default(),
                shipping_address_collection_allowed_countries: core::default::Default::default(),
                submit_type: core::default::Default::default(),
                subscription_data_description: core::default::Default::default(),
                subscription_data_trial_period_days: core::default::Default::default(),
                tax_id_collection_enabled: core::default::Default::default(),
                transfer_data_amount: core::default::Default::default(),
                transfer_data_destination: core::default::Default::default(),
                line_items: core::default::Default::default(),
                shipping_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PaymentLinkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentLinkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PaymentLinkRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `after_completion_hosted_confirmation_custom_message` after provisioning.\nThe custom message that is displayed to the customer after the purchase is complete."]
    pub fn after_completion_hosted_confirmation_custom_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_completion_hosted_confirmation_custom_message", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `after_completion_redirect_url` after provisioning.\nThe URL the customer will be redirected to after the purchase is complete."]
    pub fn after_completion_redirect_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.after_completion_redirect_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `after_completion_type` after provisioning.\nThe specified behavior after the purchase is complete."]
    pub fn after_completion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after_completion_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allow_promotion_codes` after provisioning.\nEnables user redeemable promotion codes."]
    pub fn allow_promotion_codes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_promotion_codes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_amount` after provisioning.\nThe amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account. Can only be applied when there are no line items with recurring prices."]
    pub fn application_fee_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_percent` after provisioning.\nA non-negative decimal between 0 and 100, with at most two decimal places. This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account. There must be at least 1 line item with a recurring price to use this field."]
    pub fn application_fee_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_percent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\nIf `true`, tax will be calculated automatically using the customer's location."]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_address_collection` after provisioning.\nConfiguration for collecting the customer's billing address."]
    pub fn billing_address_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_address_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `consent_collection_promotions` after provisioning.\nIf set to `auto`, enables the collection of customer consent for promotional communications."]
    pub fn consent_collection_promotions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.consent_collection_promotions", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `consent_collection_terms_of_service` after provisioning.\nIf set to `required`, it requires cutomers to accept the terms of service before being able to pay. If set to `none`, customers won't be shown a checkbox to accept the terms of service."]
    pub fn consent_collection_terms_of_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.consent_collection_terms_of_service", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_text_shipping_address_message` after provisioning.\nText may be up to 500 characters in length."]
    pub fn custom_text_shipping_address_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.custom_text_shipping_address_message", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `custom_text_submit_message` after provisioning.\nText may be up to 500 characters in length."]
    pub fn custom_text_submit_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_text_submit_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_creation` after provisioning.\nConfigures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers)."]
    pub fn customer_creation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_creation", self.extract_ref()))
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
        "Get a reference to the value of field `on_behalf_of` after provisioning.\nThe account on behalf of which to charge."]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_intent_data_capture_method` after provisioning.\nIndicates when the funds will be captured from the customer's account."]
    pub fn payment_intent_data_capture_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_intent_data_capture_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_intent_data_setup_future_usage` after provisioning.\nIndicates that you intend to make future payments with the payment method collected during checkout."]
    pub fn payment_intent_data_setup_future_usage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_intent_data_setup_future_usage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_method_collection` after provisioning.\nSpecify whether Checkout should collect a payment method. When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.\n\nCan only be set in `subscription` mode.\n\nIf you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials)."]
    pub fn payment_method_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_method_collection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_method_types` after provisioning.\nThe list of payment method types that customers can use. If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support))."]
    pub fn payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payment_method_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `phone_number_collection_enabled` after provisioning.\nIf `true`, a phone number will be collected during checkout."]
    pub fn phone_number_collection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.phone_number_collection_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_collection_allowed_countries` after provisioning.\nAn array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations. Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`."]
    pub fn shipping_address_collection_allowed_countries(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.shipping_address_collection_allowed_countries", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `submit_type` after provisioning.\nDescribes the type of transaction being performed in order to customize relevant text on the page, such as the submit button. Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`)."]
    pub fn submit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.submit_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_data_description` after provisioning.\nThe subscription's description, meant to be displayable to the customer. Use this field to optionally store an explanation of the subscription."]
    pub fn subscription_data_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_data_description", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `subscription_data_trial_period_days` after provisioning.\nInteger representing the number of trial period days before the customer is charged for the first time."]
    pub fn subscription_data_trial_period_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.subscription_data_trial_period_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tax_id_collection_enabled` after provisioning.\nIndicates whether tax ID collection is enabled for the session."]
    pub fn tax_id_collection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_id_collection_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_amount` after provisioning.\nThe amount in %s that will be transferred to the destination account. By default, the entire amount is transferred to the destination."]
    pub fn transfer_data_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_destination` after provisioning.\nThe connected account receiving the transfer."]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe public URL that can be shared with customers."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `line_items` after provisioning.\n"]
    pub fn line_items(&self) -> terrars::ListRef<PaymentLink_LineItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.line_items", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shipping_options` after provisioning.\n"]
    pub fn shipping_options(&self) -> terrars::ListRef<PaymentLink_ShippingOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shipping_options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PaymentLinkDynamic {
    line_items: std::option::Option<terrars::DynamicBlock<PaymentLink_LineItems>>,
    shipping_options: std::option::Option<terrars::DynamicBlock<PaymentLink_ShippingOptions>>,
}

#[derive(Serialize)]
pub struct PaymentLink_LineItems {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjustable_quantity_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjustable_quantity_maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjustable_quantity_minimum: std::option::Option<terrars::PrimField<f64>>,
    price: terrars::PrimField<std::string::String>,
    quantity: terrars::PrimField<f64>,
}

impl PaymentLink_LineItems {
    #[doc = "Set the field `adjustable_quantity_enabled`.\n"]
    pub fn set_adjustable_quantity_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.adjustable_quantity_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `adjustable_quantity_maximum`.\n"]
    pub fn set_adjustable_quantity_maximum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.adjustable_quantity_maximum = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `adjustable_quantity_minimum`.\n"]
    pub fn set_adjustable_quantity_minimum(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.adjustable_quantity_minimum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PaymentLink_LineItems {
    type O = terrars::BlockAssignable<PaymentLink_LineItems>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentLink_LineItems {
    #[doc = ""]
    pub price: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub quantity: terrars::PrimField<f64>,
}

impl BuildPaymentLink_LineItems {
    pub fn build(self) -> PaymentLink_LineItems {
        PaymentLink_LineItems {
            adjustable_quantity_enabled: core::default::Default::default(),
            adjustable_quantity_maximum: core::default::Default::default(),
            adjustable_quantity_minimum: core::default::Default::default(),
            price: self.price,
            quantity: self.quantity,
        }
    }
}

#[allow(dead_code)]
pub struct PaymentLink_LineItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentLink_LineItemsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PaymentLink_LineItemsRef {
        PaymentLink_LineItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentLink_LineItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `adjustable_quantity_enabled` after provisioning.\n"]
    pub fn adjustable_quantity_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable_quantity_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `adjustable_quantity_maximum` after provisioning.\n"]
    pub fn adjustable_quantity_maximum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable_quantity_maximum", self.base))
    }

    #[doc = "Get a reference to the value of field `adjustable_quantity_minimum` after provisioning.\n"]
    pub fn adjustable_quantity_minimum(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable_quantity_minimum", self.base))
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\n"]
    pub fn price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}

#[derive(Serialize)]
pub struct PaymentLink_ShippingOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_rate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PaymentLink_ShippingOptions {
    #[doc = "Set the field `shipping_rate`.\nThe ID of the Shipping Rate to use for this shipping option."]
    pub fn set_shipping_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shipping_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PaymentLink_ShippingOptions {
    type O = terrars::BlockAssignable<PaymentLink_ShippingOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPaymentLink_ShippingOptions {}

impl BuildPaymentLink_ShippingOptions {
    pub fn build(self) -> PaymentLink_ShippingOptions {
        PaymentLink_ShippingOptions { shipping_rate: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PaymentLink_ShippingOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PaymentLink_ShippingOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PaymentLink_ShippingOptionsRef {
        PaymentLink_ShippingOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PaymentLink_ShippingOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `shipping_amount` after provisioning.\nA non-negative integer in cents representing how much to charge."]
    pub fn shipping_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_amount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shipping_rate` after provisioning.\nThe ID of the Shipping Rate to use for this shipping option."]
    pub fn shipping_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_rate", self.base))
    }
}
