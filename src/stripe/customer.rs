use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct CustomerData {
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
    address_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    balance: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cash_balance_settings_reconciliation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coupon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice_settings_default_payment_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice_settings_footer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice_settings_rendering_options_amount_tax_display: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_invoice_sequence: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_locales: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    promotion_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_address_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shipping_phone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_exempt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_clock: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice_settings_custom_fields: std::option::Option<std::vec::Vec<CustomerInvoiceSettingsCustomFieldsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_id_data: std::option::Option<std::vec::Vec<CustomerTaxIdDataEl>>,
    dynamic: CustomerDynamic,
}

struct Customer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CustomerData>,
}

#[derive(Clone)]
pub struct Customer(std::rc::Rc<Customer_>);

impl Customer {
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

    #[doc = "Set the field `address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_address_city(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `address_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_address_country(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_address_line1(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_address_line2(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_postal_code`.\nZIP or postal code."]
    pub fn set_address_postal_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address_state`.\nState, county, province, or region."]
    pub fn set_address_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().address_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `balance`.\nAn integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices. A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice."]
    pub fn set_balance(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().balance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cash_balance_settings_reconciliation_mode`.\nThe configuration for how funds that land in the customer cash balance are reconciled."]
    pub fn set_cash_balance_settings_reconciliation_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cash_balance_settings_reconciliation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `coupon`.\n"]
    pub fn set_coupon(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().coupon = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn arbitrary string that you can attach to a customer object. It is displayed alongside the customer in the dashboard."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `email`.\nCustomer's email address. It's displayed alongside the customer in your dashboard and can be useful for searching and tracking. This may be up to *512 characters*."]
    pub fn set_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invoice_prefix`.\nThe prefix for the customer used to generate unique invoice numbers. Must be 3–12 uppercase letters or numbers."]
    pub fn set_invoice_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().invoice_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invoice_settings_default_payment_method`.\nID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices."]
    pub fn set_invoice_settings_default_payment_method(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().invoice_settings_default_payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invoice_settings_footer`.\nDefault footer to be displayed on invoices for this customer."]
    pub fn set_invoice_settings_footer(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().invoice_settings_footer = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invoice_settings_rendering_options_amount_tax_display`.\nHow line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub fn set_invoice_settings_rendering_options_amount_tax_display(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().invoice_settings_rendering_options_amount_tax_display =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe customer's full name or business name."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `next_invoice_sequence`.\nThe sequence to be used on the customer's next invoice. Defaults to 1."]
    pub fn set_next_invoice_sequence(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().next_invoice_sequence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_method`.\n"]
    pub fn set_payment_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone`.\nThe customer's phone number."]
    pub fn set_phone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().phone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_locales`.\nCustomer's preferred languages, ordered by preference."]
    pub fn set_preferred_locales(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().preferred_locales = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `promotion_code`.\nThe API ID of a promotion code to apply to the customer. The customer will have a discount applied on all recurring payments. Charges you create through the API will not have the discount."]
    pub fn set_promotion_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().promotion_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_shipping_address_city(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shipping_address_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shipping_address_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_shipping_address_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shipping_address_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_shipping_address_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shipping_address_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_shipping_address_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shipping_address_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_address_postal_code`.\nZIP or postal code."]
    pub fn set_shipping_address_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shipping_address_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_address_state`.\nState, county, province, or region."]
    pub fn set_shipping_address_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().shipping_address_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_name`.\nRecipient name."]
    pub fn set_shipping_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shipping_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shipping_phone`.\nRecipient phone (including extension)."]
    pub fn set_shipping_phone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().shipping_phone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_exempt`.\nThe customer's tax exemption. One of `none`, `exempt`, or `reverse`."]
    pub fn set_tax_exempt(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_exempt = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_ip_address`.\nA recent IP address of the customer used for tax reporting and tax location inference."]
    pub fn set_tax_ip_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_clock`.\nID of the test clock to attach to the customer."]
    pub fn set_test_clock(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().test_clock = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invoice_settings_custom_fields`.\n"]
    pub fn set_invoice_settings_custom_fields(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerInvoiceSettingsCustomFieldsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().invoice_settings_custom_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.invoice_settings_custom_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tax_id_data`.\n"]
    pub fn set_tax_id_data(self, v: impl std::convert::Into<terrars::BlockAssignable<CustomerTaxIdDataEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tax_id_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tax_id_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_state` after provisioning.\nState, county, province, or region."]
    pub fn address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `balance` after provisioning.\nAn integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices. A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice."]
    pub fn balance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_available` after provisioning.\nA hash of all cash balances available to this customer. You cannot delete a customer with any cash balances, even if the balance is 0. Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn cash_balance_available(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.cash_balance_available", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_customer` after provisioning.\nThe ID of the customer whose cash balance this object represents."]
    pub fn cash_balance_customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cash_balance_customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn cash_balance_livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cash_balance_livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn cash_balance_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cash_balance_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_settings_reconciliation_mode` after provisioning.\nThe configuration for how funds that land in the customer cash balance are reconciled."]
    pub fn cash_balance_settings_reconciliation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cash_balance_settings_reconciliation_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `coupon` after provisioning.\n"]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_source` after provisioning.\nID of the default payment source for the customer.\n\nIf you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead."]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delinquent` after provisioning.\nWhen the customer's latest invoice is billed by charging automatically, `delinquent` is `true` if the invoice's latest charge failed. When the customer's latest invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't paid by its due date.\n\nIf an invoice is marked uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't get reset to `false`."]
    pub fn delinquent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delinquent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string that you can attach to a customer object. It is displayed alongside the customer in the dashboard."]
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
    pub fn discount_coupon_currency_options(&self) -> terrars::ListRef<CustomerDiscountCouponCurrencyOptionsElRef> {
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
        "Get a reference to the value of field `email` after provisioning.\nCustomer's email address. It's displayed alongside the customer in your dashboard and can be useful for searching and tracking. This may be up to *512 characters*."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_credit_balance` after provisioning.\nThe current multi-currency balances, if any, being stored on the customer. If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency. If negative, the customer has an amount owed that will be added to their next invoice denominated in that currency. These balances do not refer to any unpaid invoices. They solely track amounts that have yet to be successfully applied to any invoice. A balance in a particular currency is only applied to any invoice as an invoice in that currency is finalized."]
    pub fn invoice_credit_balance(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.invoice_credit_balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_prefix` after provisioning.\nThe prefix for the customer used to generate unique invoice numbers. Must be 3–12 uppercase letters or numbers."]
    pub fn invoice_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_settings_default_payment_method` after provisioning.\nID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices."]
    pub fn invoice_settings_default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoice_settings_default_payment_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `invoice_settings_footer` after provisioning.\nDefault footer to be displayed on invoices for this customer."]
    pub fn invoice_settings_footer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_settings_footer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_settings_rendering_options_amount_tax_display` after provisioning.\nHow line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub fn invoice_settings_rendering_options_amount_tax_display(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoice_settings_rendering_options_amount_tax_display", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe customer's full name or business name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_invoice_sequence` after provisioning.\nThe sequence to be used on the customer's next invoice. Defaults to 1."]
    pub fn next_invoice_sequence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_invoice_sequence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payment_method` after provisioning.\n"]
    pub fn payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone` after provisioning.\nThe customer's phone number."]
    pub fn phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preferred_locales` after provisioning.\nCustomer's preferred languages, ordered by preference."]
    pub fn preferred_locales(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_locales", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `promotion_code` after provisioning.\nThe API ID of a promotion code to apply to the customer. The customer will have a discount applied on all recurring payments. Charges you create through the API will not have the discount."]
    pub fn promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promotion_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn shipping_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn shipping_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn shipping_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn shipping_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn shipping_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_state` after provisioning.\nState, county, province, or region."]
    pub fn shipping_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_carrier` after provisioning.\nThe delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub fn shipping_carrier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_carrier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shipping_name` after provisioning.\nRecipient name."]
    pub fn shipping_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_phone` after provisioning.\nRecipient phone (including extension)."]
    pub fn shipping_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_tracking_number` after provisioning.\nThe tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas."]
    pub fn shipping_tracking_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_tracking_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sources_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn sources_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sources_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sources_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn sources_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sources_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sources_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn sources_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sources_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_data` after provisioning.\nDetails about each object."]
    pub fn subscriptions_data(&self) -> terrars::ListRef<CustomerSubscriptionsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subscriptions_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn subscriptions_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn subscriptions_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn subscriptions_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_automatic_tax` after provisioning.\nSurfaces if automatic tax computation is possible given the current customer location information."]
    pub fn tax_automatic_tax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_automatic_tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_exempt` after provisioning.\nThe customer's tax exemption. One of `none`, `exempt`, or `reverse`."]
    pub fn tax_exempt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_exempt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tax_ids_data` after provisioning.\nDetails about each object."]
    pub fn tax_ids_data(&self) -> terrars::ListRef<CustomerTaxIdsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_ids_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ids_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn tax_ids_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ids_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ids_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn tax_ids_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ids_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ids_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn tax_ids_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ids_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ip_address` after provisioning.\nA recent IP address of the customer used for tax reporting and tax location inference."]
    pub fn tax_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_location_country` after provisioning.\nThe customer's country as identified by Stripe Tax."]
    pub fn tax_location_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_location_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_location_source` after provisioning.\nThe data source used to infer the customer's location."]
    pub fn tax_location_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_location_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_location_state` after provisioning.\nThe customer's state, county, province, or region as identified by Stripe Tax."]
    pub fn tax_location_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_location_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock to attach to the customer."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoice_settings_custom_fields` after provisioning.\n"]
    pub fn invoice_settings_custom_fields(&self) -> terrars::ListRef<CustomerInvoiceSettingsCustomFieldsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.invoice_settings_custom_fields", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tax_id_data` after provisioning.\n"]
    pub fn tax_id_data(&self) -> terrars::ListRef<CustomerTaxIdDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_id_data", self.extract_ref()))
    }
}

impl terrars::Referable for Customer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Customer { }

impl terrars::ToListMappable for Customer {
    type O = terrars::ListRef<CustomerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Customer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_customer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomer {
    pub tf_id: std::string::String,
}

impl BuildCustomer {
    pub fn build(self, stack: &mut terrars::Stack) -> Customer {
        let out = Customer(std::rc::Rc::new(Customer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CustomerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                address_city: core::default::Default::default(),
                address_country: core::default::Default::default(),
                address_line1: core::default::Default::default(),
                address_line2: core::default::Default::default(),
                address_postal_code: core::default::Default::default(),
                address_state: core::default::Default::default(),
                balance: core::default::Default::default(),
                cash_balance_settings_reconciliation_mode: core::default::Default::default(),
                coupon: core::default::Default::default(),
                description: core::default::Default::default(),
                email: core::default::Default::default(),
                invoice_prefix: core::default::Default::default(),
                invoice_settings_default_payment_method: core::default::Default::default(),
                invoice_settings_footer: core::default::Default::default(),
                invoice_settings_rendering_options_amount_tax_display: core::default::Default::default(),
                name: core::default::Default::default(),
                next_invoice_sequence: core::default::Default::default(),
                payment_method: core::default::Default::default(),
                phone: core::default::Default::default(),
                preferred_locales: core::default::Default::default(),
                promotion_code: core::default::Default::default(),
                shipping_address_city: core::default::Default::default(),
                shipping_address_country: core::default::Default::default(),
                shipping_address_line1: core::default::Default::default(),
                shipping_address_line2: core::default::Default::default(),
                shipping_address_postal_code: core::default::Default::default(),
                shipping_address_state: core::default::Default::default(),
                shipping_name: core::default::Default::default(),
                shipping_phone: core::default::Default::default(),
                source: core::default::Default::default(),
                tax_exempt: core::default::Default::default(),
                tax_ip_address: core::default::Default::default(),
                test_clock: core::default::Default::default(),
                invoice_settings_custom_fields: core::default::Default::default(),
                tax_id_data: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `address_state` after provisioning.\nState, county, province, or region."]
    pub fn address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `balance` after provisioning.\nAn integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices. A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice."]
    pub fn balance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_available` after provisioning.\nA hash of all cash balances available to this customer. You cannot delete a customer with any cash balances, even if the balance is 0. Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal)."]
    pub fn cash_balance_available(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.cash_balance_available", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_customer` after provisioning.\nThe ID of the customer whose cash balance this object represents."]
    pub fn cash_balance_customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cash_balance_customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn cash_balance_livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cash_balance_livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn cash_balance_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cash_balance_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cash_balance_settings_reconciliation_mode` after provisioning.\nThe configuration for how funds that land in the customer cash balance are reconciled."]
    pub fn cash_balance_settings_reconciliation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.cash_balance_settings_reconciliation_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `coupon` after provisioning.\n"]
    pub fn coupon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_source` after provisioning.\nID of the default payment source for the customer.\n\nIf you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead."]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delinquent` after provisioning.\nWhen the customer's latest invoice is billed by charging automatically, `delinquent` is `true` if the invoice's latest charge failed. When the customer's latest invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't paid by its due date.\n\nIf an invoice is marked uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't get reset to `false`."]
    pub fn delinquent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delinquent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string that you can attach to a customer object. It is displayed alongside the customer in the dashboard."]
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
    pub fn discount_coupon_currency_options(&self) -> terrars::ListRef<CustomerDiscountCouponCurrencyOptionsElRef> {
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
        "Get a reference to the value of field `email` after provisioning.\nCustomer's email address. It's displayed alongside the customer in your dashboard and can be useful for searching and tracking. This may be up to *512 characters*."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_credit_balance` after provisioning.\nThe current multi-currency balances, if any, being stored on the customer. If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency. If negative, the customer has an amount owed that will be added to their next invoice denominated in that currency. These balances do not refer to any unpaid invoices. They solely track amounts that have yet to be successfully applied to any invoice. A balance in a particular currency is only applied to any invoice as an invoice in that currency is finalized."]
    pub fn invoice_credit_balance(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.invoice_credit_balance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_prefix` after provisioning.\nThe prefix for the customer used to generate unique invoice numbers. Must be 3–12 uppercase letters or numbers."]
    pub fn invoice_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_settings_default_payment_method` after provisioning.\nID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices."]
    pub fn invoice_settings_default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoice_settings_default_payment_method", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `invoice_settings_footer` after provisioning.\nDefault footer to be displayed on invoices for this customer."]
    pub fn invoice_settings_footer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice_settings_footer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice_settings_rendering_options_amount_tax_display` after provisioning.\nHow line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub fn invoice_settings_rendering_options_amount_tax_display(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.invoice_settings_rendering_options_amount_tax_display", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe customer's full name or business name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `next_invoice_sequence` after provisioning.\nThe sequence to be used on the customer's next invoice. Defaults to 1."]
    pub fn next_invoice_sequence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_invoice_sequence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payment_method` after provisioning.\n"]
    pub fn payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone` after provisioning.\nThe customer's phone number."]
    pub fn phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preferred_locales` after provisioning.\nCustomer's preferred languages, ordered by preference."]
    pub fn preferred_locales(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_locales", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `promotion_code` after provisioning.\nThe API ID of a promotion code to apply to the customer. The customer will have a discount applied on all recurring payments. Charges you create through the API will not have the discount."]
    pub fn promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.promotion_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn shipping_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn shipping_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn shipping_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn shipping_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn shipping_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_address_state` after provisioning.\nState, county, province, or region."]
    pub fn shipping_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_carrier` after provisioning.\nThe delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub fn shipping_carrier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_carrier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shipping_name` after provisioning.\nRecipient name."]
    pub fn shipping_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_phone` after provisioning.\nRecipient phone (including extension)."]
    pub fn shipping_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shipping_tracking_number` after provisioning.\nThe tracking number for a physical product, obtained from the delivery service. If multiple tracking numbers were generated for this purchase, please separate them with commas."]
    pub fn shipping_tracking_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shipping_tracking_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sources_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn sources_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sources_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sources_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn sources_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sources_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sources_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn sources_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sources_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_data` after provisioning.\nDetails about each object."]
    pub fn subscriptions_data(&self) -> terrars::ListRef<CustomerSubscriptionsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subscriptions_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn subscriptions_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn subscriptions_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscriptions_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn subscriptions_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriptions_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_automatic_tax` after provisioning.\nSurfaces if automatic tax computation is possible given the current customer location information."]
    pub fn tax_automatic_tax(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_automatic_tax", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_exempt` after provisioning.\nThe customer's tax exemption. One of `none`, `exempt`, or `reverse`."]
    pub fn tax_exempt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_exempt", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tax_ids_data` after provisioning.\nDetails about each object."]
    pub fn tax_ids_data(&self) -> terrars::ListRef<CustomerTaxIdsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_ids_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ids_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn tax_ids_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ids_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ids_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn tax_ids_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ids_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ids_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn tax_ids_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ids_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_ip_address` after provisioning.\nA recent IP address of the customer used for tax reporting and tax location inference."]
    pub fn tax_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_ip_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_location_country` after provisioning.\nThe customer's country as identified by Stripe Tax."]
    pub fn tax_location_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_location_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_location_source` after provisioning.\nThe data source used to infer the customer's location."]
    pub fn tax_location_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_location_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_location_state` after provisioning.\nThe customer's state, county, province, or region as identified by Stripe Tax."]
    pub fn tax_location_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_location_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock to attach to the customer."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invoice_settings_custom_fields` after provisioning.\n"]
    pub fn invoice_settings_custom_fields(&self) -> terrars::ListRef<CustomerInvoiceSettingsCustomFieldsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.invoice_settings_custom_fields", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tax_id_data` after provisioning.\n"]
    pub fn tax_id_data(&self) -> terrars::ListRef<CustomerTaxIdDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_id_data", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CustomerDiscountCouponCurrencyOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerDiscountCouponCurrencyOptionsEl {
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

impl terrars::ToListMappable for CustomerDiscountCouponCurrencyOptionsEl {
    type O = terrars::BlockAssignable<CustomerDiscountCouponCurrencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerDiscountCouponCurrencyOptionsEl {}

impl BuildCustomerDiscountCouponCurrencyOptionsEl {
    pub fn build(self) -> CustomerDiscountCouponCurrencyOptionsEl {
        CustomerDiscountCouponCurrencyOptionsEl {
            amount_off: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct CustomerDiscountCouponCurrencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerDiscountCouponCurrencyOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerDiscountCouponCurrencyOptionsElRef {
        CustomerDiscountCouponCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerDiscountCouponCurrencyOptionsElRef {
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
pub struct CustomerSubscriptionsDataElDefaultTaxRatesEl {
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

impl CustomerSubscriptionsDataElDefaultTaxRatesEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElDefaultTaxRatesEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElDefaultTaxRatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElDefaultTaxRatesEl {}

impl BuildCustomerSubscriptionsDataElDefaultTaxRatesEl {
    pub fn build(self) -> CustomerSubscriptionsDataElDefaultTaxRatesEl {
        CustomerSubscriptionsDataElDefaultTaxRatesEl {
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

pub struct CustomerSubscriptionsDataElDefaultTaxRatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElDefaultTaxRatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElDefaultTaxRatesElRef {
        CustomerSubscriptionsDataElDefaultTaxRatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElDefaultTaxRatesElRef {
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
pub struct CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {}

impl BuildCustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {
    pub fn build(self) -> CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {
        CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl {
            amount_off: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

pub struct CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsElRef {
        CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsElRef {
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
pub struct CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {
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

impl CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {}

impl BuildCustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {
    pub fn build(self) -> CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {
        CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

pub struct CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersElRef {
        CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersElRef {
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
pub struct CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {
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
    tiers: std::option::Option<terrars::ListField<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {
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
                    ::Into<terrars::ListField<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersEl>>,
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {}

impl BuildCustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {
    pub fn build(self) -> CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {
        CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl {
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

pub struct CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElRef {
        CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElRef {
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
    pub fn tiers(&self) -> terrars::ListRef<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElTiersElRef> {
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
pub struct CustomerSubscriptionsDataElItemsDataElPriceTiersEl {
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

impl CustomerSubscriptionsDataElItemsDataElPriceTiersEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElItemsDataElPriceTiersEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElItemsDataElPriceTiersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElItemsDataElPriceTiersEl {}

impl BuildCustomerSubscriptionsDataElItemsDataElPriceTiersEl {
    pub fn build(self) -> CustomerSubscriptionsDataElItemsDataElPriceTiersEl {
        CustomerSubscriptionsDataElItemsDataElPriceTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

pub struct CustomerSubscriptionsDataElItemsDataElPriceTiersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElItemsDataElPriceTiersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElItemsDataElPriceTiersElRef {
        CustomerSubscriptionsDataElItemsDataElPriceTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElItemsDataElPriceTiersElRef {
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
pub struct CustomerSubscriptionsDataElItemsDataElTaxRatesEl {
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

impl CustomerSubscriptionsDataElItemsDataElTaxRatesEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElItemsDataElTaxRatesEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElItemsDataElTaxRatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElItemsDataElTaxRatesEl {}

impl BuildCustomerSubscriptionsDataElItemsDataElTaxRatesEl {
    pub fn build(self) -> CustomerSubscriptionsDataElItemsDataElTaxRatesEl {
        CustomerSubscriptionsDataElItemsDataElTaxRatesEl {
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

pub struct CustomerSubscriptionsDataElItemsDataElTaxRatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElItemsDataElTaxRatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElItemsDataElTaxRatesElRef {
        CustomerSubscriptionsDataElItemsDataElTaxRatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElItemsDataElTaxRatesElRef {
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
pub struct CustomerSubscriptionsDataElItemsDataEl {
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
    ::Option<terrars::ListField<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl>>,
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
    price_tiers: std::option::Option<terrars::ListField<CustomerSubscriptionsDataElItemsDataElPriceTiersEl>>,
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
    tax_rates: std::option::Option<terrars::ListField<CustomerSubscriptionsDataElItemsDataElTaxRatesEl>>,
}

impl CustomerSubscriptionsDataElItemsDataEl {
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
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsEl>>,
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
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElItemsDataElPriceTiersEl>>,
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
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElItemsDataElTaxRatesEl>>,
    ) -> Self {
        self.tax_rates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerSubscriptionsDataElItemsDataEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElItemsDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElItemsDataEl {}

impl BuildCustomerSubscriptionsDataElItemsDataEl {
    pub fn build(self) -> CustomerSubscriptionsDataElItemsDataEl {
        CustomerSubscriptionsDataElItemsDataEl {
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

pub struct CustomerSubscriptionsDataElItemsDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElItemsDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerSubscriptionsDataElItemsDataElRef {
        CustomerSubscriptionsDataElItemsDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElItemsDataElRef {
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
    ) -> terrars::ListRef<CustomerSubscriptionsDataElItemsDataElPriceCurrencyOptionsElRef> {
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
    pub fn price_tiers(&self) -> terrars::ListRef<CustomerSubscriptionsDataElItemsDataElPriceTiersElRef> {
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
    pub fn tax_rates(&self) -> terrars::ListRef<CustomerSubscriptionsDataElItemsDataElTaxRatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {
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

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {
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

impl terrars
::ToListMappable for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {
    type O =
        terrars
        ::BlockAssignable<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {}

impl BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {
    pub fn build(self) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersElRef {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersElRef {
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
pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {
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
    ::Option<
        terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {
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
                        terrars
                        ::ListField<
                            CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersEl,
                        >,
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {
    type O =
        terrars::BlockAssignable<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {}

impl BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {
    pub fn build(self) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl {
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

pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElRef {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElRef {
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
    pub fn tiers(
        &self,
    ) -> terrars
    ::ListRef<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElTiersElRef> {
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
pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {
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

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {}

impl BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {
    pub fn build(self) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: core::default::Default::default(),
        }
    }
}

pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersElRef {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersElRef {
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
pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {
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

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {
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

impl terrars::ToListMappable for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {}

impl BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {
    pub fn build(self) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl {
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

pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesElRef {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesElRef {
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
pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {
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
    ::Option<terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl>>,
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
    price_tiers: std
    ::option
    ::Option<terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl>>,
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
    tax_rates: std
    ::option
    ::Option<terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl>>,
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {
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
                    ::Into<
                        terrars
                        ::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesEl>>,
    ) -> Self {
        self.tax_rates = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {}

impl BuildCustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {
    pub fn build(self) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl {
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

pub struct CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElRef {
        CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElRef {
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
    ) -> terrars::ListRef<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceCurrencyOptionsElRef> {
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
    pub fn price_tiers(
        &self,
    ) -> terrars::ListRef<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElPriceTiersElRef> {
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
    pub fn tax_rates(
        &self,
    ) -> terrars::ListRef<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElTaxRatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerSubscriptionsDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_fee_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_tax_enabled: std::option::Option<terrars::PrimField<bool>>,
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
    canceled_at: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_period_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_period_start: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_until_due: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_payment_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_tax_rates: std::option::Option<terrars::ListField<CustomerSubscriptionsDataElDefaultTaxRatesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_checkout_session: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_amount_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_applies_to_products: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_currency_options: std
    ::option
    ::Option<terrars::ListField<CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_duration_in_months: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_max_redemptions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_percent_off: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_redeem_by: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_times_redeemed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_coupon_valid: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_customer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_invoice: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_invoice_item: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_promotion_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_start: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount_subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ended_at: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items_data: std::option::Option<terrars::ListField<CustomerSubscriptionsDataElItemsDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items_has_more: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items_object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_invoice: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_pending_invoice_item_invoice: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_behalf_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pause_collection_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pause_collection_resumes_at: std::option::Option<terrars::PrimField<f64>>,
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
    pending_setup_intent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_update_billing_cycle_anchor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_update_expires_at: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_update_subscription_items: std
    ::option
    ::Option<terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_update_trial_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_update_trial_from_plan: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_clock: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_amount_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_data_destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trial_start: std::option::Option<terrars::PrimField<f64>>,
}

impl CustomerSubscriptionsDataEl {
    #[doc = "Set the field `application`.\n"]
    pub fn set_application(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.application = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `application_fee_percent`.\n"]
    pub fn set_application_fee_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.application_fee_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_tax_enabled`.\n"]
    pub fn set_automatic_tax_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_tax_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_cycle_anchor`.\n"]
    pub fn set_billing_cycle_anchor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.billing_cycle_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_thresholds_amount_gte`.\n"]
    pub fn set_billing_thresholds_amount_gte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.billing_thresholds_amount_gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_thresholds_reset_billing_cycle_anchor`.\n"]
    pub fn set_billing_thresholds_reset_billing_cycle_anchor(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.billing_thresholds_reset_billing_cycle_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cancel_at`.\n"]
    pub fn set_cancel_at(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cancel_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cancel_at_period_end`.\n"]
    pub fn set_cancel_at_period_end(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.cancel_at_period_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canceled_at`.\n"]
    pub fn set_canceled_at(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.canceled_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collection_method`.\n"]
    pub fn set_collection_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `currency`.\n"]
    pub fn set_currency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `current_period_end`.\n"]
    pub fn set_current_period_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.current_period_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `current_period_start`.\n"]
    pub fn set_current_period_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.current_period_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer`.\n"]
    pub fn set_customer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `days_until_due`.\n"]
    pub fn set_days_until_due(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_until_due = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_payment_method`.\n"]
    pub fn set_default_payment_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_source`.\n"]
    pub fn set_default_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_tax_rates`.\n"]
    pub fn set_default_tax_rates(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElDefaultTaxRatesEl>>,
    ) -> Self {
        self.default_tax_rates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_checkout_session`.\n"]
    pub fn set_discount_checkout_session(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_checkout_session = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_amount_off`.\n"]
    pub fn set_discount_coupon_amount_off(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_coupon_amount_off = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_applies_to_products`.\n"]
    pub fn set_discount_coupon_applies_to_products(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.discount_coupon_applies_to_products = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_created`.\n"]
    pub fn set_discount_coupon_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_coupon_created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_currency`.\n"]
    pub fn set_discount_coupon_currency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_coupon_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_currency_options`.\n"]
    pub fn set_discount_coupon_currency_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsEl>>,
    ) -> Self {
        self.discount_coupon_currency_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_duration`.\n"]
    pub fn set_discount_coupon_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_coupon_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_duration_in_months`.\n"]
    pub fn set_discount_coupon_duration_in_months(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.discount_coupon_duration_in_months = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_id`.\n"]
    pub fn set_discount_coupon_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_coupon_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_livemode`.\n"]
    pub fn set_discount_coupon_livemode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.discount_coupon_livemode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_max_redemptions`.\n"]
    pub fn set_discount_coupon_max_redemptions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_coupon_max_redemptions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_metadata`.\n"]
    pub fn set_discount_coupon_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.discount_coupon_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_name`.\n"]
    pub fn set_discount_coupon_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_coupon_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_object`.\n"]
    pub fn set_discount_coupon_object(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_coupon_object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_percent_off`.\n"]
    pub fn set_discount_coupon_percent_off(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_coupon_percent_off = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_redeem_by`.\n"]
    pub fn set_discount_coupon_redeem_by(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_coupon_redeem_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_times_redeemed`.\n"]
    pub fn set_discount_coupon_times_redeemed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_coupon_times_redeemed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_coupon_valid`.\n"]
    pub fn set_discount_coupon_valid(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.discount_coupon_valid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_customer`.\n"]
    pub fn set_discount_customer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount_customer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_end`.\n"]
    pub fn set_discount_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_id`.\n"]
    pub fn set_discount_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_invoice`.\n"]
    pub fn set_discount_invoice(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount_invoice = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_invoice_item`.\n"]
    pub fn set_discount_invoice_item(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_invoice_item = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_object`.\n"]
    pub fn set_discount_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.discount_object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_promotion_code`.\n"]
    pub fn set_discount_promotion_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_promotion_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_start`.\n"]
    pub fn set_discount_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.discount_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discount_subscription`.\n"]
    pub fn set_discount_subscription(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.discount_subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ended_at`.\n"]
    pub fn set_ended_at(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ended_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items_data`.\n"]
    pub fn set_items_data(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElItemsDataEl>>,
    ) -> Self {
        self.items_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items_has_more`.\n"]
    pub fn set_items_has_more(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.items_has_more = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items_object`.\n"]
    pub fn set_items_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items_object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items_url`.\n"]
    pub fn set_items_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_invoice`.\n"]
    pub fn set_latest_invoice(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.latest_invoice = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `next_pending_invoice_item_invoice`.\n"]
    pub fn set_next_pending_invoice_item_invoice(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.next_pending_invoice_item_invoice = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_behalf_of`.\n"]
    pub fn set_on_behalf_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.on_behalf_of = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pause_collection_behavior`.\n"]
    pub fn set_pause_collection_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pause_collection_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pause_collection_resumes_at`.\n"]
    pub fn set_pause_collection_resumes_at(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pause_collection_resumes_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type`.\n"]
    pub fn set_payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_acss_debit_verification_method`.\n"]
    pub fn set_payment_settings_payment_method_options_acss_debit_verification_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_acss_debit_verification_method =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_bancontact_preferred_language`.\n"]
    pub fn set_payment_settings_payment_method_options_bancontact_preferred_language(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_bancontact_preferred_language =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_mandate_options_amount`.\n"]
    pub fn set_payment_settings_payment_method_options_card_mandate_options_amount(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.payment_settings_payment_method_options_card_mandate_options_amount =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_mandate_options_amount_type`.\n"]
    pub fn set_payment_settings_payment_method_options_card_mandate_options_amount_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_card_mandate_options_amount_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_mandate_options_description`.\n"]
    pub fn set_payment_settings_payment_method_options_card_mandate_options_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_card_mandate_options_description =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_network`.\n"]
    pub fn set_payment_settings_payment_method_options_card_network(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_card_network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_card_request_three_d_secure`.\n"]
    pub fn set_payment_settings_payment_method_options_card_request_three_d_secure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_card_request_three_d_secure =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country`.\n"]
    pub fn set_payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_customer_balance_bank_transfer_type`.\n"]
    pub fn set_payment_settings_payment_method_options_customer_balance_bank_transfer_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_customer_balance_bank_transfer_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_customer_balance_funding_type`.\n"]
    pub fn set_payment_settings_payment_method_options_customer_balance_funding_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_customer_balance_funding_type =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `payment_settings_payment_method_options_us_bank_account_financial_connections_permissions`.\n"]
    pub fn set_payment_settings_payment_method_options_us_bank_account_financial_connections_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.payment_settings_payment_method_options_us_bank_account_financial_connections_permissions =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_options_us_bank_account_verification_method`.\n"]
    pub fn set_payment_settings_payment_method_options_us_bank_account_verification_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_payment_method_options_us_bank_account_verification_method =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_payment_method_types`.\n"]
    pub fn set_payment_settings_payment_method_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.payment_settings_payment_method_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_settings_save_default_payment_method`.\n"]
    pub fn set_payment_settings_save_default_payment_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.payment_settings_save_default_payment_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_invoice_item_interval_interval`.\n"]
    pub fn set_pending_invoice_item_interval_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pending_invoice_item_interval_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_invoice_item_interval_interval_count`.\n"]
    pub fn set_pending_invoice_item_interval_interval_count(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.pending_invoice_item_interval_interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_setup_intent`.\n"]
    pub fn set_pending_setup_intent(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.pending_setup_intent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_update_billing_cycle_anchor`.\n"]
    pub fn set_pending_update_billing_cycle_anchor(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.pending_update_billing_cycle_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_update_expires_at`.\n"]
    pub fn set_pending_update_expires_at(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pending_update_expires_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_update_subscription_items`.\n"]
    pub fn set_pending_update_subscription_items(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsEl>>,
    ) -> Self {
        self.pending_update_subscription_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_update_trial_end`.\n"]
    pub fn set_pending_update_trial_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pending_update_trial_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_update_trial_from_plan`.\n"]
    pub fn set_pending_update_trial_from_plan(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pending_update_trial_from_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_clock`.\n"]
    pub fn set_test_clock(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.test_clock = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_data_amount_percent`.\n"]
    pub fn set_transfer_data_amount_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transfer_data_amount_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_data_destination`.\n"]
    pub fn set_transfer_data_destination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transfer_data_destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trial_end`.\n"]
    pub fn set_trial_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.trial_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trial_start`.\n"]
    pub fn set_trial_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.trial_start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerSubscriptionsDataEl {
    type O = terrars::BlockAssignable<CustomerSubscriptionsDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerSubscriptionsDataEl {}

impl BuildCustomerSubscriptionsDataEl {
    pub fn build(self) -> CustomerSubscriptionsDataEl {
        CustomerSubscriptionsDataEl {
            application: core::default::Default::default(),
            application_fee_percent: core::default::Default::default(),
            automatic_tax_enabled: core::default::Default::default(),
            billing_cycle_anchor: core::default::Default::default(),
            billing_thresholds_amount_gte: core::default::Default::default(),
            billing_thresholds_reset_billing_cycle_anchor: core::default::Default::default(),
            cancel_at: core::default::Default::default(),
            cancel_at_period_end: core::default::Default::default(),
            canceled_at: core::default::Default::default(),
            collection_method: core::default::Default::default(),
            created: core::default::Default::default(),
            currency: core::default::Default::default(),
            current_period_end: core::default::Default::default(),
            current_period_start: core::default::Default::default(),
            customer: core::default::Default::default(),
            days_until_due: core::default::Default::default(),
            default_payment_method: core::default::Default::default(),
            default_source: core::default::Default::default(),
            default_tax_rates: core::default::Default::default(),
            description: core::default::Default::default(),
            discount_checkout_session: core::default::Default::default(),
            discount_coupon_amount_off: core::default::Default::default(),
            discount_coupon_applies_to_products: core::default::Default::default(),
            discount_coupon_created: core::default::Default::default(),
            discount_coupon_currency: core::default::Default::default(),
            discount_coupon_currency_options: core::default::Default::default(),
            discount_coupon_duration: core::default::Default::default(),
            discount_coupon_duration_in_months: core::default::Default::default(),
            discount_coupon_id: core::default::Default::default(),
            discount_coupon_livemode: core::default::Default::default(),
            discount_coupon_max_redemptions: core::default::Default::default(),
            discount_coupon_metadata: core::default::Default::default(),
            discount_coupon_name: core::default::Default::default(),
            discount_coupon_object: core::default::Default::default(),
            discount_coupon_percent_off: core::default::Default::default(),
            discount_coupon_redeem_by: core::default::Default::default(),
            discount_coupon_times_redeemed: core::default::Default::default(),
            discount_coupon_valid: core::default::Default::default(),
            discount_customer: core::default::Default::default(),
            discount_end: core::default::Default::default(),
            discount_id: core::default::Default::default(),
            discount_invoice: core::default::Default::default(),
            discount_invoice_item: core::default::Default::default(),
            discount_object: core::default::Default::default(),
            discount_promotion_code: core::default::Default::default(),
            discount_start: core::default::Default::default(),
            discount_subscription: core::default::Default::default(),
            ended_at: core::default::Default::default(),
            id: core::default::Default::default(),
            items_data: core::default::Default::default(),
            items_has_more: core::default::Default::default(),
            items_object: core::default::Default::default(),
            items_url: core::default::Default::default(),
            latest_invoice: core::default::Default::default(),
            livemode: core::default::Default::default(),
            metadata: core::default::Default::default(),
            next_pending_invoice_item_invoice: core::default::Default::default(),
            object: core::default::Default::default(),
            on_behalf_of: core::default::Default::default(),
            pause_collection_behavior: core::default::Default::default(),
            pause_collection_resumes_at: core::default::Default::default(),
            payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type: core
            ::default
            ::Default
            ::default(),
            payment_settings_payment_method_options_acss_debit_verification_method: core::default::Default::default(),
            payment_settings_payment_method_options_bancontact_preferred_language: core::default::Default::default(),
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
            payment_settings_payment_method_options_customer_balance_funding_type: core::default::Default::default(),
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
            pending_setup_intent: core::default::Default::default(),
            pending_update_billing_cycle_anchor: core::default::Default::default(),
            pending_update_expires_at: core::default::Default::default(),
            pending_update_subscription_items: core::default::Default::default(),
            pending_update_trial_end: core::default::Default::default(),
            pending_update_trial_from_plan: core::default::Default::default(),
            schedule: core::default::Default::default(),
            start_date: core::default::Default::default(),
            status: core::default::Default::default(),
            test_clock: core::default::Default::default(),
            transfer_data_amount_percent: core::default::Default::default(),
            transfer_data_destination: core::default::Default::default(),
            trial_end: core::default::Default::default(),
            trial_start: core::default::Default::default(),
        }
    }
}

pub struct CustomerSubscriptionsDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerSubscriptionsDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerSubscriptionsDataElRef {
        CustomerSubscriptionsDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerSubscriptionsDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.base))
    }

    #[doc = "Get a reference to the value of field `application_fee_percent` after provisioning.\n"]
    pub fn application_fee_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_fee_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `automatic_tax_enabled` after provisioning.\n"]
    pub fn automatic_tax_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_tax_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `billing_cycle_anchor` after provisioning.\n"]
    pub fn billing_cycle_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_cycle_anchor", self.base))
    }

    #[doc = "Get a reference to the value of field `billing_thresholds_amount_gte` after provisioning.\n"]
    pub fn billing_thresholds_amount_gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_thresholds_amount_gte", self.base))
    }

    #[doc =
        "Get a reference to the value of field `billing_thresholds_reset_billing_cycle_anchor` after provisioning.\n"]
    pub fn billing_thresholds_reset_billing_cycle_anchor(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.billing_thresholds_reset_billing_cycle_anchor", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cancel_at` after provisioning.\n"]
    pub fn cancel_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cancel_at", self.base))
    }

    #[doc = "Get a reference to the value of field `cancel_at_period_end` after provisioning.\n"]
    pub fn cancel_at_period_end(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cancel_at_period_end", self.base))
    }

    #[doc = "Get a reference to the value of field `canceled_at` after provisioning.\n"]
    pub fn canceled_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canceled_at", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_method` after provisioning.\n"]
    pub fn collection_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_method", self.base))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\n"]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }

    #[doc = "Get a reference to the value of field `current_period_end` after provisioning.\n"]
    pub fn current_period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_end", self.base))
    }

    #[doc = "Get a reference to the value of field `current_period_start` after provisioning.\n"]
    pub fn current_period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_period_start", self.base))
    }

    #[doc = "Get a reference to the value of field `customer` after provisioning.\n"]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }

    #[doc = "Get a reference to the value of field `days_until_due` after provisioning.\n"]
    pub fn days_until_due(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_until_due", self.base))
    }

    #[doc = "Get a reference to the value of field `default_payment_method` after provisioning.\n"]
    pub fn default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_payment_method", self.base))
    }

    #[doc = "Get a reference to the value of field `default_source` after provisioning.\n"]
    pub fn default_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_source", self.base))
    }

    #[doc = "Get a reference to the value of field `default_tax_rates` after provisioning.\n"]
    pub fn default_tax_rates(&self) -> terrars::ListRef<CustomerSubscriptionsDataElDefaultTaxRatesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_tax_rates", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_checkout_session` after provisioning.\n"]
    pub fn discount_checkout_session(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_checkout_session", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_amount_off` after provisioning.\n"]
    pub fn discount_coupon_amount_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_amount_off", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_applies_to_products` after provisioning.\n"]
    pub fn discount_coupon_applies_to_products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discount_coupon_applies_to_products", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_created` after provisioning.\n"]
    pub fn discount_coupon_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_created", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_currency` after provisioning.\n"]
    pub fn discount_coupon_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_currency", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_currency_options` after provisioning.\n"]
    pub fn discount_coupon_currency_options(
        &self,
    ) -> terrars::ListRef<CustomerSubscriptionsDataElDiscountCouponCurrencyOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discount_coupon_currency_options", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_duration` after provisioning.\n"]
    pub fn discount_coupon_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_duration_in_months` after provisioning.\n"]
    pub fn discount_coupon_duration_in_months(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_duration_in_months", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_id` after provisioning.\n"]
    pub fn discount_coupon_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_id", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_livemode` after provisioning.\n"]
    pub fn discount_coupon_livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_livemode", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_max_redemptions` after provisioning.\n"]
    pub fn discount_coupon_max_redemptions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_max_redemptions", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_metadata` after provisioning.\n"]
    pub fn discount_coupon_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.discount_coupon_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_name` after provisioning.\n"]
    pub fn discount_coupon_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_name", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_object` after provisioning.\n"]
    pub fn discount_coupon_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_object", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_percent_off` after provisioning.\n"]
    pub fn discount_coupon_percent_off(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_percent_off", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_redeem_by` after provisioning.\n"]
    pub fn discount_coupon_redeem_by(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_redeem_by", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_times_redeemed` after provisioning.\n"]
    pub fn discount_coupon_times_redeemed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_times_redeemed", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_coupon_valid` after provisioning.\n"]
    pub fn discount_coupon_valid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_coupon_valid", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_customer` after provisioning.\n"]
    pub fn discount_customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_customer", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_end` after provisioning.\n"]
    pub fn discount_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_end", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_id` after provisioning.\n"]
    pub fn discount_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_id", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_invoice` after provisioning.\n"]
    pub fn discount_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_invoice", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_invoice_item` after provisioning.\n"]
    pub fn discount_invoice_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_invoice_item", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_object` after provisioning.\n"]
    pub fn discount_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_object", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_promotion_code` after provisioning.\n"]
    pub fn discount_promotion_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_promotion_code", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_start` after provisioning.\n"]
    pub fn discount_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_start", self.base))
    }

    #[doc = "Get a reference to the value of field `discount_subscription` after provisioning.\n"]
    pub fn discount_subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `ended_at` after provisioning.\n"]
    pub fn ended_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ended_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `items_data` after provisioning.\n"]
    pub fn items_data(&self) -> terrars::ListRef<CustomerSubscriptionsDataElItemsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items_data", self.base))
    }

    #[doc = "Get a reference to the value of field `items_has_more` after provisioning.\n"]
    pub fn items_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items_has_more", self.base))
    }

    #[doc = "Get a reference to the value of field `items_object` after provisioning.\n"]
    pub fn items_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items_object", self.base))
    }

    #[doc = "Get a reference to the value of field `items_url` after provisioning.\n"]
    pub fn items_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items_url", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_invoice` after provisioning.\n"]
    pub fn latest_invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_invoice", self.base))
    }

    #[doc = "Get a reference to the value of field `livemode` after provisioning.\n"]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `next_pending_invoice_item_invoice` after provisioning.\n"]
    pub fn next_pending_invoice_item_invoice(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_pending_invoice_item_invoice", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `on_behalf_of` after provisioning.\n"]
    pub fn on_behalf_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_behalf_of", self.base))
    }

    #[doc = "Get a reference to the value of field `pause_collection_behavior` after provisioning.\n"]
    pub fn pause_collection_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_collection_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `pause_collection_resumes_at` after provisioning.\n"]
    pub fn pause_collection_resumes_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pause_collection_resumes_at", self.base))
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_acss_debit_mandate_options_transaction_type",
                self.base
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_acss_debit_verification_method` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_acss_debit_verification_method(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_acss_debit_verification_method", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_bancontact_preferred_language` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_bancontact_preferred_language(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_bancontact_preferred_language", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_amount` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_mandate_options_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_amount", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_amount_type` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_mandate_options_amount_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_amount_type", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_mandate_options_description` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_mandate_options_description(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_mandate_options_description", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_network` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_network", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_card_request_three_d_secure` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_card_request_three_d_secure(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_card_request_three_d_secure", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_customer_balance_bank_transfer_eu_bank_transfer_country",
                self.base
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_bank_transfer_type` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_customer_balance_bank_transfer_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_customer_balance_bank_transfer_type", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_customer_balance_funding_type` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_customer_balance_funding_type(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_customer_balance_funding_type", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_us_bank_account_financial_connections_permissions` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_us_bank_account_financial_connections_permissions(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!(
                "{}.payment_settings_payment_method_options_us_bank_account_financial_connections_permissions",
                self.base
            ),
        )
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_payment_method_options_us_bank_account_verification_method` after provisioning.\n"]
    pub fn payment_settings_payment_method_options_us_bank_account_verification_method(
        &self,
    ) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_payment_method_options_us_bank_account_verification_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `payment_settings_payment_method_types` after provisioning.\n"]
    pub fn payment_settings_payment_method_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payment_settings_payment_method_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `payment_settings_save_default_payment_method` after provisioning.\n"]
    pub fn payment_settings_save_default_payment_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.payment_settings_save_default_payment_method", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pending_invoice_item_interval_interval` after provisioning.\n"]
    pub fn pending_invoice_item_interval_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_item_interval_interval", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `pending_invoice_item_interval_interval_count` after provisioning.\n"]
    pub fn pending_invoice_item_interval_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_invoice_item_interval_interval_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `pending_setup_intent` after provisioning.\n"]
    pub fn pending_setup_intent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_setup_intent", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_update_billing_cycle_anchor` after provisioning.\n"]
    pub fn pending_update_billing_cycle_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_billing_cycle_anchor", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_update_expires_at` after provisioning.\n"]
    pub fn pending_update_expires_at(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_expires_at", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_update_subscription_items` after provisioning.\n"]
    pub fn pending_update_subscription_items(
        &self,
    ) -> terrars::ListRef<CustomerSubscriptionsDataElPendingUpdateSubscriptionItemsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pending_update_subscription_items", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_update_trial_end` after provisioning.\n"]
    pub fn pending_update_trial_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_trial_end", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_update_trial_from_plan` after provisioning.\n"]
    pub fn pending_update_trial_from_plan(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_update_trial_from_plan", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `test_clock` after provisioning.\n"]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_data_amount_percent` after provisioning.\n"]
    pub fn transfer_data_amount_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_amount_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_data_destination` after provisioning.\n"]
    pub fn transfer_data_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_data_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `trial_end` after provisioning.\n"]
    pub fn trial_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_end", self.base))
    }

    #[doc = "Get a reference to the value of field `trial_start` after provisioning.\n"]
    pub fn trial_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trial_start", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerTaxIdsDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_verified_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_verified_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerTaxIdsDataEl {
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

    #[doc = "Set the field `customer`.\n"]
    pub fn set_customer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `livemode`.\n"]
    pub fn set_livemode(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.livemode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `verification_status`.\n"]
    pub fn set_verification_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.verification_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verification_verified_address`.\n"]
    pub fn set_verification_verified_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.verification_verified_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verification_verified_name`.\n"]
    pub fn set_verification_verified_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.verification_verified_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerTaxIdsDataEl {
    type O = terrars::BlockAssignable<CustomerTaxIdsDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerTaxIdsDataEl {}

impl BuildCustomerTaxIdsDataEl {
    pub fn build(self) -> CustomerTaxIdsDataEl {
        CustomerTaxIdsDataEl {
            country: core::default::Default::default(),
            created: core::default::Default::default(),
            customer: core::default::Default::default(),
            id: core::default::Default::default(),
            livemode: core::default::Default::default(),
            object: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
            verification_status: core::default::Default::default(),
            verification_verified_address: core::default::Default::default(),
            verification_verified_name: core::default::Default::default(),
        }
    }
}

pub struct CustomerTaxIdsDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerTaxIdsDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerTaxIdsDataElRef {
        CustomerTaxIdsDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerTaxIdsDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `customer` after provisioning.\n"]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `livemode` after provisioning.\n"]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.base))
    }

    #[doc = "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `verification_status` after provisioning.\n"]
    pub fn verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_status", self.base))
    }

    #[doc = "Get a reference to the value of field `verification_verified_address` after provisioning.\n"]
    pub fn verification_verified_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_verified_address", self.base))
    }

    #[doc = "Get a reference to the value of field `verification_verified_name` after provisioning.\n"]
    pub fn verification_verified_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_verified_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerInvoiceSettingsCustomFieldsEl {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CustomerInvoiceSettingsCustomFieldsEl { }

impl terrars::ToListMappable for CustomerInvoiceSettingsCustomFieldsEl {
    type O = terrars::BlockAssignable<CustomerInvoiceSettingsCustomFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerInvoiceSettingsCustomFieldsEl {
    #[doc = "The name of the custom field."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the custom field."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCustomerInvoiceSettingsCustomFieldsEl {
    pub fn build(self) -> CustomerInvoiceSettingsCustomFieldsEl {
        CustomerInvoiceSettingsCustomFieldsEl {
            name: self.name,
            value: self.value,
        }
    }
}

pub struct CustomerInvoiceSettingsCustomFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerInvoiceSettingsCustomFieldsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerInvoiceSettingsCustomFieldsElRef {
        CustomerInvoiceSettingsCustomFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerInvoiceSettingsCustomFieldsElRef {
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
pub struct CustomerTaxIdDataEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CustomerTaxIdDataEl { }

impl terrars::ToListMappable for CustomerTaxIdDataEl {
    type O = terrars::BlockAssignable<CustomerTaxIdDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerTaxIdDataEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCustomerTaxIdDataEl {
    pub fn build(self) -> CustomerTaxIdDataEl {
        CustomerTaxIdDataEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct CustomerTaxIdDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerTaxIdDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerTaxIdDataElRef {
        CustomerTaxIdDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerTaxIdDataElRef {
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

#[derive(Serialize, Default)]
struct CustomerDynamic {
    invoice_settings_custom_fields: std::option::Option<terrars::DynamicBlock<CustomerInvoiceSettingsCustomFieldsEl>>,
    tax_id_data: std::option::Option<terrars::DynamicBlock<CustomerTaxIdDataEl>>,
}
