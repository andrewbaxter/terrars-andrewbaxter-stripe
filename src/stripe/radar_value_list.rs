use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct RadarValueListData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    alias: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_type: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct RadarValueList_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RadarValueListData>,
}

#[derive(Clone)]
pub struct RadarValueList(Rc<RadarValueList_>);

impl RadarValueList {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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

    #[doc= "Set the field `item_type`.\nType of the items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`. Use `string` if the item type is unknown or mixed."]
    pub fn set_item_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().item_type = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nThe name of the value list for use in rules."]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_by` after provisioning.\nThe name or email address of the user who created this value list."]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `item_type` after provisioning.\nType of the items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`. Use `string` if the item type is unknown or mixed."]
    pub fn item_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.item_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_data` after provisioning.\nDetails about each object."]
    pub fn list_items_data(&self) -> ListRef<RadarValueListListItemsDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.list_items_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn list_items_has_more(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_items_has_more", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn list_items_object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_items_object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn list_items_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_items_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the value list."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }
}

impl Referable for RadarValueList {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for RadarValueList { }

impl ToListMappable for RadarValueList {
    type O = ListRef<RadarValueListRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for RadarValueList_ {
    fn extract_resource_type(&self) -> String {
        "stripe_radar_value_list".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRadarValueList {
    pub tf_id: String,
    #[doc= "The name of the value list for use in rules."]
    pub alias: PrimField<String>,
    #[doc= "The human-readable name of the value list."]
    pub name: PrimField<String>,
}

impl BuildRadarValueList {
    pub fn build(self, stack: &mut Stack) -> RadarValueList {
        let out = RadarValueList(Rc::new(RadarValueList_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RadarValueListData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for RadarValueListRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RadarValueListRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alias` after provisioning.\nThe name of the value list for use in rules."]
    pub fn alias(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.alias", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_by` after provisioning.\nThe name or email address of the user who created this value list."]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `item_type` after provisioning.\nType of the items in the value list. One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`. Use `string` if the item type is unknown or mixed."]
    pub fn item_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.item_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_data` after provisioning.\nDetails about each object."]
    pub fn list_items_data(&self) -> ListRef<RadarValueListListItemsDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.list_items_data", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn list_items_has_more(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_items_has_more", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn list_items_object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_items_object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `list_items_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn list_items_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.list_items_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe human-readable name of the value list."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RadarValueListListItemsDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    livemode: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    object: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_list: Option<PrimField<String>>,
}

impl RadarValueListListItemsDataEl {
    #[doc= "Set the field `created`.\n"]
    pub fn set_created(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.created = Some(v.into());
        self
    }

    #[doc= "Set the field `created_by`.\n"]
    pub fn set_created_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_by = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `livemode`.\n"]
    pub fn set_livemode(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.livemode = Some(v.into());
        self
    }

    #[doc= "Set the field `object`.\n"]
    pub fn set_object(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.object = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }

    #[doc= "Set the field `value_list`.\n"]
    pub fn set_value_list(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value_list = Some(v.into());
        self
    }
}

impl ToListMappable for RadarValueListListItemsDataEl {
    type O = BlockAssignable<RadarValueListListItemsDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for RadarValueListListItemsDataElRef {
    fn new(shared: StackShared, base: String) -> RadarValueListListItemsDataElRef {
        RadarValueListListItemsDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RadarValueListListItemsDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.base))
    }

    #[doc= "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\n"]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.base))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\n"]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc= "Get a reference to the value of field `value_list` after provisioning.\n"]
    pub fn value_list(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value_list", self.base))
    }
}
