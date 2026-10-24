use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct ProductData {
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
    default_price_data_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_price_data_recurring_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_price_data_recurring_interval_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_price_data_tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_price_data_unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_price_data_unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    images: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_dimensions_height: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_dimensions_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_dimensions_weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    package_dimensions_width: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shippable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement_descriptor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_price_data_currency_options: std::option::Option<std::vec::Vec<ProductDefaultPriceDataCurrencyOptionsEl>>,
    dynamic: ProductDynamic,
}

struct Product_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ProductData>,
}

#[derive(Clone)]
pub struct Product(std::rc::Rc<Product_>);

impl Product {
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

    #[doc = "Set the field `default_price_data_currency`.\n"]
    pub fn set_default_price_data_currency(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_price_data_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_price_data_recurring_interval`.\n"]
    pub fn set_default_price_data_recurring_interval(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_price_data_recurring_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_price_data_recurring_interval_count`.\n"]
    pub fn set_default_price_data_recurring_interval_count(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().default_price_data_recurring_interval_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_price_data_tax_behavior`.\n"]
    pub fn set_default_price_data_tax_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_price_data_tax_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_price_data_unit_amount`.\n"]
    pub fn set_default_price_data_unit_amount(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_price_data_unit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_price_data_unit_amount_decimal`.\n"]
    pub fn set_default_price_data_unit_amount_decimal(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_price_data_unit_amount_decimal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nThe product's description, meant to be displayable to the customer. Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `images`.\nA list of up to 8 URLs of images for this product, meant to be displayable to the customer."]
    pub fn set_images(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().images = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_dimensions_height`.\nHeight, in inches."]
    pub fn set_package_dimensions_height(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().package_dimensions_height = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_dimensions_length`.\nLength, in inches."]
    pub fn set_package_dimensions_length(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().package_dimensions_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_dimensions_weight`.\nWeight, in ounces."]
    pub fn set_package_dimensions_weight(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().package_dimensions_weight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `package_dimensions_width`.\nWidth, in inches."]
    pub fn set_package_dimensions_width(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().package_dimensions_width = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shippable`.\nWhether this product is shipped (i.e., physical goods)."]
    pub fn set_shippable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().shippable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `statement_descriptor`.\nAn arbitrary string to be displayed on your customer's credit card or bank statement. While most banks display this information consistently, some may display it incorrectly or not at all.\n\nThis may be up to 22 characters. The statement description may not include `<`, `>`, `\\`, `\"`, `'` characters, and will appear on your customer's statement in capital letters. Non-ASCII characters are automatically stripped.\n It must contain at least one letter."]
    pub fn set_statement_descriptor(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().statement_descriptor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tax_code`.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn set_tax_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tax_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unit_label`.\nA label that represents units of this product in Stripe and on customers’ receipts and invoices. When set, this will be included in associated invoice line item descriptions."]
    pub fn set_unit_label(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().unit_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nA URL of a publicly-accessible webpage for this product."]
    pub fn set_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_price_data_currency_options`.\n"]
    pub fn set_default_price_data_currency_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProductDefaultPriceDataCurrencyOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_price_data_currency_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_price_data_currency_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_price` after provisioning.\nThe ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product."]
    pub fn default_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_price_data_currency` after provisioning.\n"]
    pub fn default_price_data_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_price_data_currency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_price_data_recurring_interval` after provisioning.\n"]
    pub fn default_price_data_recurring_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_recurring_interval", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_recurring_interval_count` after provisioning.\n"]
    pub fn default_price_data_recurring_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_recurring_interval_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_tax_behavior` after provisioning.\n"]
    pub fn default_price_data_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_tax_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_unit_amount` after provisioning.\n"]
    pub fn default_price_data_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_unit_amount", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn default_price_data_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_unit_amount_decimal", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe product's description, meant to be displayable to the customer. Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAn identifier will be randomly generated by Stripe. You can optionally override this ID, but the ID must be unique across all products in your Stripe account."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `images` after provisioning.\nA list of up to 8 URLs of images for this product, meant to be displayable to the customer."]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe product's name, meant to be displayable to the customer."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_height` after provisioning.\nHeight, in inches."]
    pub fn package_dimensions_height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_height", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_length` after provisioning.\nLength, in inches."]
    pub fn package_dimensions_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_weight` after provisioning.\nWeight, in ounces."]
    pub fn package_dimensions_weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_weight", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_width` after provisioning.\nWidth, in inches."]
    pub fn package_dimensions_width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_width", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shippable` after provisioning.\nWhether this product is shipped (i.e., physical goods)."]
    pub fn shippable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shippable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `statement_descriptor` after provisioning.\nAn arbitrary string to be displayed on your customer's credit card or bank statement. While most banks display this information consistently, some may display it incorrectly or not at all.\n\nThis may be up to 22 characters. The statement description may not include `<`, `>`, `\\`, `\"`, `'` characters, and will appear on your customer's statement in capital letters. Non-ASCII characters are automatically stripped.\n It must contain at least one letter."]
    pub fn statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_descriptor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_label` after provisioning.\nA label that represents units of this product in Stripe and on customers’ receipts and invoices. When set, this will be included in associated invoice line item descriptions."]
    pub fn unit_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nTime at which the object was last updated. Measured in seconds since the Unix epoch."]
    pub fn updated(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nA URL of a publicly-accessible webpage for this product."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_price_data_currency_options` after provisioning.\n"]
    pub fn default_price_data_currency_options(&self) -> terrars::ListRef<ProductDefaultPriceDataCurrencyOptionsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_price_data_currency_options", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Product {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Product { }

impl terrars::ToListMappable for Product {
    type O = terrars::ListRef<ProductRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Product_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_product".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProduct {
    pub tf_id: std::string::String,
    #[doc = "The product's name, meant to be displayable to the customer."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildProduct {
    pub fn build(self, stack: &mut terrars::Stack) -> Product {
        let out = Product(std::rc::Rc::new(Product_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ProductData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_price_data_currency: core::default::Default::default(),
                default_price_data_recurring_interval: core::default::Default::default(),
                default_price_data_recurring_interval_count: core::default::Default::default(),
                default_price_data_tax_behavior: core::default::Default::default(),
                default_price_data_unit_amount: core::default::Default::default(),
                default_price_data_unit_amount_decimal: core::default::Default::default(),
                description: core::default::Default::default(),
                images: core::default::Default::default(),
                name: self.name,
                package_dimensions_height: core::default::Default::default(),
                package_dimensions_length: core::default::Default::default(),
                package_dimensions_weight: core::default::Default::default(),
                package_dimensions_width: core::default::Default::default(),
                shippable: core::default::Default::default(),
                statement_descriptor: core::default::Default::default(),
                tax_code: core::default::Default::default(),
                unit_label: core::default::Default::default(),
                url: core::default::Default::default(),
                default_price_data_currency_options: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProductRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProductRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProductRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_price` after provisioning.\nThe ID of the [Price](https://stripe.com/docs/api/prices) object that is the default price for this product."]
    pub fn default_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_price_data_currency` after provisioning.\n"]
    pub fn default_price_data_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_price_data_currency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_price_data_recurring_interval` after provisioning.\n"]
    pub fn default_price_data_recurring_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_recurring_interval", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_recurring_interval_count` after provisioning.\n"]
    pub fn default_price_data_recurring_interval_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_recurring_interval_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_tax_behavior` after provisioning.\n"]
    pub fn default_price_data_tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_tax_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_unit_amount` after provisioning.\n"]
    pub fn default_price_data_unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_unit_amount", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_price_data_unit_amount_decimal` after provisioning.\n"]
    pub fn default_price_data_unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_price_data_unit_amount_decimal", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe product's description, meant to be displayable to the customer. Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nAn identifier will be randomly generated by Stripe. You can optionally override this ID, but the ID must be unique across all products in your Stripe account."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `images` after provisioning.\nA list of up to 8 URLs of images for this product, meant to be displayable to the customer."]
    pub fn images(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.images", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe product's name, meant to be displayable to the customer."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_height` after provisioning.\nHeight, in inches."]
    pub fn package_dimensions_height(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_height", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_length` after provisioning.\nLength, in inches."]
    pub fn package_dimensions_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_length", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_weight` after provisioning.\nWeight, in ounces."]
    pub fn package_dimensions_weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_weight", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `package_dimensions_width` after provisioning.\nWidth, in inches."]
    pub fn package_dimensions_width(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.package_dimensions_width", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shippable` after provisioning.\nWhether this product is shipped (i.e., physical goods)."]
    pub fn shippable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shippable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `statement_descriptor` after provisioning.\nAn arbitrary string to be displayed on your customer's credit card or bank statement. While most banks display this information consistently, some may display it incorrectly or not at all.\n\nThis may be up to 22 characters. The statement description may not include `<`, `>`, `\\`, `\"`, `'` characters, and will appear on your customer's statement in capital letters. Non-ASCII characters are automatically stripped.\n It must contain at least one letter."]
    pub fn statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement_descriptor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tax_code` after provisioning.\nA [tax code](https://stripe.com/docs/tax/tax-categories) ID."]
    pub fn tax_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `unit_label` after provisioning.\nA label that represents units of this product in Stripe and on customers’ receipts and invoices. When set, this will be included in associated invoice line item descriptions."]
    pub fn unit_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_label", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nTime at which the object was last updated. Measured in seconds since the Unix epoch."]
    pub fn updated(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nA URL of a publicly-accessible webpage for this product."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_price_data_currency_options` after provisioning.\n"]
    pub fn default_price_data_currency_options(&self) -> terrars::ListRef<ProductDefaultPriceDataCurrencyOptionsElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_price_data_currency_options", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ProductDefaultPriceDataCurrencyOptionsElTiersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flat_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    up_to: terrars::PrimField<std::string::String>,
}

impl ProductDefaultPriceDataCurrencyOptionsElTiersEl {
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
}

impl terrars::ToListMappable for ProductDefaultPriceDataCurrencyOptionsElTiersEl {
    type O = terrars::BlockAssignable<ProductDefaultPriceDataCurrencyOptionsElTiersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProductDefaultPriceDataCurrencyOptionsElTiersEl {
    #[doc = ""]
    pub up_to: terrars::PrimField<std::string::String>,
}

impl BuildProductDefaultPriceDataCurrencyOptionsElTiersEl {
    pub fn build(self) -> ProductDefaultPriceDataCurrencyOptionsElTiersEl {
        ProductDefaultPriceDataCurrencyOptionsElTiersEl {
            flat_amount: core::default::Default::default(),
            flat_amount_decimal: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            up_to: self.up_to,
        }
    }
}

pub struct ProductDefaultPriceDataCurrencyOptionsElTiersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProductDefaultPriceDataCurrencyOptionsElTiersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ProductDefaultPriceDataCurrencyOptionsElTiersElRef {
        ProductDefaultPriceDataCurrencyOptionsElTiersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProductDefaultPriceDataCurrencyOptionsElTiersElRef {
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
    pub fn up_to(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.up_to", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProductDefaultPriceDataCurrencyOptionsElDynamic {
    tiers: std::option::Option<terrars::DynamicBlock<ProductDefaultPriceDataCurrencyOptionsElTiersEl>>,
}

#[derive(Serialize)]
pub struct ProductDefaultPriceDataCurrencyOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_maximum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_minimum: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_unit_amount_preset: std::option::Option<terrars::PrimField<f64>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tax_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_amount_decimal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tiers: std::option::Option<std::vec::Vec<ProductDefaultPriceDataCurrencyOptionsElTiersEl>>,
    dynamic: ProductDefaultPriceDataCurrencyOptionsElDynamic,
}

impl ProductDefaultPriceDataCurrencyOptionsEl {
    #[doc = "Set the field `custom_unit_amount_enabled`.\n"]
    pub fn set_custom_unit_amount_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.custom_unit_amount_enabled = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `tax_behavior`.\n"]
    pub fn set_tax_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tax_behavior = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tiers`.\n"]
    pub fn set_tiers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProductDefaultPriceDataCurrencyOptionsElTiersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tiers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tiers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ProductDefaultPriceDataCurrencyOptionsEl {
    type O = terrars::BlockAssignable<ProductDefaultPriceDataCurrencyOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProductDefaultPriceDataCurrencyOptionsEl {
    #[doc = "Key for this field in parent map (synthetic to work around Terraform limitations)"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildProductDefaultPriceDataCurrencyOptionsEl {
    pub fn build(self) -> ProductDefaultPriceDataCurrencyOptionsEl {
        ProductDefaultPriceDataCurrencyOptionsEl {
            custom_unit_amount_enabled: core::default::Default::default(),
            custom_unit_amount_maximum: core::default::Default::default(),
            custom_unit_amount_minimum: core::default::Default::default(),
            custom_unit_amount_preset: core::default::Default::default(),
            key: self.key,
            tax_behavior: core::default::Default::default(),
            unit_amount: core::default::Default::default(),
            unit_amount_decimal: core::default::Default::default(),
            tiers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ProductDefaultPriceDataCurrencyOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProductDefaultPriceDataCurrencyOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ProductDefaultPriceDataCurrencyOptionsElRef {
        ProductDefaultPriceDataCurrencyOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProductDefaultPriceDataCurrencyOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_unit_amount_enabled` after provisioning.\n"]
    pub fn custom_unit_amount_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_unit_amount_enabled", self.base))
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

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nKey for this field in parent map (synthetic to work around Terraform limitations)"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `tax_behavior` after provisioning.\n"]
    pub fn tax_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tax_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount` after provisioning.\n"]
    pub fn unit_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_amount_decimal` after provisioning.\n"]
    pub fn unit_amount_decimal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_amount_decimal", self.base))
    }

    #[doc = "Get a reference to the value of field `tiers` after provisioning.\n"]
    pub fn tiers(&self) -> terrars::ListRef<ProductDefaultPriceDataCurrencyOptionsElTiersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tiers", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProductDynamic {
    default_price_data_currency_options: std
    ::option
    ::Option<terrars::DynamicBlock<ProductDefaultPriceDataCurrencyOptionsEl>>,
}
