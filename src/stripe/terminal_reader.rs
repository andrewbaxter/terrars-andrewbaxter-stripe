use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct TerminalReaderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    registration_code: PrimField<String>,
}

struct TerminalReader_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TerminalReaderData>,
}

#[derive(Clone)]
pub struct TerminalReader(Rc<TerminalReader_>);

impl TerminalReader {
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

    #[doc= "Set the field `label`.\nCustom label given to the reader for easier identification. If no label is specified, the registration code will be used."]
    pub fn set_label(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().label = Some(v.into());
        self
    }

    #[doc= "Set the field `location`.\nThe location to assign the reader to."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `action_failure_code` after provisioning.\nFailure code, only set if status is `failed`."]
    pub fn action_failure_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_failure_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_failure_message` after provisioning.\nDetailed failure message, only set if status is `failed`."]
    pub fn action_failure_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_failure_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_process_payment_intent_payment_intent` after provisioning.\nMost recent PaymentIntent processed by the reader."]
    pub fn action_process_payment_intent_payment_intent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_payment_intent", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_payment_intent_process_config_skip_tipping` after provisioning.\nOverride showing a tipping selection screen on this transaction."]
    pub fn action_process_payment_intent_process_config_skip_tipping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_skip_tipping", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_payment_intent_process_config_tipping_amount_eligible` after provisioning.\nAmount used to calculate tip suggestions on tipping selection screen for this transaction. Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency)."]
    pub fn action_process_payment_intent_process_config_tipping_amount_eligible(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_tipping_amount_eligible", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_setup_intent_generated_card` after provisioning.\nID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions. Only present if it was possible to generate a card PaymentMethod."]
    pub fn action_process_setup_intent_generated_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_generated_card", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_setup_intent_setup_intent` after provisioning.\nMost recent SetupIntent processed by the reader."]
    pub fn action_process_setup_intent_setup_intent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_setup_intent", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn action_set_reader_display_cart_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_currency", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_line_items` after provisioning.\nList of line items in the cart."]
    pub fn action_set_reader_display_cart_line_items(
        &self,
    ) -> ListRef<TerminalReaderActionSetReaderDisplayCartLineItemsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_line_items", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_tax` after provisioning.\nTax amount for the entire cart. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_tax(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_set_reader_display_cart_tax", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_total` after provisioning.\nTotal amount for the entire cart, including tax. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_total(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_set_reader_display_cart_total", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_type` after provisioning.\nType of information to be displayed by the reader."]
    pub fn action_set_reader_display_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_set_reader_display_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_status` after provisioning.\nStatus of the action performed by the reader."]
    pub fn action_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_type` after provisioning.\nType of action performed by the reader."]
    pub fn action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_sw_version` after provisioning.\nThe current software version of the reader."]
    pub fn device_sw_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_sw_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_type` after provisioning.\nType of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`."]
    pub fn device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\nThe local IP address of the reader."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\nCustom label given to the reader for easier identification. If no label is specified, the registration code will be used."]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location to assign the reader to."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registration_code` after provisioning.\nA code generated by the reader used for registering to an account."]
    pub fn registration_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `serial_number` after provisioning.\nSerial number of the reader."]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe networking status of the reader."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl Resource for TerminalReader {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for TerminalReader {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for TerminalReader {
    type O = ListRef<TerminalReaderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for TerminalReader_ {
    fn extract_resource_type(&self) -> String {
        "stripe_terminal_reader".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTerminalReader {
    pub tf_id: String,
    #[doc= "A code generated by the reader used for registering to an account."]
    pub registration_code: PrimField<String>,
}

impl BuildTerminalReader {
    pub fn build(self, stack: &mut Stack) -> TerminalReader {
        let out = TerminalReader(Rc::new(TerminalReader_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TerminalReaderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for TerminalReaderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TerminalReaderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action_failure_code` after provisioning.\nFailure code, only set if status is `failed`."]
    pub fn action_failure_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_failure_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_failure_message` after provisioning.\nDetailed failure message, only set if status is `failed`."]
    pub fn action_failure_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_failure_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_process_payment_intent_payment_intent` after provisioning.\nMost recent PaymentIntent processed by the reader."]
    pub fn action_process_payment_intent_payment_intent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_payment_intent", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_payment_intent_process_config_skip_tipping` after provisioning.\nOverride showing a tipping selection screen on this transaction."]
    pub fn action_process_payment_intent_process_config_skip_tipping(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_skip_tipping", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_payment_intent_process_config_tipping_amount_eligible` after provisioning.\nAmount used to calculate tip suggestions on tipping selection screen for this transaction. Must be a positive integer in the smallest currency unit (e.g., 100 cents to represent $1.00 or 100 to represent ¥100, a zero-decimal currency)."]
    pub fn action_process_payment_intent_process_config_tipping_amount_eligible(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_payment_intent_process_config_tipping_amount_eligible", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_setup_intent_generated_card` after provisioning.\nID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for future transactions. Only present if it was possible to generate a card PaymentMethod."]
    pub fn action_process_setup_intent_generated_card(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_generated_card", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_process_setup_intent_setup_intent` after provisioning.\nMost recent SetupIntent processed by the reader."]
    pub fn action_process_setup_intent_setup_intent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_process_setup_intent_setup_intent", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn action_set_reader_display_cart_currency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_currency", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_line_items` after provisioning.\nList of line items in the cart."]
    pub fn action_set_reader_display_cart_line_items(
        &self,
    ) -> ListRef<TerminalReaderActionSetReaderDisplayCartLineItemsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.action_set_reader_display_cart_line_items", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_tax` after provisioning.\nTax amount for the entire cart. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_tax(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_set_reader_display_cart_tax", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_cart_total` after provisioning.\nTotal amount for the entire cart, including tax. A positive integer in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn action_set_reader_display_cart_total(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_set_reader_display_cart_total", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_set_reader_display_type` after provisioning.\nType of information to be displayed by the reader."]
    pub fn action_set_reader_display_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_set_reader_display_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_status` after provisioning.\nStatus of the action performed by the reader."]
    pub fn action_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_type` after provisioning.\nType of action performed by the reader."]
    pub fn action_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_sw_version` after provisioning.\nThe current software version of the reader."]
    pub fn device_sw_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_sw_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `device_type` after provisioning.\nType of reader, one of `bbpos_wisepad3`, `stripe_m2`, `bbpos_chipper2x`, `bbpos_wisepos_e`, `verifone_P400`, or `simulated_wisepos_e`."]
    pub fn device_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip_address` after provisioning.\nThe local IP address of the reader."]
    pub fn ip_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label` after provisioning.\nCustom label given to the reader for easier identification. If no label is specified, the registration code will be used."]
    pub fn label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe location to assign the reader to."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `registration_code` after provisioning.\nA code generated by the reader used for registering to an account."]
    pub fn registration_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.registration_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `serial_number` after provisioning.\nSerial number of the reader."]
    pub fn serial_number(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe networking status of the reader."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct TerminalReaderActionSetReaderDisplayCartLineItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<PrimField<f64>>,
}

impl TerminalReaderActionSetReaderDisplayCartLineItemsEl {
    #[doc= "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.amount = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `quantity`.\n"]
    pub fn set_quantity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.quantity = Some(v.into());
        self
    }
}

impl ToListMappable for TerminalReaderActionSetReaderDisplayCartLineItemsEl {
    type O = BlockAssignable<TerminalReaderActionSetReaderDisplayCartLineItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTerminalReaderActionSetReaderDisplayCartLineItemsEl {}

impl BuildTerminalReaderActionSetReaderDisplayCartLineItemsEl {
    pub fn build(self) -> TerminalReaderActionSetReaderDisplayCartLineItemsEl {
        TerminalReaderActionSetReaderDisplayCartLineItemsEl {
            amount: core::default::Default::default(),
            description: core::default::Default::default(),
            quantity: core::default::Default::default(),
        }
    }
}

pub struct TerminalReaderActionSetReaderDisplayCartLineItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for TerminalReaderActionSetReaderDisplayCartLineItemsElRef {
    fn new(shared: StackShared, base: String) -> TerminalReaderActionSetReaderDisplayCartLineItemsElRef {
        TerminalReaderActionSetReaderDisplayCartLineItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TerminalReaderActionSetReaderDisplayCartLineItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}
