use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct TerminalLocationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_city: Option<PrimField<String>>,
    address_country: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_line2: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_postal_code: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address_state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configuration_overrides: Option<PrimField<String>>,
    display_name: PrimField<String>,
}

struct TerminalLocation_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TerminalLocationData>,
}

#[derive(Clone)]
pub struct TerminalLocation(Rc<TerminalLocation_>);

impl TerminalLocation {
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

    #[doc= "Set the field `address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_address_city(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_city = Some(v.into());
        self
    }

    #[doc= "Set the field `address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_address_line1(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_line1 = Some(v.into());
        self
    }

    #[doc= "Set the field `address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_address_line2(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_line2 = Some(v.into());
        self
    }

    #[doc= "Set the field `address_postal_code`.\nZIP or postal code."]
    pub fn set_address_postal_code(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_postal_code = Some(v.into());
        self
    }

    #[doc= "Set the field `address_state`.\nState, county, province, or region."]
    pub fn set_address_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().address_state = Some(v.into());
        self
    }

    #[doc= "Set the field `configuration_overrides`.\nThe ID of a configuration that will be used to customize all readers in this location."]
    pub fn set_configuration_overrides(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().configuration_overrides = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn address_city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_city", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn address_country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_country", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn address_line1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_line1", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn address_line2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_line2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn address_postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_postal_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_state` after provisioning.\nState, county, province, or region."]
    pub fn address_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration_overrides` after provisioning.\nThe ID of a configuration that will be used to customize all readers in this location."]
    pub fn configuration_overrides(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_overrides", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nA name for the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }
}

impl Resource for TerminalLocation {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for TerminalLocation {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for TerminalLocation {
    type O = ListRef<TerminalLocationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for TerminalLocation_ {
    fn extract_resource_type(&self) -> String {
        "stripe_terminal_location".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTerminalLocation {
    pub tf_id: String,
    #[doc= "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub address_country: PrimField<String>,
    #[doc= "A name for the location."]
    pub display_name: PrimField<String>,
}

impl BuildTerminalLocation {
    pub fn build(self, stack: &mut Stack) -> TerminalLocation {
        let out = TerminalLocation(Rc::new(TerminalLocation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TerminalLocationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                address_city: core::default::Default::default(),
                address_country: self.address_country,
                address_line1: core::default::Default::default(),
                address_line2: core::default::Default::default(),
                address_postal_code: core::default::Default::default(),
                address_state: core::default::Default::default(),
                configuration_overrides: core::default::Default::default(),
                display_name: self.display_name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TerminalLocationRef {
    shared: StackShared,
    base: String,
}

impl Ref for TerminalLocationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TerminalLocationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn address_city(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_city", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn address_country(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_country", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn address_line1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_line1", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn address_line2(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_line2", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn address_postal_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_postal_code", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `address_state` after provisioning.\nState, county, province, or region."]
    pub fn address_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `configuration_overrides` after provisioning.\nThe ID of a configuration that will be used to customize all readers in this location."]
    pub fn configuration_overrides(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.configuration_overrides", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nA name for the location."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }
}
