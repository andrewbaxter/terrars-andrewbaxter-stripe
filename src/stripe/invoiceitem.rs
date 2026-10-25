#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct InvoiceitemData_ {
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
    amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    customer: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discountable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoice: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_start: std::option::Option<terrars::PrimField<f64>>,
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
    subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_rates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discounts: std::option::Option<std::vec::Vec<Invoiceitem_Discounts>>,
    dynamic: InvoiceitemDynamic,
}

struct Invoiceitem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<InvoiceitemData_>,
}

#[derive(Clone)]
pub struct Invoiceitem(std::rc::Rc<Invoiceitem_>);

impl Invoiceitem {
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
        "Set the field `amount`.\nThe integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. Passing in a negative `amount` will reduce the `amount_due` on the invoice."]
    pub fn set_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `currency`.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn set_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().currency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn arbitrary string which you can attach to the invoice item. The description is displayed in the invoice for easy tracking."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `discountable`.\nControls whether discounts apply to this invoice item. Defaults to false for prorations or negative invoice items, and true for all other invoice items."]
    pub fn set_discountable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().discountable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invoice`.\nThe ID of an existing invoice to add this invoice item to. When left blank, the invoice item will be added to the next upcoming scheduled invoice. This is useful when adding invoice items in response to an invoice.created webhook. You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice."]
    pub fn set_invoice(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().invoice = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_end`.\nThe end of the period, which must be greater than or equal to the start."]
    pub fn set_period_end(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().period_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_start`.\nThe start of the period."]
    pub fn set_period_start(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().period_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price`.\nThe ID of the price object."]
    pub fn set_price(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_currency`.\n"]
    pub fn set_price_data_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().price_data_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_product`.\n"]
    pub fn set_price_data_product(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().price_data_product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_tax_behavior`.\n"]
    pub fn set_price_data_tax_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().price_data_tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_unit_amount`.\n"]
    pub fn set_price_data_unit_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().price_data_unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_data_unit_amount_decimal`.\n"]
    pub fn set_price_data_unit_amount_decimal(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().price_data_unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quantity`.\nNon-negative integer. The quantity of units for the invoice item."]
    pub fn set_quantity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().quantity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subscription`.\nThe ID of a subscription to add this invoice item to. When left blank, the invoice item will be be added to the next upcoming scheduled invoice. When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item. Use this when you want to express that an invoice item has been accrued within the context of a particular subscription."]
    pub fn set_subscription(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_behavior`.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn set_tax_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_code`.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn set_tax_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tax_rates`.\nThe tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item."]
    pub fn set_tax_rates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tax_rates = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount`.\nThe integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. This `unit_amount` will be multiplied by the quantity to get the full amount. Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice."]
    pub fn set_unit_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_amount_decimal`.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn set_unit_amount_decimal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `discounts`.\n"]
    pub fn set_discounts(self, v: impl std::convert::Into<terrars::BlockAssignable<Invoiceitem_Discounts>>) -> Self {
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
        "Get a reference to the value of field `amount` after provisioning.\nThe integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. Passing in a negative `amount` will reduce the `amount_due` on the invoice."]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer who will be billed when this invoice item is billed."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `date` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string which you can attach to the invoice item. The description is displayed in the invoice for easy tracking."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discountable` after provisioning.\nControls whether discounts apply to this invoice item. Defaults to false for prorations or negative invoice items, and true for all other invoice items."]
    pub fn discountable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discountable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice` after provisioning.\nThe ID of an existing invoice to add this invoice item to. When left blank, the invoice item will be added to the next upcoming scheduled invoice. This is useful when adding invoice items in response to an invoice.created webhook. You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice."]
    pub fn invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice", self.extract_ref()))
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
        "Get a reference to the value of field `period_end` after provisioning.\nThe end of the period, which must be greater than or equal to the start."]
    pub fn period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `period_start` after provisioning.\nThe start of the period."]
    pub fn period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\nThe ID of the price object."]
    pub fn price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_currency` after provisioning.\n"]
    pub fn price_data_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_currency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_product` after provisioning.\n"]
    pub fn price_data_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_product", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_tax_behavior` after provisioning.\n"]
    pub fn price_data_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_tax_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount` after provisioning.\n"]
    pub fn price_data_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn price_data_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.price_data_unit_amount_decimal", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `proration` after provisioning.\nWhether the invoice item was created automatically as a proration adjustment when the customer switched plans."]
    pub fn proration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quantity` after provisioning.\nNon-negative integer. The quantity of units for the invoice item."]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nThe ID of a subscription to add this invoice item to. When left blank, the invoice item will be be added to the next upcoming scheduled invoice. When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item. Use this when you want to express that an invoice item has been accrued within the context of a particular subscription."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_item` after provisioning.\nThe subscription item that this invoice item has been created for, if any."]
    pub fn subscription_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_item", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_rates` after provisioning.\nThe tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item."]
    pub fn tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this invoice item belongs to."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount` after provisioning.\nThe integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. This `unit_amount` will be multiplied by the quantity to get the full amount. Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice."]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> terrars::ListRef<Invoiceitem_DiscountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discounts", self.extract_ref()))
    }
}

