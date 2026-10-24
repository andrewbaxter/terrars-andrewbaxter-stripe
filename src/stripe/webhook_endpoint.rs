use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct WebhookEndpointData {
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
    api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    enabled_events: terrars::ListField<terrars::PrimField<std::string::String>>,
    url: terrars::PrimField<std::string::String>,
}

struct WebhookEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WebhookEndpointData>,
}

#[derive(Clone)]
pub struct WebhookEndpoint(std::rc::Rc<WebhookEndpoint_>);

impl WebhookEndpoint {
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
        "Set the field `api_version`.\nEvents sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version."]
    pub fn set_api_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connect`.\nWhether this endpoint should receive events from connected accounts (`true`), or from your account (`false`). Defaults to `false`."]
    pub fn set_connect(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().connect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of what the webhook is used for."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `api_version` after provisioning.\nEvents sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version."]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nThe ID of the associated Connect application."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connect` after provisioning.\nWhether this endpoint should receive events from connected accounts (`true`), or from your account (`false`). Defaults to `false`."]
    pub fn connect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of what the webhook is used for."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled_events` after provisioning.\nThe list of events to enable for this endpoint. You may specify `['*']` to enable all events, except those that require explicit selection."]
    pub fn enabled_events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
        "Get a reference to the value of field `secret` after provisioning.\nThe endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures). Only returned at creation."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the webhook. It can be `enabled` or `disabled`."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the webhook endpoint."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl terrars::Referable for WebhookEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WebhookEndpoint { }

impl terrars::ToListMappable for WebhookEndpoint {
    type O = terrars::ListRef<WebhookEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WebhookEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_webhook_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWebhookEndpoint {
    pub tf_id: std::string::String,
    #[doc =
        "The list of events to enable for this endpoint. You may specify `['*']` to enable all events, except those that require explicit selection."]
    pub enabled_events: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The URL of the webhook endpoint."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildWebhookEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> WebhookEndpoint {
        let out = WebhookEndpoint(std::rc::Rc::new(WebhookEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WebhookEndpointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WebhookEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WebhookEndpointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_version` after provisioning.\nEvents sent to this endpoint will be generated with this Stripe Version instead of your account's default Stripe Version."]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nThe ID of the associated Connect application."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connect` after provisioning.\nWhether this endpoint should receive events from connected accounts (`true`), or from your account (`false`). Defaults to `false`."]
    pub fn connect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of what the webhook is used for."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enabled_events` after provisioning.\nThe list of events to enable for this endpoint. You may specify `['*']` to enable all events, except those that require explicit selection."]
    pub fn enabled_events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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
        "Get a reference to the value of field `secret` after provisioning.\nThe endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures). Only returned at creation."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of the webhook. It can be `enabled` or `disabled`."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL of the webhook endpoint."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}
