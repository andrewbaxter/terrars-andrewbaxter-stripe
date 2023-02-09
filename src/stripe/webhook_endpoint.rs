use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct WebhookEndpointData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connect: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    enabled_events: ListField<PrimField<String>>,
    url: PrimField<String>,
}

struct WebhookEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WebhookEndpointData>,
}

#[derive(Clone)]
pub struct WebhookEndpoint(Rc<WebhookEndpoint_>);

impl WebhookEndpoint {
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

    #[doc= "Set the field `api_version`.\nEvents sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version."]
    pub fn set_api_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_version = Some(v.into());
        self
    }

    #[doc= "Set the field `connect`.\nWhether this endpoint should receive events from connected accounts (`true`), or from your account (`false`). Defaults to `false`."]
    pub fn set_connect(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().connect = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of what the webhook is used for."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `api_version` after provisioning.\nEvents sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version."]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `application` after provisioning.\nThe ID of the associated Connect application."]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connect` after provisioning.\nWhether this endpoint should receive events from connected accounts (`true`), or from your account (`false`). Defaults to `false`."]
    pub fn connect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of what the webhook is used for."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_events` after provisioning.\nThe list of events to enable for this endpoint. You may specify `['*']` to enable all events, except those that require explicit selection."]
    pub fn enabled_events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enabled_events", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nThe endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures). Only returned at creation."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the webhook. It can be `enabled` or `disabled`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the webhook endpoint."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl Resource for WebhookEndpoint {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for WebhookEndpoint {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for WebhookEndpoint {
    type O = ListRef<WebhookEndpointRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for WebhookEndpoint_ {
    fn extract_resource_type(&self) -> String {
        "stripe_webhook_endpoint".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWebhookEndpoint {
    pub tf_id: String,
    #[doc= "The list of events to enable for this endpoint. You may specify `['*']` to enable all events, except those that require explicit selection."]
    pub enabled_events: ListField<PrimField<String>>,
    #[doc= "The URL of the webhook endpoint."]
    pub url: PrimField<String>,
}

impl BuildWebhookEndpoint {
    pub fn build(self, stack: &mut Stack) -> WebhookEndpoint {
        let out = WebhookEndpoint(Rc::new(WebhookEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WebhookEndpointData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_version: core::default::Default::default(),
                connect: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled_events: self.enabled_events,
                url: self.url,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WebhookEndpointRef {
    shared: StackShared,
    base: String,
}

impl Ref for WebhookEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WebhookEndpointRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_version` after provisioning.\nEvents sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version."]
    pub fn api_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `application` after provisioning.\nThe ID of the associated Connect application."]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `connect` after provisioning.\nWhether this endpoint should receive events from connected accounts (`true`), or from your account (`false`). Defaults to `false`."]
    pub fn connect(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.connect", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of what the webhook is used for."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled_events` after provisioning.\nThe list of events to enable for this endpoint. You may specify `['*']` to enable all events, except those that require explicit selection."]
    pub fn enabled_events(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.enabled_events", self.extract_ref()))
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

    #[doc= "Get a reference to the value of field `secret` after provisioning.\nThe endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures). Only returned at creation."]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nThe status of the webhook. It can be `enabled` or `disabled`."]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\nThe URL of the webhook endpoint."]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
