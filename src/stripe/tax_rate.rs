use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct TaxRateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    inclusive: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jurisdiction: Option<PrimField<String>>,
    percentage: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_type: Option<PrimField<String>>,
}

struct TaxRate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TaxRateData>,
}

#[derive(Clone)]
pub struct TaxRate(Rc<TaxRate_>);

impl TaxRate {
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

    #[doc= "Set the field `country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_country(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().country = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn arbitrary string attached to the tax rate for your internal use only. It will not be visible to your customers."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `jurisdiction`.\nThe jurisdiction for the tax rate. You can use this label field for tax reporting purposes. It also appears on your customer’s invoice."]
    pub fn set_jurisdiction(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().jurisdiction = Some(v.into());
        self
    }

    #[doc= "Set the field `state`.\n[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix. For example, \"NY\" for New York, United States."]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc= "Set the field `tax_type`.\nThe high-level tax type, such as `vat` or `sales_tax`."]
    pub fn set_tax_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().tax_type = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string attached to the tax rate for your internal use only. It will not be visible to your customers."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tax rate, which will be shown to users."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inclusive` after provisioning.\nThis specifies if the tax rate is inclusive or exclusive."]
    pub fn inclusive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jurisdiction` after provisioning.\nThe jurisdiction for the tax rate. You can use this label field for tax reporting purposes. It also appears on your customer’s invoice."]
    pub fn jurisdiction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThis represents the tax rate percent out of 100."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix. For example, \"NY\" for New York, United States."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_type` after provisioning.\nThe high-level tax type, such as `vat` or `sales_tax`."]
    pub fn tax_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_type", self.extract_ref()))
    }
}

impl Resource for TaxRate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for TaxRate {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for TaxRate {
    type O = ListRef<TaxRateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TaxRate_ {
    fn extract_resource_type(&self) -> String {
        "stripe_tax_rate".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTaxRate {
    pub tf_id: String,
    #[doc= "The display name of the tax rate, which will be shown to users."]
    pub display_name: PrimField<String>,
    #[doc= "This specifies if the tax rate is inclusive or exclusive."]
    pub inclusive: PrimField<bool>,
    #[doc= "This represents the tax rate percent out of 100."]
    pub percentage: PrimField<f64>,
}

impl BuildTaxRate {
    pub fn build(self, stack: &mut Stack) -> TaxRate {
        let out = TaxRate(Rc::new(TaxRate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TaxRateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                country: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                inclusive: self.inclusive,
                jurisdiction: core::default::Default::default(),
                percentage: self.percentage,
                state: core::default::Default::default(),
                tax_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TaxRateRef {
    shared: StackShared,
    base: String,
}

impl Ref for TaxRateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TaxRateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn arbitrary string attached to the tax rate for your internal use only. It will not be visible to your customers."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the tax rate, which will be shown to users."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inclusive` after provisioning.\nThis specifies if the tax rate is inclusive or exclusive."]
    pub fn inclusive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.inclusive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `jurisdiction` after provisioning.\nThe jurisdiction for the tax rate. You can use this label field for tax reporting purposes. It also appears on your customer’s invoice."]
    pub fn jurisdiction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.jurisdiction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `percentage` after provisioning.\nThis represents the tax rate percent out of 100."]
    pub fn percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n[ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix. For example, \"NY\" for New York, United States."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tax_type` after provisioning.\nThe high-level tax type, such as `vat` or `sales_tax`."]
    pub fn tax_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tax_type", self.extract_ref()))
    }
}
