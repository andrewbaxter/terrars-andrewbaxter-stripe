use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct RadarValueListData {
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
    alias: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_type: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

struct RadarValueList_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RadarValueListData>,
}

#[derive(Clone)]
pub struct RadarValueList(std::rc::Rc<RadarValueList_>);

impl RadarValueList {
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
        "Set the field `item_type`.\nType of the items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`. Use `string` if the item type is unknown or mixed."]
    pub fn set_item_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().item_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nThe name of the value list for use in rules."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_by` after provisioning.\nThe name or email address of the user who created this value list."]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `item_type` after provisioning.\nType of the items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`. Use `string` if the item type is unknown or mixed."]
    pub fn item_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_items_data` after provisioning.\nDetails about each object."]
    pub fn list_items_data(&self) -> terrars::ListRef<RadarValueListListItemsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_items_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_items_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn list_items_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_items_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_items_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn list_items_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_items_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_items_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn list_items_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_items_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the value list."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }
}

impl terrars::Referable for RadarValueList {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RadarValueList { }

impl terrars::ToListMappable for RadarValueList {
    type O = terrars::ListRef<RadarValueListRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RadarValueList_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_radar_value_list".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRadarValueList {
    pub tf_id: std::string::String,
    #[doc = "The name of the value list for use in rules."]
    pub alias: terrars::PrimField<std::string::String>,
    #[doc = "The human-readable name of the value list."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRadarValueList {
    pub fn build(self, stack: &mut terrars::Stack) -> RadarValueList {
        let out = RadarValueList(std::rc::Rc::new(RadarValueList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RadarValueListData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                alias: self.alias,
                item_type: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RadarValueListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RadarValueListRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RadarValueListRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `alias` after provisioning.\nThe name of the value list for use in rules."]
    pub fn alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_by` after provisioning.\nThe name or email address of the user who created this value list."]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `item_type` after provisioning.\nType of the items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`. Use `string` if the item type is unknown or mixed."]
    pub fn item_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_items_data` after provisioning.\nDetails about each object."]
    pub fn list_items_data(&self) -> terrars::ListRef<RadarValueListListItemsDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_items_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_items_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn list_items_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_items_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_items_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn list_items_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_items_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_items_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn list_items_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.list_items_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the value list."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RadarValueListListItemsDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    livemode: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_list: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RadarValueListListItemsDataEl {
    #[doc = "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.created = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_by`.\n"]
    pub fn set_created_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_by = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_list`.\n"]
    pub fn set_value_list(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RadarValueListListItemsDataEl {
    type O = terrars::BlockAssignable<RadarValueListListItemsDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRadarValueListListItemsDataEl {}

impl BuildRadarValueListListItemsDataEl {
    pub fn build(self) -> RadarValueListListItemsDataEl {
        RadarValueListListItemsDataEl {
            created: core::default::Default::default(),
            created_by: core::default::Default::default(),
            id: core::default::Default::default(),
            livemode: core::default::Default::default(),
            object: core::default::Default::default(),
            value: core::default::Default::default(),
            value_list: core::default::Default::default(),
        }
    }
}

pub struct RadarValueListListItemsDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RadarValueListListItemsDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RadarValueListListItemsDataElRef {
        RadarValueListListItemsDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RadarValueListListItemsDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
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

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `value_list` after provisioning.\n"]
    pub fn value_list(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_list", self.base))
    }
}
