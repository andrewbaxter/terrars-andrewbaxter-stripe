#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct InvoiceData_ {
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
    account_tax_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_fee_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_advance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_tax_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer: std::option::Option<terrars::PrimField<std::string::String>>,
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
    due_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    footer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_invoice_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_invoice_invoice: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_behalf_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_default_mandate: std::option::Option<terrars::PrimField<std::string::String>>,
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
    payment_settings_payment_method_options_card_installments_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_installments_plan_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_installments_plan_interval: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_settings_payment_method_options_card_installments_plan_type: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
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
    pending_invoice_items_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rendering_options_amount_tax_display: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_descriptor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_fields: std::option::Option<std::vec::Vec<Invoice_CustomFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discounts: std::option::Option<std::vec::Vec<Invoice_Discounts>>,
    dynamic: InvoiceDynamic,
}

struct Invoice_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<InvoiceData_>,
}

#[derive(Clone)]
pub struct Invoice(std::rc::Rc<Invoice_>);

impl Invoice {
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
        "Set the field `account_tax_ids`.\nThe account tax IDs associated with the invoice. Only editable when the invoice is a draft."]
    pub fn set_account_tax_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().account_tax_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `application_fee_amount`.\nA fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account. The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee. For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees)."]
    pub fn set_application_fee_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().application_fee_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `auto_advance`.\nControls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice. When `false`, the invoice's state will not automatically advance without an explicit action."]
    pub fn set_auto_advance(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_advance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `automatic_tax_enabled`.\nWhether Stripe automatically computes tax on this invoice. Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices."]
    pub fn set_automatic_tax_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic_tax_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `collection_method`.\nEither `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions. Defaults to `charge_automatically`."]
    pub fn set_collection_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().collection_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `currency`.\nThe currency to create this invoice in. Defaults to that of `customer` if not specified."]
    pub fn set_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer`.\nThe ID of the customer who will be billed."]
    pub fn set_customer(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `days_until_due`.\nThe number of days from when the invoice is created until it is due. Valid only for invoices where `collection_method=send_invoice`."]
    pub fn set_days_until_due(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().days_until_due = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_payment_method`.\nID of the default payment method for the invoice. It must belong to the customer associated with the invoice. If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings."]
    pub fn set_default_payment_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_source`.\nID of the default payment source for the invoice. It must belong to the customer associated with the invoice and be in a chargeable state. If not set, defaults to the subscription's default source, if any, or to the customer's default source."]
    pub fn set_default_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_tax_rates`.\nThe tax rates that will apply to any line item that does not have `tax_rates` set."]
    pub fn set_default_tax_rates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().default_tax_rates = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn arbitrary string attached to the object. Often useful for displaying to users. Referenced as 'memo' in the Dashboard."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `due_date`.\nThe date on which payment for this invoice is due. Valid only for invoices where `collection_method=send_invoice`."]
    pub fn set_due_date(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().due_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `footer`.\nFooter to be displayed on the invoice."]
    pub fn set_footer(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().footer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_invoice_action`.\nThe relation between this invoice and the cloned invoice"]
    pub fn set_from_invoice_action(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from_invoice_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `from_invoice_invoice`.\nThe invoice that was cloned."]
    pub fn set_from_invoice_invoice(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().from_invoice_invoice = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `on_behalf_of`.\nThe account (if any) for which the funds of the invoice payment are intended. If set, the invoice will be presented with the branding and support information of the specified account. See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details."]
    pub fn set_on_behalf_of(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().on_behalf_of = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_default_mandate`.\nID of the mandate to be used for this invoice. It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set."]
    pub fn set_payment_settings_default_mandate(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_default_mandate = std::option::Option::Some(v.into());
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
        "Set the field `payment_settings_payment_method_options_card_installments_enabled`.\nWhether Installments are enabled for this Invoice."]
    pub fn set_payment_settings_payment_method_options_card_installments_enabled(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_installments_enabled =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_installments_plan_count`.\n"]
    pub fn set_payment_settings_payment_method_options_card_installments_plan_count(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_installments_plan_count =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_installments_plan_interval`.\n"]
    pub fn set_payment_settings_payment_method_options_card_installments_plan_interval(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_installments_plan_interval =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_installments_plan_type`.\n"]
    pub fn set_payment_settings_payment_method_options_card_installments_plan_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_options_card_installments_plan_type =
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
        "Set the field `payment_settings_payment_method_types`.\nThe list of payment method types (e.g. card) to provide to the invoice’s PaymentIntent. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice)."]
    pub fn set_payment_settings_payment_method_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_settings_payment_method_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pending_invoice_items_behavior`.\nHow to handle pending invoice items on invoice creation. One of `include` or `exclude`. `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist. `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not. Defaults to `exclude` if the parameter is omitted."]
    pub fn set_pending_invoice_items_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().pending_invoice_items_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rendering_options_amount_tax_display`.\nHow line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub fn set_rendering_options_amount_tax_display(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().rendering_options_amount_tax_display = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `statement_descriptor`.\nExtra information about a charge for the customer's credit card statement. It must contain at least one letter. If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`."]
    pub fn set_statement_descriptor(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().statement_descriptor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subscription`.\nThe ID of the subscription to invoice, if any. If set, the created invoice will only include pending invoice items for that subscription and pending invoice items not associated with any subscription if `pending_invoice_items_behavior` is `include`. The subscription's billing cycle and regular subscription events won't be affected."]
    pub fn set_subscription(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `transfer_data_amount`.\nThe amount in %s that will be transferred to the destination account when the invoice is paid. By default, the entire amount is transferred to the destination."]
    pub fn set_transfer_data_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().transfer_data_amount = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `custom_fields`.\n"]
    pub fn set_custom_fields(self, v: impl std::convert::Into<terrars::BlockAssignable<Invoice_CustomFields>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `discounts`.\n"]
    pub fn set_discounts(self, v: impl std::convert::Into<terrars::BlockAssignable<Invoice_Discounts>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().discounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.discounts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_country` after provisioning.\nThe country of the business associated with this invoice, most often the business creating the invoice."]
    pub fn account_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `account_name` after provisioning.\nThe public name of the business associated with this invoice, most often the business creating the invoice."]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `account_tax_ids` after provisioning.\nThe account tax IDs associated with the invoice. Only editable when the invoice is a draft."]
    pub fn account_tax_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_tax_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_due` after provisioning.\nFinal amount due at this time for this invoice. If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0. If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account. The charge that gets generated for the invoice will be for the amount specified in `amount_due`."]
    pub fn amount_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_paid` after provisioning.\nThe amount, in %s, that was paid."]
    pub fn amount_paid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_paid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_remaining` after provisioning.\nThe difference between amount_due and amount_paid, in %s."]
    pub fn amount_remaining(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_remaining", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the invoice."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_amount` after provisioning.\nA fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account. The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee. For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees)."]
    pub fn application_fee_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attempt_count` after provisioning.\nNumber of payment attempts made for this invoice, from the perspective of the payment retry schedule. Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count. In other words, manual payment attempts after the first attempt do not affect the retry schedule."]
    pub fn attempt_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attempted` after provisioning.\nWhether an attempt has been made to pay the invoice. An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users."]
    pub fn attempted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_advance` after provisioning.\nControls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice. When `false`, the invoice's state will not automatically advance without an explicit action."]
    pub fn auto_advance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_advance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\nWhether Stripe automatically computes tax on this invoice. Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices."]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_status` after provisioning.\nThe status of the most recent automated tax calculation for this invoice."]
    pub fn automatic_tax_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_reason` after provisioning.\nIndicates the reason why the invoice was created. `subscription_cycle` indicates an invoice created by a subscription advancing into a new period. `subscription_create` indicates an invoice created due to creating a subscription. `subscription_update` indicates an invoice created due to updating a subscription. `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement. `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor). The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint. `subscription_threshold` indicates an invoice created due to a billing threshold being reached."]
    pub fn billing_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `charge` after provisioning.\nID of the latest charge generated for this invoice, if any."]
    pub fn charge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.charge", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_method` after provisioning.\nEither `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions. Defaults to `charge_automatically`."]
    pub fn collection_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThe currency to create this invoice in. Defaults to that of `customer` if not specified."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer who will be billed."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn customer_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn customer_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn customer_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn customer_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn customer_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_state` after provisioning.\nState, county, province, or region."]
    pub fn customer_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_email` after provisioning.\nThe customer's email. Until the invoice is finalized, this field will equal `customer.email`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_name` after provisioning.\nThe customer's name. Until the invoice is finalized, this field will equal `customer.name`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_phone` after provisioning.\nThe customer's phone number. Until the invoice is finalized, this field will equal `customer.phone`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn customer_shipping_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn customer_shipping_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn customer_shipping_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn customer_shipping_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn customer_shipping_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_state` after provisioning.\nState, county, province, or region."]
    pub fn customer_shipping_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_carrier` after provisioning.\nThe delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub fn customer_shipping_carrier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_shipping_carrier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_shipping_name` after provisioning.\nRecipient name."]
    pub fn customer_shipping_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_shipping_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_phone` after provisioning.\nRecipient phone (including extension)."]
    pub fn customer_shipping_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_shipping_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_tracking_number` after provisioning.\nThe tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas."]
    pub fn customer_shipping_tracking_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_tracking_number", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_tax_exempt` after provisioning.\nThe customer's tax exempt status. Until the invoice is finalized, this field will equal `customer.tax_exempt`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_tax_exempt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_tax_exempt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_tax_ids` after provisioning.\nThe customer's tax IDs. Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_tax_ids(&self) -> terrars::ListRef<Invoice_CustomerTaxIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_tax_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `days_until_due` after provisioning.\nThe number of days from when the invoice is created until it is due. Valid only for invoices where `collection_method=send_invoice`."]
    pub fn days_until_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_until_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_payment_method` after provisioning.\nID of the default payment method for the invoice. It must belong to the customer associated with the invoice. If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings."]
    pub fn default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_payment_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_source` after provisioning.\nID of the default payment source for the invoice. It must belong to the customer associated with the invoice and be in a chargeable state. If not set, defaults to the subscription's default source, if any, or to the customer's default source."]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_tax_rates` after provisioning.\nThe tax rates that will apply to any line item that does not have `tax_rates` set."]
    pub fn default_tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_tax_rates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string attached to the object. Often useful for displaying to users. Referenced as 'memo' in the Dashboard."]
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
    pub fn discount_coupon_currency_options(&self) -> terrars::ListRef<Invoice_DiscountCouponCurrencyOptionsRef> {
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
        "Get a reference to the value of field `due_date` after provisioning.\nThe date on which payment for this invoice is due. Valid only for invoices where `collection_method=send_invoice`."]
    pub fn due_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ending_balance` after provisioning.\nEnding customer balance after the invoice is finalized. Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice. If the invoice has not been finalized yet, this will be null."]
    pub fn ending_balance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ending_balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `footer` after provisioning.\nFooter to be displayed on the invoice."]
    pub fn footer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.footer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_invoice_action` after provisioning.\nThe relation between this invoice and the cloned invoice"]
    pub fn from_invoice_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_invoice_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_invoice_invoice` after provisioning.\nThe invoice that was cloned."]
    pub fn from_invoice_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_invoice_invoice", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosted_invoice_url` after provisioning.\nThe URL for the hosted invoice page, which allows customers to view and pay an invoice. If the invoice has not been finalized yet, this will be null."]
    pub fn hosted_invoice_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_invoice_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object. This property is always present unless the invoice is an upcoming invoice. See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_pdf` after provisioning.\nThe link to download the PDF for the invoice. If the invoice has not been finalized yet, this will be null."]
    pub fn invoice_pdf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_pdf", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_revision` after provisioning.\nThe ID of the most recent non-draft revision of this invoice"]
    pub fn latest_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lines_data` after provisioning.\nDetails about each object."]
    pub fn lines_data(&self) -> terrars::ListRef<Invoice_LinesDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lines_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lines_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn lines_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lines_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lines_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn lines_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lines_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lines_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn lines_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lines_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_payment_attempt` after provisioning.\nThe time at which payment will next be attempted. This value will be `null` for invoices where `collection_method=send_invoice`."]
    pub fn next_payment_attempt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_payment_attempt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nA unique, identifying string that appears on emails sent to the customer for this invoice. This starts with the customer's unique invoice_prefix if it is specified."]
    pub fn number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_behalf_of` after provisioning.\nThe account (if any) for which the funds of the invoice payment are intended. If set, the invoice will be presented with the branding and support information of the specified account. See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details."]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paid` after provisioning.\nWhether payment was successfully collected for this invoice. An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance."]
    pub fn paid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paid_out_of_band` after provisioning.\nReturns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe."]
    pub fn paid_out_of_band(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paid_out_of_band", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_intent` after provisioning.\nThe PaymentIntent associated with this invoice. The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice. Note that voiding an invoice will cancel the PaymentIntent."]
    pub fn payment_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_intent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_default_mandate` after provisioning.\nID of the mandate to be used for this invoice. It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set."]
    pub fn payment_settings_default_mandate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_default_mandate", self.extract_ref()),
        )
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
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_enabled` after provisioning.\nWhether Installments are enabled for this Invoice."]
    pub fn payment_settings_payment_method_options_card_installments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_plan_count` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_installments_plan_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_plan_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_plan_interval` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_installments_plan_interval(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_plan_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_plan_type` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_installments_plan_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_plan_type", self.extract_ref()),
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
        "Get a reference to the value of field `payment_settings_payment_method_types` after provisioning.\nThe list of payment method types (e.g. card) to provide to the invoice’s PaymentIntent. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice)."]
    pub fn payment_settings_payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_items_behavior` after provisioning.\nHow to handle pending invoice items on invoice creation. One of `include` or `exclude`. `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist. `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not. Defaults to `exclude` if the parameter is omitted."]
    pub fn pending_invoice_items_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_items_behavior", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `period_end` after provisioning.\nEnd of the usage period during which invoice items were added to this invoice."]
    pub fn period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period_start` after provisioning.\nStart of the usage period during which invoice items were added to this invoice."]
    pub fn period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `post_payment_credit_notes_amount` after provisioning.\nTotal amount of all post-payment credit notes issued for this invoice."]
    pub fn post_payment_credit_notes_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_payment_credit_notes_amount", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pre_payment_credit_notes_amount` after provisioning.\nTotal amount of all pre-payment credit notes issued for this invoice."]
    pub fn pre_payment_credit_notes_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_payment_credit_notes_amount", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `quote` after provisioning.\nThe quote this invoice was generated from."]
    pub fn quote(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `receipt_number` after provisioning.\nThis is the transaction number that appears on email receipts sent for this invoice."]
    pub fn receipt_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.receipt_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rendering_options_amount_tax_display` after provisioning.\nHow line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub fn rendering_options_amount_tax_display(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.rendering_options_amount_tax_display", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `starting_balance` after provisioning.\nStarting customer balance before the invoice is finalized. If the invoice has not been finalized yet, this will be the current customer balance. For revision invoices, this also includes any customer balance that was applied to the original invoice."]
    pub fn starting_balance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `statement_descriptor` after provisioning.\nExtra information about a charge for the customer's credit card statement. It must contain at least one letter. If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`."]
    pub fn statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_descriptor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview)"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_finalized_at` after provisioning.\nThe time that the invoice draft was finalized."]
    pub fn status_transitions_finalized_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_transitions_finalized_at", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_marked_uncollectible_at` after provisioning.\nThe time that the invoice was marked uncollectible."]
    pub fn status_transitions_marked_uncollectible_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_transitions_marked_uncollectible_at", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_paid_at` after provisioning.\nThe time that the invoice was paid."]
    pub fn status_transitions_paid_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_transitions_paid_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_voided_at` after provisioning.\nThe time that the invoice was voided."]
    pub fn status_transitions_voided_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_transitions_voided_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nThe ID of the subscription to invoice, if any. If set, the created invoice will only include pending invoice items for that subscription and pending invoice items not associated with any subscription if `pending_invoice_items_behavior` is `include`. The subscription's billing cycle and regular subscription events won't be affected."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_proration_date` after provisioning.\nOnly set for upcoming invoices that preview prorations. The time used to calculate prorations."]
    pub fn subscription_proration_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_proration_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subtotal` after provisioning.\nTotal of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied. Item discounts are already incorporated"]
    pub fn subtotal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtotal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subtotal_excluding_tax` after provisioning.\nThe integer amount in %s representing the subtotal of the invoice before any invoice level discount or tax is applied. Item discounts are already incorporated"]
    pub fn subtotal_excluding_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtotal_excluding_tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax` after provisioning.\nThe amount of tax on this invoice. This is the sum of all the tax amounts on this invoice."]
    pub fn tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this invoice belongs to."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold_reason_amount_gte` after provisioning.\nThe total invoice amount threshold boundary if it triggered the threshold invoice."]
    pub fn threshold_reason_amount_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_reason_amount_gte", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold_reason_item_reasons` after provisioning.\nIndicates which line items triggered a threshold invoice."]
    pub fn threshold_reason_item_reasons(&self) -> terrars::ListRef<Invoice_ThresholdReasonItemReasonsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_reason_item_reasons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total` after provisioning.\nTotal after discounts and taxes."]
    pub fn total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_discount_amounts` after provisioning.\nThe aggregate amounts calculated per discount across all line items."]
    pub fn total_discount_amounts(&self) -> terrars::ListRef<Invoice_TotalDiscountAmountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_discount_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_excluding_tax` after provisioning.\nThe integer amount in %s representing the total amount of the invoice including all discounts but excluding all tax."]
    pub fn total_excluding_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_excluding_tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_tax_amounts` after provisioning.\nThe aggregate amounts calculated per tax rate for all line items."]
    pub fn total_tax_amounts(&self) -> terrars::ListRef<Invoice_TotalTaxAmountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_tax_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_amount` after provisioning.\nThe amount in %s that will be transferred to the destination account when the invoice is paid. By default, the entire amount is transferred to the destination."]
    pub fn transfer_data_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_destination` after provisioning.\nThe account where funds from the payment will be transferred to upon payment success."]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `webhooks_delivered_at` after provisioning.\nInvoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand). This field tracks the time when webhooks for this invoice were successfully delivered. If the invoice had no webhooks to deliver, this will be set while the invoice is being created."]
    pub fn webhooks_delivered_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhooks_delivered_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_fields` after provisioning.\n"]
    pub fn custom_fields(&self) -> terrars::ListRef<Invoice_CustomFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> terrars::ListRef<Invoice_DiscountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discounts", self.extract_ref()))
    }
}

impl terrars::Referable for Invoice {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Invoice { }

impl terrars::ToListMappable for Invoice {
    type O = terrars::ListRef<InvoiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Invoice_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_invoice".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInvoice {
    pub tf_id: std::string::String,
}

impl BuildInvoice {
    pub fn build(self, stack: &mut terrars::Stack) -> Invoice {
        let out = Invoice(std::rc::Rc::new(Invoice_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(InvoiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_tax_ids: core::default::Default::default(),
                application_fee_amount: core::default::Default::default(),
                auto_advance: core::default::Default::default(),
                automatic_tax_enabled: core::default::Default::default(),
                collection_method: core::default::Default::default(),
                currency: core::default::Default::default(),
                customer: core::default::Default::default(),
                days_until_due: core::default::Default::default(),
                default_payment_method: core::default::Default::default(),
                default_source: core::default::Default::default(),
                default_tax_rates: core::default::Default::default(),
                description: core::default::Default::default(),
                due_date: core::default::Default::default(),
                footer: core::default::Default::default(),
                from_invoice_action: core::default::Default::default(),
                from_invoice_invoice: core::default::Default::default(),
                on_behalf_of: core::default::Default::default(),
                payment_settings_default_mandate: core::default::Default::default(),
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
                payment_settings_payment_method_options_card_installments_enabled: core::default::Default::default(),
                payment_settings_payment_method_options_card_installments_plan_count: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_card_installments_plan_interval: core
                ::default
                ::Default
                ::default(),
                payment_settings_payment_method_options_card_installments_plan_type: core::default::Default::default(),
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
                pending_invoice_items_behavior: core::default::Default::default(),
                rendering_options_amount_tax_display: core::default::Default::default(),
                statement_descriptor: core::default::Default::default(),
                subscription: core::default::Default::default(),
                transfer_data_amount: core::default::Default::default(),
                transfer_data_destination: core::default::Default::default(),
                custom_fields: core::default::Default::default(),
                discounts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct InvoiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for InvoiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl InvoiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_country` after provisioning.\nThe country of the business associated with this invoice, most often the business creating the invoice."]
    pub fn account_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `account_name` after provisioning.\nThe public name of the business associated with this invoice, most often the business creating the invoice."]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `account_tax_ids` after provisioning.\nThe account tax IDs associated with the invoice. Only editable when the invoice is a draft."]
    pub fn account_tax_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_tax_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_due` after provisioning.\nFinal amount due at this time for this invoice. If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0. If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account. The charge that gets generated for the invoice will be for the amount specified in `amount_due`."]
    pub fn amount_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_paid` after provisioning.\nThe amount, in %s, that was paid."]
    pub fn amount_paid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_paid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `amount_remaining` after provisioning.\nThe difference between amount_due and amount_paid, in %s."]
    pub fn amount_remaining(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_remaining", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the invoice."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application_fee_amount` after provisioning.\nA fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account. The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee. For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees)."]
    pub fn application_fee_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attempt_count` after provisioning.\nNumber of payment attempts made for this invoice, from the perspective of the payment retry schedule. Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count. In other words, manual payment attempts after the first attempt do not affect the retry schedule."]
    pub fn attempt_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempt_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attempted` after provisioning.\nWhether an attempt has been made to pay the invoice. An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users."]
    pub fn attempted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attempted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_advance` after provisioning.\nControls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice. When `false`, the invoice's state will not automatically advance without an explicit action."]
    pub fn auto_advance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_advance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\nWhether Stripe automatically computes tax on this invoice. Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices."]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `automatic_tax_status` after provisioning.\nThe status of the most recent automated tax calculation for this invoice."]
    pub fn automatic_tax_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `billing_reason` after provisioning.\nIndicates the reason why the invoice was created. `subscription_cycle` indicates an invoice created by a subscription advancing into a new period. `subscription_create` indicates an invoice created due to creating a subscription. `subscription_update` indicates an invoice created due to updating a subscription. `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement. `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor). The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint. `subscription_threshold` indicates an invoice created due to a billing threshold being reached."]
    pub fn billing_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `charge` after provisioning.\nID of the latest charge generated for this invoice, if any."]
    pub fn charge(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.charge", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_method` after provisioning.\nEither `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions. Defaults to `charge_automatically`."]
    pub fn collection_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThe currency to create this invoice in. Defaults to that of `customer` if not specified."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer who will be billed."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn customer_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn customer_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn customer_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn customer_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn customer_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_address_state` after provisioning.\nState, county, province, or region."]
    pub fn customer_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_email` after provisioning.\nThe customer's email. Until the invoice is finalized, this field will equal `customer.email`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_name` after provisioning.\nThe customer's name. Until the invoice is finalized, this field will equal `customer.name`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_phone` after provisioning.\nThe customer's phone number. Until the invoice is finalized, this field will equal `customer.phone`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn customer_shipping_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn customer_shipping_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn customer_shipping_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn customer_shipping_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn customer_shipping_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_address_state` after provisioning.\nState, county, province, or region."]
    pub fn customer_shipping_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_address_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_carrier` after provisioning.\nThe delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub fn customer_shipping_carrier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_shipping_carrier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_shipping_name` after provisioning.\nRecipient name."]
    pub fn customer_shipping_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_shipping_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_phone` after provisioning.\nRecipient phone (including extension)."]
    pub fn customer_shipping_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_shipping_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_shipping_tracking_number` after provisioning.\nThe tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas."]
    pub fn customer_shipping_tracking_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_shipping_tracking_number", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `customer_tax_exempt` after provisioning.\nThe customer's tax exempt status. Until the invoice is finalized, this field will equal `customer.tax_exempt`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_tax_exempt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_tax_exempt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer_tax_ids` after provisioning.\nThe customer's tax IDs. Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`. Once the invoice is finalized, this field will no longer be updated."]
    pub fn customer_tax_ids(&self) -> terrars::ListRef<Invoice_CustomerTaxIdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_tax_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `days_until_due` after provisioning.\nThe number of days from when the invoice is created until it is due. Valid only for invoices where `collection_method=send_invoice`."]
    pub fn days_until_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_until_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_payment_method` after provisioning.\nID of the default payment method for the invoice. It must belong to the customer associated with the invoice. If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings."]
    pub fn default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_payment_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_source` after provisioning.\nID of the default payment source for the invoice. It must belong to the customer associated with the invoice and be in a chargeable state. If not set, defaults to the subscription's default source, if any, or to the customer's default source."]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_tax_rates` after provisioning.\nThe tax rates that will apply to any line item that does not have `tax_rates` set."]
    pub fn default_tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_tax_rates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string attached to the object. Often useful for displaying to users. Referenced as 'memo' in the Dashboard."]
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
    pub fn discount_coupon_currency_options(&self) -> terrars::ListRef<Invoice_DiscountCouponCurrencyOptionsRef> {
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
        "Get a reference to the value of field `due_date` after provisioning.\nThe date on which payment for this invoice is due. Valid only for invoices where `collection_method=send_invoice`."]
    pub fn due_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.due_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ending_balance` after provisioning.\nEnding customer balance after the invoice is finalized. Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice. If the invoice has not been finalized yet, this will be null."]
    pub fn ending_balance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ending_balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `footer` after provisioning.\nFooter to be displayed on the invoice."]
    pub fn footer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.footer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_invoice_action` after provisioning.\nThe relation between this invoice and the cloned invoice"]
    pub fn from_invoice_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_invoice_action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `from_invoice_invoice` after provisioning.\nThe invoice that was cloned."]
    pub fn from_invoice_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_invoice_invoice", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hosted_invoice_url` after provisioning.\nThe URL for the hosted invoice page, which allows customers to view and pay an invoice. If the invoice has not been finalized yet, this will be null."]
    pub fn hosted_invoice_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_invoice_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object. This property is always present unless the invoice is an upcoming invoice. See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_pdf` after provisioning.\nThe link to download the PDF for the invoice. If the invoice has not been finalized yet, this will be null."]
    pub fn invoice_pdf(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_pdf", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_revision` after provisioning.\nThe ID of the most recent non-draft revision of this invoice"]
    pub fn latest_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lines_data` after provisioning.\nDetails about each object."]
    pub fn lines_data(&self) -> terrars::ListRef<Invoice_LinesDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lines_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lines_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn lines_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lines_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lines_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn lines_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lines_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lines_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn lines_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lines_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_payment_attempt` after provisioning.\nThe time at which payment will next be attempted. This value will be `null` for invoices where `collection_method=send_invoice`."]
    pub fn next_payment_attempt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_payment_attempt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number` after provisioning.\nA unique, identifying string that appears on emails sent to the customer for this invoice. This starts with the customer's unique invoice_prefix if it is specified."]
    pub fn number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `on_behalf_of` after provisioning.\nThe account (if any) for which the funds of the invoice payment are intended. If set, the invoice will be presented with the branding and support information of the specified account. See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details."]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paid` after provisioning.\nWhether payment was successfully collected for this invoice. An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance."]
    pub fn paid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `paid_out_of_band` after provisioning.\nReturns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe."]
    pub fn paid_out_of_band(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.paid_out_of_band", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_intent` after provisioning.\nThe PaymentIntent associated with this invoice. The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice. Note that voiding an invoice will cancel the PaymentIntent."]
    pub fn payment_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_intent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_default_mandate` after provisioning.\nID of the mandate to be used for this invoice. It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set."]
    pub fn payment_settings_default_mandate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_default_mandate", self.extract_ref()),
        )
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
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_enabled` after provisioning.\nWhether Installments are enabled for this Invoice."]
    pub fn payment_settings_payment_method_options_card_installments_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_plan_count` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_installments_plan_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_plan_count", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_plan_interval` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_installments_plan_interval(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_plan_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_installments_plan_type` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_installments_plan_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_installments_plan_type", self.extract_ref()),
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
        "Get a reference to the value of field `payment_settings_payment_method_types` after provisioning.\nThe list of payment method types (e.g. card) to provide to the invoice’s PaymentIntent. If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice)."]
    pub fn payment_settings_payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_items_behavior` after provisioning.\nHow to handle pending invoice items on invoice creation. One of `include` or `exclude`. `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist. `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not. Defaults to `exclude` if the parameter is omitted."]
    pub fn pending_invoice_items_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_items_behavior", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `period_end` after provisioning.\nEnd of the usage period during which invoice items were added to this invoice."]
    pub fn period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `period_start` after provisioning.\nStart of the usage period during which invoice items were added to this invoice."]
    pub fn period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `post_payment_credit_notes_amount` after provisioning.\nTotal amount of all post-payment credit notes issued for this invoice."]
    pub fn post_payment_credit_notes_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.post_payment_credit_notes_amount", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `pre_payment_credit_notes_amount` after provisioning.\nTotal amount of all pre-payment credit notes issued for this invoice."]
    pub fn pre_payment_credit_notes_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pre_payment_credit_notes_amount", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `quote` after provisioning.\nThe quote this invoice was generated from."]
    pub fn quote(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quote", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `receipt_number` after provisioning.\nThis is the transaction number that appears on email receipts sent for this invoice."]
    pub fn receipt_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.receipt_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rendering_options_amount_tax_display` after provisioning.\nHow line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub fn rendering_options_amount_tax_display(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.rendering_options_amount_tax_display", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `starting_balance` after provisioning.\nStarting customer balance before the invoice is finalized. If the invoice has not been finalized yet, this will be the current customer balance. For revision invoices, this also includes any customer balance that was applied to the original invoice."]
    pub fn starting_balance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.starting_balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `statement_descriptor` after provisioning.\nExtra information about a charge for the customer's credit card statement. It must contain at least one letter. If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`."]
    pub fn statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_descriptor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`. [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview)"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_finalized_at` after provisioning.\nThe time that the invoice draft was finalized."]
    pub fn status_transitions_finalized_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_transitions_finalized_at", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_marked_uncollectible_at` after provisioning.\nThe time that the invoice was marked uncollectible."]
    pub fn status_transitions_marked_uncollectible_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.status_transitions_marked_uncollectible_at", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_paid_at` after provisioning.\nThe time that the invoice was paid."]
    pub fn status_transitions_paid_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_transitions_paid_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status_transitions_voided_at` after provisioning.\nThe time that the invoice was voided."]
    pub fn status_transitions_voided_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_transitions_voided_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nThe ID of the subscription to invoice, if any. If set, the created invoice will only include pending invoice items for that subscription and pending invoice items not associated with any subscription if `pending_invoice_items_behavior` is `include`. The subscription's billing cycle and regular subscription events won't be affected."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_proration_date` after provisioning.\nOnly set for upcoming invoices that preview prorations. The time used to calculate prorations."]
    pub fn subscription_proration_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_proration_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subtotal` after provisioning.\nTotal of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied. Item discounts are already incorporated"]
    pub fn subtotal(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtotal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subtotal_excluding_tax` after provisioning.\nThe integer amount in %s representing the subtotal of the invoice before any invoice level discount or tax is applied. Item discounts are already incorporated"]
    pub fn subtotal_excluding_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtotal_excluding_tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax` after provisioning.\nThe amount of tax on this invoice. This is the sum of all the tax amounts on this invoice."]
    pub fn tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this invoice belongs to."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold_reason_amount_gte` after provisioning.\nThe total invoice amount threshold boundary if it triggered the threshold invoice."]
    pub fn threshold_reason_amount_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_reason_amount_gte", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threshold_reason_item_reasons` after provisioning.\nIndicates which line items triggered a threshold invoice."]
    pub fn threshold_reason_item_reasons(&self) -> terrars::ListRef<Invoice_ThresholdReasonItemReasonsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_reason_item_reasons", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total` after provisioning.\nTotal after discounts and taxes."]
    pub fn total(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_discount_amounts` after provisioning.\nThe aggregate amounts calculated per discount across all line items."]
    pub fn total_discount_amounts(&self) -> terrars::ListRef<Invoice_TotalDiscountAmountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_discount_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_excluding_tax` after provisioning.\nThe integer amount in %s representing the total amount of the invoice including all discounts but excluding all tax."]
    pub fn total_excluding_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_excluding_tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_tax_amounts` after provisioning.\nThe aggregate amounts calculated per tax rate for all line items."]
    pub fn total_tax_amounts(&self) -> terrars::ListRef<Invoice_TotalTaxAmountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_tax_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_amount` after provisioning.\nThe amount in %s that will be transferred to the destination account when the invoice is paid. By default, the entire amount is transferred to the destination."]
    pub fn transfer_data_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `transfer_data_destination` after provisioning.\nThe account where funds from the payment will be transferred to upon payment success."]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `webhooks_delivered_at` after provisioning.\nInvoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand). This field tracks the time when webhooks for this invoice were successfully delivered. If the invoice had no webhooks to deliver, this will be set while the invoice is being created."]
    pub fn webhooks_delivered_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.webhooks_delivered_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_fields` after provisioning.\n"]
    pub fn custom_fields(&self) -> terrars::ListRef<Invoice_CustomFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> terrars::ListRef<Invoice_DiscountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discounts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Invoice_CustomerTaxIds {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_CustomerTaxIds {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_CustomerTaxIds {
    type O = terrars::BlockAssignable<Invoice_CustomerTaxIds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_CustomerTaxIds {}

impl BuildInvoice_CustomerTaxIds {
    pub fn build(self) -> Invoice_CustomerTaxIds {
        Invoice_CustomerTaxIds {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_CustomerTaxIdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_CustomerTaxIdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_CustomerTaxIdsRef {
        Invoice_CustomerTaxIdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_CustomerTaxIdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_DiscountCouponCurrencyOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_DiscountCouponCurrencyOptions {
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

impl terrars::ToListMappable for Invoice_DiscountCouponCurrencyOptions {
    type O = terrars::BlockAssignable<Invoice_DiscountCouponCurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_DiscountCouponCurrencyOptions {}

impl BuildInvoice_DiscountCouponCurrencyOptions {
    pub fn build(self) -> Invoice_DiscountCouponCurrencyOptions {
        Invoice_DiscountCouponCurrencyOptions {
            amount_off: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_DiscountCouponCurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_DiscountCouponCurrencyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_DiscountCouponCurrencyOptionsRef {
        Invoice_DiscountCouponCurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_DiscountCouponCurrencyOptionsRef {
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
pub struct Invoice_LinesData_DiscountAmounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_LinesData_DiscountAmounts {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount`.\n"]
    pub fn set_discount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_LinesData_DiscountAmounts {
    type O = terrars::BlockAssignable<Invoice_LinesData_DiscountAmounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData_DiscountAmounts {}

impl BuildInvoice_LinesData_DiscountAmounts {
    pub fn build(self) -> Invoice_LinesData_DiscountAmounts {
        Invoice_LinesData_DiscountAmounts {
            amount: core::default::Default::default(),
            discount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_LinesData_DiscountAmountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesData_DiscountAmountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_LinesData_DiscountAmountsRef {
        Invoice_LinesData_DiscountAmountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesData_DiscountAmountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `discount` after provisioning.\n"]
    pub fn discount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_LinesData_PriceCurrencyOptions_Tiers {
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

impl Invoice_LinesData_PriceCurrencyOptions_Tiers {
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

impl terrars::ToListMappable for Invoice_LinesData_PriceCurrencyOptions_Tiers {
    type O = terrars::BlockAssignable<Invoice_LinesData_PriceCurrencyOptions_Tiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData_PriceCurrencyOptions_Tiers {}

impl BuildInvoice_LinesData_PriceCurrencyOptions_Tiers {
    pub fn build(self) -> Invoice_LinesData_PriceCurrencyOptions_Tiers {
        Invoice_LinesData_PriceCurrencyOptions_Tiers {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_LinesData_PriceCurrencyOptions_TiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesData_PriceCurrencyOptions_TiersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Invoice_LinesData_PriceCurrencyOptions_TiersRef {
        Invoice_LinesData_PriceCurrencyOptions_TiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesData_PriceCurrencyOptions_TiersRef {
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
pub struct Invoice_LinesData_PriceCurrencyOptions {
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
    tiers: std::option::Option<terrars::ListField<Invoice_LinesData_PriceCurrencyOptions_Tiers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_LinesData_PriceCurrencyOptions {
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
        v: impl std::convert::Into<terrars::ListField<Invoice_LinesData_PriceCurrencyOptions_Tiers>>,
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

impl terrars::ToListMappable for Invoice_LinesData_PriceCurrencyOptions {
    type O = terrars::BlockAssignable<Invoice_LinesData_PriceCurrencyOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData_PriceCurrencyOptions {}

impl BuildInvoice_LinesData_PriceCurrencyOptions {
    pub fn build(self) -> Invoice_LinesData_PriceCurrencyOptions {
        Invoice_LinesData_PriceCurrencyOptions {
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
pub struct Invoice_LinesData_PriceCurrencyOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesData_PriceCurrencyOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_LinesData_PriceCurrencyOptionsRef {
        Invoice_LinesData_PriceCurrencyOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesData_PriceCurrencyOptionsRef {
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
    pub fn tiers(&self) -> terrars::ListRef<Invoice_LinesData_PriceCurrencyOptions_TiersRef> {
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
pub struct Invoice_LinesData_PriceTiers {
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

impl Invoice_LinesData_PriceTiers {
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

impl terrars::ToListMappable for Invoice_LinesData_PriceTiers {
    type O = terrars::BlockAssignable<Invoice_LinesData_PriceTiers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData_PriceTiers {}

impl BuildInvoice_LinesData_PriceTiers {
    pub fn build(self) -> Invoice_LinesData_PriceTiers {
        Invoice_LinesData_PriceTiers {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_LinesData_PriceTiersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesData_PriceTiersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_LinesData_PriceTiersRef {
        Invoice_LinesData_PriceTiersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesData_PriceTiersRef {
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
pub struct Invoice_LinesData_TaxAmounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_LinesData_TaxAmounts {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusive`.\n"]
    pub fn set_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_rate`.\n"]
    pub fn set_tax_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_LinesData_TaxAmounts {
    type O = terrars::BlockAssignable<Invoice_LinesData_TaxAmounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData_TaxAmounts {}

impl BuildInvoice_LinesData_TaxAmounts {
    pub fn build(self) -> Invoice_LinesData_TaxAmounts {
        Invoice_LinesData_TaxAmounts {
            amount: core::default::Default::default(),
            inclusive: core::default::Default::default(),
            tax_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_LinesData_TaxAmountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesData_TaxAmountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_LinesData_TaxAmountsRef {
        Invoice_LinesData_TaxAmountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesData_TaxAmountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusive` after provisioning.\n"]
    pub fn inclusive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_rate` after provisioning.\n"]
    pub fn tax_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_LinesData_TaxRates {
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

impl Invoice_LinesData_TaxRates {
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

impl terrars::ToListMappable for Invoice_LinesData_TaxRates {
    type O = terrars::BlockAssignable<Invoice_LinesData_TaxRates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData_TaxRates {}

impl BuildInvoice_LinesData_TaxRates {
    pub fn build(self) -> Invoice_LinesData_TaxRates {
        Invoice_LinesData_TaxRates {
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
pub struct Invoice_LinesData_TaxRatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesData_TaxRatesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_LinesData_TaxRatesRef {
        Invoice_LinesData_TaxRatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesData_TaxRatesRef {
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
pub struct Invoice_LinesData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_excluding_tax: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_amounts: std::option::Option<terrars::ListField<Invoice_LinesData_DiscountAmounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discountable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice_item: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_start: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_active: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_billing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_currency_options: std::option::Option<terrars::ListField<Invoice_LinesData_PriceCurrencyOptions>>,
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
    price_tiers: std::option::Option<terrars::ListField<Invoice_LinesData_PriceTiers>>,
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
    proration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proration_details_credited_items_invoice: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proration_details_credited_items_invoice_line_items: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quantity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription_item: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_amounts: std::option::Option<terrars::ListField<Invoice_LinesData_TaxAmounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rates: std::option::Option<terrars::ListField<Invoice_LinesData_TaxRates>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_excluding_tax: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_LinesData {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amount_excluding_tax`.\n"]
    pub fn set_amount_excluding_tax(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount_excluding_tax = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `currency`.\n"]
    pub fn set_currency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_amounts`.\n"]
    pub fn set_discount_amounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Invoice_LinesData_DiscountAmounts>>,
    ) -> Self {
        self.discount_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discountable`.\n"]
    pub fn set_discountable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.discountable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discounts`.\n"]
    pub fn set_discounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.discounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invoice_item`.\n"]
    pub fn set_invoice_item(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.invoice_item = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `period_end`.\n"]
    pub fn set_period_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_start`.\n"]
    pub fn set_period_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_start = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<Invoice_LinesData_PriceCurrencyOptions>>,
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
        v: impl std::convert::Into<terrars::ListField<Invoice_LinesData_PriceTiers>>,
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

    #[doc = "Set the field `proration`.\n"]
    pub fn set_proration(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proration_details_credited_items_invoice`.\n"]
    pub fn set_proration_details_credited_items_invoice(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.proration_details_credited_items_invoice = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proration_details_credited_items_invoice_line_items`.\n"]
    pub fn set_proration_details_credited_items_invoice_line_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.proration_details_credited_items_invoice_line_items = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `subscription_item`.\n"]
    pub fn set_subscription_item(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subscription_item = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_amounts`.\n"]
    pub fn set_tax_amounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Invoice_LinesData_TaxAmounts>>,
    ) -> Self {
        self.tax_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_rates`.\n"]
    pub fn set_tax_rates(mut self, v: impl std::convert::Into<terrars::ListField<Invoice_LinesData_TaxRates>>) -> Self {
        self.tax_rates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_amount_excluding_tax`.\n"]
    pub fn set_unit_amount_excluding_tax(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unit_amount_excluding_tax = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_LinesData {
    type O = terrars::BlockAssignable<Invoice_LinesData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_LinesData {}

impl BuildInvoice_LinesData {
    pub fn build(self) -> Invoice_LinesData {
        Invoice_LinesData {
            amount: core::default::Default::default(),
            amount_excluding_tax: core::default::Default::default(),
            currency: core::default::Default::default(),
            description: core::default::Default::default(),
            discount_amounts: core::default::Default::default(),
            discountable: core::default::Default::default(),
            discounts: core::default::Default::default(),
            id: core::default::Default::default(),
            invoice_item: core::default::Default::default(),
            livemode: core::default::Default::default(),
            metadata: core::default::Default::default(),
            object: core::default::Default::default(),
            period_end: core::default::Default::default(),
            period_start: core::default::Default::default(),
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
            proration: core::default::Default::default(),
            proration_details_credited_items_invoice: core::default::Default::default(),
            proration_details_credited_items_invoice_line_items: core::default::Default::default(),
            quantity: core::default::Default::default(),
            subscription: core::default::Default::default(),
            subscription_item: core::default::Default::default(),
            tax_amounts: core::default::Default::default(),
            tax_rates: core::default::Default::default(),
            type_: core::default::Default::default(),
            unit_amount_excluding_tax: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_LinesDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_LinesDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_LinesDataRef {
        Invoice_LinesDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_LinesDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `amount_excluding_tax` after provisioning.\n"]
    pub fn amount_excluding_tax(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount_excluding_tax", self.base))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\n"]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_amounts` after provisioning.\n"]
    pub fn discount_amounts(&self) -> terrars::ListRef<Invoice_LinesData_DiscountAmountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discount_amounts", self.base))
    }

    #[doc = "Get a reference to the value of field `discountable` after provisioning.\n"]
    pub fn discountable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discountable", self.base))
    }

    #[doc = "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discounts", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `invoice_item` after provisioning.\n"]
    pub fn invoice_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_item", self.base))
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

    #[doc = "Get a reference to the value of field `period_end` after provisioning.\n"]
    pub fn period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.base))
    }

    #[doc = "Get a reference to the value of field `period_start` after provisioning.\n"]
    pub fn period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.base))
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
    pub fn price_currency_options(&self) -> terrars::ListRef<Invoice_LinesData_PriceCurrencyOptionsRef> {
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
    pub fn price_tiers(&self) -> terrars::ListRef<Invoice_LinesData_PriceTiersRef> {
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

    #[doc = "Get a reference to the value of field `proration` after provisioning.\n"]
    pub fn proration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proration", self.base))
    }

    #[doc = "Get a reference to the value of field `proration_details_credited_items_invoice` after provisioning.\n"]
    pub fn proration_details_credited_items_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.proration_details_credited_items_invoice", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `proration_details_credited_items_invoice_line_items` after provisioning.\n"]
    pub fn proration_details_credited_items_invoice_line_items(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.proration_details_credited_items_invoice_line_items", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription` after provisioning.\n"]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription_item` after provisioning.\n"]
    pub fn subscription_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_item", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_amounts` after provisioning.\n"]
    pub fn tax_amounts(&self) -> terrars::ListRef<Invoice_LinesData_TaxAmountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_amounts", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_rates` after provisioning.\n"]
    pub fn tax_rates(&self) -> terrars::ListRef<Invoice_LinesData_TaxRatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount_excluding_tax` after provisioning.\n"]
    pub fn unit_amount_excluding_tax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_excluding_tax", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_ThresholdReasonItemReasons {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    line_item_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_gte: std::option::Option<terrars::PrimField<f64>>,
}

impl Invoice_ThresholdReasonItemReasons {
    #[doc = "Set the field `line_item_ids`.\n"]
    pub fn set_line_item_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.line_item_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_gte`.\n"]
    pub fn set_usage_gte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.usage_gte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_ThresholdReasonItemReasons {
    type O = terrars::BlockAssignable<Invoice_ThresholdReasonItemReasons>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_ThresholdReasonItemReasons {}

impl BuildInvoice_ThresholdReasonItemReasons {
    pub fn build(self) -> Invoice_ThresholdReasonItemReasons {
        Invoice_ThresholdReasonItemReasons {
            line_item_ids: core::default::Default::default(),
            usage_gte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_ThresholdReasonItemReasonsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_ThresholdReasonItemReasonsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_ThresholdReasonItemReasonsRef {
        Invoice_ThresholdReasonItemReasonsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_ThresholdReasonItemReasonsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `line_item_ids` after provisioning.\n"]
    pub fn line_item_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.line_item_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_gte` after provisioning.\n"]
    pub fn usage_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_gte", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_TotalDiscountAmounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_TotalDiscountAmounts {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount`.\n"]
    pub fn set_discount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_TotalDiscountAmounts {
    type O = terrars::BlockAssignable<Invoice_TotalDiscountAmounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_TotalDiscountAmounts {}

impl BuildInvoice_TotalDiscountAmounts {
    pub fn build(self) -> Invoice_TotalDiscountAmounts {
        Invoice_TotalDiscountAmounts {
            amount: core::default::Default::default(),
            discount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_TotalDiscountAmountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_TotalDiscountAmountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_TotalDiscountAmountsRef {
        Invoice_TotalDiscountAmountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_TotalDiscountAmountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `discount` after provisioning.\n"]
    pub fn discount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_TotalTaxAmounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inclusive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_TotalTaxAmounts {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inclusive`.\n"]
    pub fn set_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_rate`.\n"]
    pub fn set_tax_rate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_rate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_TotalTaxAmounts {
    type O = terrars::BlockAssignable<Invoice_TotalTaxAmounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_TotalTaxAmounts {}

impl BuildInvoice_TotalTaxAmounts {
    pub fn build(self) -> Invoice_TotalTaxAmounts {
        Invoice_TotalTaxAmounts {
            amount: core::default::Default::default(),
            inclusive: core::default::Default::default(),
            tax_rate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_TotalTaxAmountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_TotalTaxAmountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_TotalTaxAmountsRef {
        Invoice_TotalTaxAmountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_TotalTaxAmountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `inclusive` after provisioning.\n"]
    pub fn inclusive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_rate` after provisioning.\n"]
    pub fn tax_rate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_rate", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_CustomFields {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Invoice_CustomFields { }

impl terrars::ToListMappable for Invoice_CustomFields {
    type O = terrars::BlockAssignable<Invoice_CustomFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_CustomFields {
    #[doc = "The name of the custom field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the custom field."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInvoice_CustomFields {
    pub fn build(self) -> Invoice_CustomFields {
        Invoice_CustomFields {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_CustomFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_CustomFieldsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_CustomFieldsRef {
        Invoice_CustomFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_CustomFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the custom field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the custom field."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct Invoice_Discounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coupon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoice_Discounts {
    #[doc = "Set the field `coupon`.\n"]
    pub fn set_coupon(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.coupon = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount`.\n"]
    pub fn set_discount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Invoice_Discounts {
    type O = terrars::BlockAssignable<Invoice_Discounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoice_Discounts {}

impl BuildInvoice_Discounts {
    pub fn build(self) -> Invoice_Discounts {
        Invoice_Discounts {
            coupon: core::default::Default::default(),
            discount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoice_DiscountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoice_DiscountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoice_DiscountsRef {
        Invoice_DiscountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoice_DiscountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `checkout_session` after provisioning.\nThe Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode."]
    pub fn checkout_session(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.checkout_session", self.base))
    }

    #[doc = "Get a reference to the value of field `coupon` after provisioning.\n"]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.base))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer associated with this discount."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\nAlways true for a deleted object"]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `discount` after provisioning.\n"]
    pub fn discount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invoice` after provisioning.\nThe invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice."]
    pub fn invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invoice_item` after provisioning.\nThe invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item."]
    pub fn invoice_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_item", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc =
        "Get a reference to the value of field `promotion_code` after provisioning.\nThe promotion code applied to create this discount."]
    pub fn promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promotion_code", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\nDate that the coupon was applied."]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nThe subscription that this coupon is applied to, if it is applied to a particular subscription."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }
}

#[derive(Serialize, Default)]
struct InvoiceDynamic {
    custom_fields: std::option::Option<terrars::DynamicBlock<Invoice_CustomFields>>,
    discounts: std::option::Option<terrars::DynamicBlock<Invoice_Discounts>>,
}