impl terrars::Referable for Invoiceitem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Invoiceitem { }

impl terrars::ToListMappable for Invoiceitem {
    type O = terrars::ListRef<InvoiceitemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Invoiceitem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_invoiceitem".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInvoiceitem {
    pub tf_id: std::string::String,
    #[doc = "The ID of the customer who will be billed when this invoice item is billed."]
    pub customer: terrars::PrimField<std::string::String>,
}

impl BuildInvoiceitem {
    pub fn build(self, stack: &mut terrars::Stack) -> Invoiceitem {
        let out = Invoiceitem(std::rc::Rc::new(Invoiceitem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(InvoiceitemData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                amount: core::default::Default::default(),
                currency: core::default::Default::default(),
                customer: self.customer,
                description: core::default::Default::default(),
                discountable: core::default::Default::default(),
                invoice: core::default::Default::default(),
                period_end: core::default::Default::default(),
                period_start: core::default::Default::default(),
                price: core::default::Default::default(),
                price_data_currency: core::default::Default::default(),
                price_data_product: core::default::Default::default(),
                price_data_tax_behavior: core::default::Default::default(),
                price_data_unit_amount: core::default::Default::default(),
                price_data_unit_amount_decimal: core::default::Default::default(),
                quantity: core::default::Default::default(),
                subscription: core::default::Default::default(),
                tax_behavior: core::default::Default::default(),
                tax_code: core::default::Default::default(),
                tax_rates: core::default::Default::default(),
                unit_amount: core::default::Default::default(),
                unit_amount_decimal: core::default::Default::default(),
                discounts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct InvoiceitemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for InvoiceitemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl InvoiceitemRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `amount` after provisioning.\nThe integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. Passing in a negative `amount` will reduce the `amount_due` on the invoice."]
    pub fn amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer who will be billed when this invoice item is billed."]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `date` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string which you can attach to the invoice item. The description is displayed in the invoice for easy tracking."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discountable` after provisioning.\nControls whether discounts apply to this invoice item. Defaults to false for prorations or negative invoice items, and true for all other invoice items."]
    pub fn discountable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discountable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invoice` after provisioning.\nThe ID of an existing invoice to add this invoice item to. When left blank, the invoice item will be added to the next upcoming scheduled invoice. This is useful when adding invoice items in response to an invoice.created webhook. You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice."]
    pub fn invoice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invoice", self.extract_ref()))
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
        "Get a reference to the value of field `period_end` after provisioning.\nThe end of the period, which must be greater than or equal to the start."]
    pub fn period_end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `period_start` after provisioning.\nThe start of the period."]
    pub fn period_start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price` after provisioning.\nThe ID of the price object."]
    pub fn price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_currency` after provisioning.\n"]
    pub fn price_data_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_currency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_product` after provisioning.\n"]
    pub fn price_data_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_product", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_tax_behavior` after provisioning.\n"]
    pub fn price_data_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_tax_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount` after provisioning.\n"]
    pub fn price_data_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_data_unit_amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn price_data_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.price_data_unit_amount_decimal", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `proration` after provisioning.\nWhether the invoice item was created automatically as a proration adjustment when the customer switched plans."]
    pub fn proration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `quantity` after provisioning.\nNon-negative integer. The quantity of units for the invoice item."]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription` after provisioning.\nThe ID of a subscription to add this invoice item to. When left blank, the invoice item will be be added to the next upcoming scheduled invoice. When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item. Use this when you want to express that an invoice item has been accrued within the context of a particular subscription."]
    pub fn subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_item` after provisioning.\nThe subscription item that this invoice item has been created for, if any."]
    pub fn subscription_item(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_item", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_rates` after provisioning.\nThe tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item."]
    pub fn tax_rates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this invoice item belongs to."]
    pub fn test_clock(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount` after provisioning.\nThe integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. This `unit_amount` will be multiplied by the quantity to get the full amount. Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice."]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> terrars::ListRef<Invoiceitem_DiscountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.discounts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Invoiceitem_Discounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    coupon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discount: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Invoiceitem_Discounts {
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

impl terrars::ToListMappable for Invoiceitem_Discounts {
    type O = terrars::BlockAssignable<Invoiceitem_Discounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoiceitem_Discounts {}

impl BuildInvoiceitem_Discounts {
    pub fn build(self) -> Invoiceitem_Discounts {
        Invoiceitem_Discounts {
            coupon: core::default::Default::default(),
            discount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Invoiceitem_DiscountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Invoiceitem_DiscountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Invoiceitem_DiscountsRef {
        Invoiceitem_DiscountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Invoiceitem_DiscountsRef {
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

    #[doc = "Get a reference to the value of field `discount` after provisioning.\n"]
    pub fn discount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discount", self.base))
    }

    #[doc =
        "Get a reference to the value of field `end` after provisioning.\nIf the coupon has a duration of `repeating`, the date that this discount will end. If the coupon has a duration of `once` or `forever`, this attribute will be null."]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
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
struct InvoiceitemDynamic {
    discounts: std::option::Option<terrars::DynamicBlock<Invoiceitem_Discounts>>,
}
