#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct TerminalReaderData_ {
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
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    registration_code: terrars::PrimField<std::string::String>,
}

struct TerminalReader_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TerminalReaderData_>,
}

#[derive(Clone)]
pub struct TerminalReader(std::rc::Rc<TerminalReader_>);

impl TerminalReader {
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
        "Set the field `label`.\nCustom label given to the reader for easier identification. If no label is specified, the registration code will be used."]
    pub fn set_label(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\nThe location to assign the reader to."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action_failure_code` after provisioning.\nFailure code, only set if status is `failed`."]
    pub fn action_failure_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action_failure_message` after provisioning.\nDetailed failure message, only set if status is `failed`."]
    pub fn action_failure_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action_process_payment_intent_payment_intent` after provisioning.\nMost recent PaymentIntent processed by the reader."]
    pub fn action_process_payment_intent_payment_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_payment_intent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_payment_intent_process_config_skip_tipping` after provisioning.\nOverride showing a tipping selection screen on this transaction."]
    pub fn action_process_payment_intent_process_config_skip_tipping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_skip_tipping", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_payment_intent_process_config_tipping_amount_eligible` after provisioning.\nAmount used to calculate tip suggestions on tipping selection screen for this transaction. Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency)."]
    pub fn action_process_payment_intent_process_config_tipping_amount_eligible(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_tipping_amount_eligible", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_setup_intent_generated_card` after provisioning.\nID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions. Only present if it was possible to generate a card PaymentMethod."]
    pub fn action_process_setup_intent_generated_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_generated_card", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_setup_intent_setup_intent` after provisioning.\nMost recent SetupIntent processed by the reader."]
    pub fn action_process_setup_intent_setup_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_setup_intent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn action_set_reader_display_cart_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_currency", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_line_items` after provisioning.\nList of line items in the cart."]
    pub fn action_set_reader_display_cart_line_items(
        &self,
    ) -> terrars::ListRef<TerminalReader_ActionSetReaderDisplayCartLineItemsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_line_items", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_tax` after provisioning.\nTax amount for the entire cart. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_tax", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_total` after provisioning.\nTotal amount for the entire cart, including tax. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_total", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_type` after provisioning.\nType of information to be displayed by the reader."]
    pub fn action_set_reader_display_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_status` after provisioning.\nStatus of the action performed by the reader."]
    pub fn action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action_type` after provisioning.\nType of action performed by the reader."]
    pub fn action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_sw_version` after provisioning.\nThe current software version of the reader."]
    pub fn device_sw_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_sw_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nType of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`."]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe local IP address of the reader."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nCustom label given to the reader for easier identification. If no label is specified, the registration code will be used."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location to assign the reader to."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registration_code` after provisioning.\nA code generated by the reader used for registering to an account."]
    pub fn registration_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\nSerial number of the reader."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe networking status of the reader."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl terrars::Referable for TerminalReader {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TerminalReader { }

impl terrars::ToListMappable for TerminalReader {
    type O = terrars::ListRef<TerminalReaderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TerminalReader_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_terminal_reader".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTerminalReader {
    pub tf_id: std::string::String,
    #[doc = "A code generated by the reader used for registering to an account."]
    pub registration_code: terrars::PrimField<std::string::String>,
}

impl BuildTerminalReader {
    pub fn build(self, stack: &mut terrars::Stack) -> TerminalReader {
        let out = TerminalReader(std::rc::Rc::new(TerminalReader_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TerminalReaderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                label: core::default::Default::default(),
                location: core::default::Default::default(),
                registration_code: self.registration_code,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TerminalReaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TerminalReaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TerminalReaderRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_failure_code` after provisioning.\nFailure code, only set if status is `failed`."]
    pub fn action_failure_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action_failure_message` after provisioning.\nDetailed failure message, only set if status is `failed`."]
    pub fn action_failure_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action_process_payment_intent_payment_intent` after provisioning.\nMost recent PaymentIntent processed by the reader."]
    pub fn action_process_payment_intent_payment_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_payment_intent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_payment_intent_process_config_skip_tipping` after provisioning.\nOverride showing a tipping selection screen on this transaction."]
    pub fn action_process_payment_intent_process_config_skip_tipping(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_skip_tipping", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_payment_intent_process_config_tipping_amount_eligible` after provisioning.\nAmount used to calculate tip suggestions on tipping selection screen for this transaction. Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency)."]
    pub fn action_process_payment_intent_process_config_tipping_amount_eligible(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_tipping_amount_eligible", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_setup_intent_generated_card` after provisioning.\nID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions. Only present if it was possible to generate a card PaymentMethod."]
    pub fn action_process_setup_intent_generated_card(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_generated_card", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_process_setup_intent_setup_intent` after provisioning.\nMost recent SetupIntent processed by the reader."]
    pub fn action_process_setup_intent_setup_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_setup_intent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn action_set_reader_display_cart_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_currency", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_line_items` after provisioning.\nList of line items in the cart."]
    pub fn action_set_reader_display_cart_line_items(
        &self,
    ) -> terrars::ListRef<TerminalReader_ActionSetReaderDisplayCartLineItemsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_line_items", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_tax` after provisioning.\nTax amount for the entire cart. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_tax", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_cart_total` after provisioning.\nTotal amount for the entire cart, including tax. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_total", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_set_reader_display_type` after provisioning.\nType of information to be displayed by the reader."]
    pub fn action_set_reader_display_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_type", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `action_status` after provisioning.\nStatus of the action performed by the reader."]
    pub fn action_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `action_type` after provisioning.\nType of action performed by the reader."]
    pub fn action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_sw_version` after provisioning.\nThe current software version of the reader."]
    pub fn device_sw_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_sw_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `device_type` after provisioning.\nType of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`."]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip_address` after provisioning.\nThe local IP address of the reader."]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label` after provisioning.\nCustom label given to the reader for easier identification. If no label is specified, the registration code will be used."]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location to assign the reader to."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `registration_code` after provisioning.\nA code generated by the reader used for registering to an account."]
    pub fn registration_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\nSerial number of the reader."]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe networking status of the reader."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TerminalReader_ActionSetReaderDisplayCartLineItems {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity: std::option::Option<terrars::PrimField<f64>>,
}

impl TerminalReader_ActionSetReaderDisplayCartLineItems {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quantity`.\n"]
    pub fn set_quantity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.quantity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TerminalReader_ActionSetReaderDisplayCartLineItems {
    type O = terrars::BlockAssignable<TerminalReader_ActionSetReaderDisplayCartLineItems>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTerminalReader_ActionSetReaderDisplayCartLineItems {}

impl BuildTerminalReader_ActionSetReaderDisplayCartLineItems {
    pub fn build(self) -> TerminalReader_ActionSetReaderDisplayCartLineItems {
        TerminalReader_ActionSetReaderDisplayCartLineItems {
            amount: core::default::Default::default(),
            description: core::default::Default::default(),
            quantity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TerminalReader_ActionSetReaderDisplayCartLineItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TerminalReader_ActionSetReaderDisplayCartLineItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> TerminalReader_ActionSetReaderDisplayCartLineItemsRef {
        TerminalReader_ActionSetReaderDisplayCartLineItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TerminalReader_ActionSetReaderDisplayCartLineItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}
