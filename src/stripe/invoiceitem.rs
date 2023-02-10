use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct InvoiceitemData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<PrimField<String>>,
    customer: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discountable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_end: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period_start: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_currency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_product: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_tax_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_unit_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data_unit_amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<InvoiceitemDiscountsEl>>,
    dynamic: InvoiceitemDynamic,
}

struct Invoiceitem_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<InvoiceitemData>,
}

#[derive(Clone)]
pub struct Invoiceitem(Rc<Invoiceitem_>);

impl Invoiceitem {
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

    #[doc= "Set the field `amount`.\nThe integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. Passing in a negative `amount` will reduce the `amount_due` on the invoice."]
    pub fn set_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().amount = Some(v.into());
        self
    }

    #[doc= "Set the field `currency`.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn set_currency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().currency = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn arbitrary string which you can attach to the invoice item. The description is displayed in the invoice for easy tracking."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `discountable`.\nControls whether discounts apply to this invoice item. Defaults to false for prorations or negative invoice items, and true for all other invoice items."]
    pub fn set_discountable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().discountable = Some(v.into());
        self
    }

    #[doc= "Set the field `invoice`.\nThe ID of an existing invoice to add this invoice item to. When left blank, the invoice item will be added to the next upcoming scheduled invoice. This is useful when adding invoice items in response to an invoice.created webhook. You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice."]
    pub fn set_invoice(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().invoice = Some(v.into());
        self
    }

    #[doc= "Set the field `period_end`.\nThe end of the period, which must be greater than or equal to the start."]
    pub fn set_period_end(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().period_end = Some(v.into());
        self
    }

    #[doc= "Set the field `period_start`.\nThe start of the period."]
    pub fn set_period_start(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().period_start = Some(v.into());
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

    #[doc= "Set the field `quantity`.\nNon-negative integer. The quantity of units for the invoice item."]
    pub fn set_quantity(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().quantity = Some(v.into());
        self
    }

    #[doc= "Set the field `subscription`.\nThe ID of a subscription to add this invoice item to. When left blank, the invoice item will be be added to the next upcoming scheduled invoice. When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item. Use this when you want to express that an invoice item has been accrued within the context of a particular subscription."]
    pub fn set_subscription(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().subscription = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_behavior`.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn set_tax_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tax_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_code`.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn set_tax_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tax_code = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_rates`.\nThe tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item."]
    pub fn set_tax_rates(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tax_rates = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount`.\nThe integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. This `unit_amount` will be multiplied by the quantity to get the full amount. Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice."]
    pub fn set_unit_amount(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().unit_amount = Some(v.into());
        self
    }

    #[doc= "Set the field `unit_amount_decimal`.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn set_unit_amount_decimal(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().unit_amount_decimal = Some(v.into());
        self
    }

    #[doc= "Set the field `discounts`.\n"]
    pub fn set_discounts(self, v: impl Into<BlockAssignable<InvoiceitemDiscountsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().discounts = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.discounts = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `amount` after provisioning.\nThe integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. Passing in a negative `amount` will reduce the `amount_due` on the invoice."]
    pub fn amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer who will be billed when this invoice item is billed."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn date(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string which you can attach to the invoice item. The description is displayed in the invoice for easy tracking."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discountable` after provisioning.\nControls whether discounts apply to this invoice item. Defaults to false for prorations or negative invoice items, and true for all other invoice items."]
    pub fn discountable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discountable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invoice` after provisioning.\nThe ID of an existing invoice to add this invoice item to. When left blank, the invoice item will be added to the next upcoming scheduled invoice. This is useful when adding invoice items in response to an invoice.created webhook. You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice."]
    pub fn invoice(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invoice", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period_end` after provisioning.\nThe end of the period, which must be greater than or equal to the start."]
    pub fn period_end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period_start` after provisioning.\nThe start of the period."]
    pub fn period_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `proration` after provisioning.\nWhether the invoice item was created automatically as a proration adjustment when the customer switched plans."]
    pub fn proration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quantity` after provisioning.\nNon-negative integer. The quantity of units for the invoice item."]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription` after provisioning.\nThe ID of a subscription to add this invoice item to. When left blank, the invoice item will be be added to the next upcoming scheduled invoice. When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item. Use this when you want to express that an invoice item has been accrued within the context of a particular subscription."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription_item` after provisioning.\nThe subscription item that this invoice item has been created for, if any."]
    pub fn subscription_item(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription_item", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn tax_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_rates` after provisioning.\nThe tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item."]
    pub fn tax_rates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this invoice item belongs to."]
    pub fn test_clock(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount` after provisioning.\nThe integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. This `unit_amount` will be multiplied by the quantity to get the full amount. Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice."]
    pub fn unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> ListRef<InvoiceitemDiscountsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.discounts", self.extract_ref()))
    }
}

impl Resource for Invoiceitem {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Invoiceitem {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Invoiceitem {
    type O = ListRef<InvoiceitemRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for Invoiceitem_ {
    fn extract_resource_type(&self) -> String {
        "stripe_invoiceitem".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInvoiceitem {
    pub tf_id: String,
    #[doc= "The ID of the customer who will be billed when this invoice item is billed."]
    pub customer: PrimField<String>,
}

impl BuildInvoiceitem {
    pub fn build(self, stack: &mut Stack) -> Invoiceitem {
        let out = Invoiceitem(Rc::new(Invoiceitem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(InvoiceitemData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct InvoiceitemRef {
    shared: StackShared,
    base: String,
}

impl Ref for InvoiceitemRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl InvoiceitemRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `amount` after provisioning.\nThe integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. Passing in a negative `amount` will reduce the `amount_due` on the invoice."]
    pub fn amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `currency` after provisioning.\nThree-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies)."]
    pub fn currency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer who will be billed when this invoice item is billed."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `date` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn date(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string which you can attach to the invoice item. The description is displayed in the invoice for easy tracking."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discountable` after provisioning.\nControls whether discounts apply to this invoice item. Defaults to false for prorations or negative invoice items, and true for all other invoice items."]
    pub fn discountable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.discountable", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `invoice` after provisioning.\nThe ID of an existing invoice to add this invoice item to. When left blank, the invoice item will be added to the next upcoming scheduled invoice. This is useful when adding invoice items in response to an invoice.created webhook. You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice."]
    pub fn invoice(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invoice", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period_end` after provisioning.\nThe end of the period, which must be greater than or equal to the start."]
    pub fn period_end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period_end", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period_start` after provisioning.\nThe start of the period."]
    pub fn period_start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.period_start", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `proration` after provisioning.\nWhether the invoice item was created automatically as a proration adjustment when the customer switched plans."]
    pub fn proration(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.proration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `quantity` after provisioning.\nNon-negative integer. The quantity of units for the invoice item."]
    pub fn quantity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription` after provisioning.\nThe ID of a subscription to add this invoice item to. When left blank, the invoice item will be be added to the next upcoming scheduled invoice. When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item. Use this when you want to express that an invoice item has been accrued within the context of a particular subscription."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subscription_item` after provisioning.\nThe subscription item that this invoice item has been created for, if any."]
    pub fn subscription_item(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription_item", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_behavior` after provisioning.\nSpecifies whether the price is considered inclusive of taxes or exclusive of taxes. One of `inclusive`, `exclusive`, or `unspecified`. Once specified as either `inclusive` or `exclusive`, it cannot be changed."]
    pub fn tax_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn tax_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_rates` after provisioning.\nThe tax rates which apply to the invoice item. When set, the `default_tax_rates` on the invoice do not apply to this invoice item."]
    pub fn tax_rates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.tax_rates", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `test_clock` after provisioning.\nID of the test clock this invoice item belongs to."]
    pub fn test_clock(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.test_clock", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount` after provisioning.\nThe integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice. This `unit_amount` will be multiplied by the quantity to get the full amount. Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice."]
    pub fn unit_amount(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `unit_amount_decimal` after provisioning.\nSame as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places. Only one of `unit_amount` and `unit_amount_decimal` can be set."]
    pub fn unit_amount_decimal(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `discounts` after provisioning.\n"]
    pub fn discounts(&self) -> ListRef<InvoiceitemDiscountsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.discounts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct InvoiceitemDiscountsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    coupon: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discount: Option<PrimField<String>>,
}

impl InvoiceitemDiscountsEl {
    #[doc= "Set the field `coupon`.\n"]
    pub fn set_coupon(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.coupon = Some(v.into());
        self
    }

    #[doc= "Set the field `discount`.\n"]
    pub fn set_discount(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.discount = Some(v.into());
        self
    }
}

impl ToListMappable for InvoiceitemDiscountsEl {
    type O = BlockAssignable<InvoiceitemDiscountsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInvoiceitemDiscountsEl {}

impl BuildInvoiceitemDiscountsEl {
    pub fn build(self) -> InvoiceitemDiscountsEl {
        InvoiceitemDiscountsEl {
            coupon: core::default::Default::default(),
            discount: core::default::Default::default(),
        }
    }
}

pub struct InvoiceitemDiscountsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for InvoiceitemDiscountsElRef {
    fn new(shared: StackShared, base: String) -> InvoiceitemDiscountsElRef {
        InvoiceitemDiscountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl InvoiceitemDiscountsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `checkout_session` after provisioning.\nThe Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode. Will not be present for subscription mode."]
    pub fn checkout_session(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.checkout_session", self.base))
    }

    #[doc= "Get a reference to the value of field `coupon` after provisioning.\n"]
    pub fn coupon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.coupon", self.base))
    }

    #[doc= "Get a reference to the value of field `customer` after provisioning.\nThe ID of the customer associated with this discount."]
    pub fn customer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }

    #[doc= "Get a reference to the value of field `discount` after provisioning.\n"]
    pub fn discount(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.discount", self.base))
    }

    #[doc= "Get a reference to the value of field `end` after provisioning.\nIf the coupon has a duration of `repeating`, the date that this discount will end. If the coupon has a duration of `once` or `forever`, this attribute will be null."]
    pub fn end(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nThe ID of the discount object. Discounts cannot be fetched by ID. Use `expand[]=discounts` in API calls to expand discount IDs in an array."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `invoice` after provisioning.\nThe invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice."]
    pub fn invoice(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invoice", self.base))
    }

    #[doc= "Get a reference to the value of field `invoice_item` after provisioning.\nThe invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item."]
    pub fn invoice_item(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.invoice_item", self.base))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc= "Get a reference to the value of field `promotion_code` after provisioning.\nThe promotion code applied to create this discount."]
    pub fn promotion_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.promotion_code", self.base))
    }

    #[doc= "Get a reference to the value of field `start` after provisioning.\nDate that the coupon was applied."]
    pub fn start(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc= "Get a reference to the value of field `subscription` after provisioning.\nThe subscription that this coupon is applied to, if it is applied to a particular subscription."]
    pub fn subscription(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subscription", self.base))
    }
}

#[derive(Serialize, Default)]
struct InvoiceitemDynamic {
    discounts: Option<DynamicBlock<InvoiceitemDiscountsEl>>,
}
