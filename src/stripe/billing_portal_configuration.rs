#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct BillingPortalConfigurationData_ {
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
    business_profile_headline: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_privacy_policy_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_terms_of_service_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_return_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_customer_update_allowed_updates: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_customer_update_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_invoice_history_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_payment_method_update_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_cancel_cancellation_reason_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_cancel_cancellation_reason_options: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_cancel_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_cancel_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_cancel_proration_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_pause_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_update_default_allowed_updates: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_update_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_update_proration_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_page_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    features_subscription_update_products: std
    ::option
    ::Option<std::vec::Vec<BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts>>,
    dynamic: BillingPortalConfigurationDynamic,
}

struct BillingPortalConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BillingPortalConfigurationData_>,
}

#[derive(Clone)]
pub struct BillingPortalConfiguration(std::rc::Rc<BillingPortalConfiguration_>);

impl BillingPortalConfiguration {
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

    #[doc = "Set the field `business_profile_headline`.\nThe messaging shown to customers in the portal."]
    pub fn set_business_profile_headline(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_headline = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_privacy_policy_url`.\nA link to the business’s publicly available privacy policy."]
    pub fn set_business_profile_privacy_policy_url(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_privacy_policy_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_terms_of_service_url`.\nA link to the business’s publicly available terms of service."]
    pub fn set_business_profile_terms_of_service_url(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_terms_of_service_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_return_url`.\nThe default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session."]
    pub fn set_default_return_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_return_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_customer_update_allowed_updates`.\nThe types of customer updates that are supported. When empty, customers are not updateable."]
    pub fn set_features_customer_update_allowed_updates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().features_customer_update_allowed_updates = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_customer_update_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_customer_update_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_customer_update_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_invoice_history_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_invoice_history_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_invoice_history_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_payment_method_update_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_payment_method_update_enabled(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().features_payment_method_update_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_subscription_cancel_cancellation_reason_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_cancel_cancellation_reason_enabled(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_cancellation_reason_enabled =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_subscription_cancel_cancellation_reason_options`.\nWhich cancellation reasons will be given as options to the customer."]
    pub fn set_features_subscription_cancel_cancellation_reason_options(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_cancellation_reason_options =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_subscription_cancel_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_cancel_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_subscription_cancel_mode`.\nWhether to cancel subscriptions immediately or at the end of the billing period."]
    pub fn set_features_subscription_cancel_mode(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_subscription_cancel_proration_behavior`.\nWhether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`."]
    pub fn set_features_subscription_cancel_proration_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_proration_behavior =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_subscription_pause_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_pause_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_pause_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_subscription_update_default_allowed_updates`.\nThe types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable."]
    pub fn set_features_subscription_update_default_allowed_updates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_update_default_allowed_updates =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_subscription_update_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_update_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_update_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `features_subscription_update_proration_behavior`.\nDetermines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`."]
    pub fn set_features_subscription_update_proration_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_update_proration_behavior =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `login_page_enabled`.\nIf `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.\n\nIf `false`, the previously generated `url`, if any, will be deactivated."]
    pub fn set_login_page_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().login_page_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `features_subscription_update_products`.\n"]
    pub fn set_features_subscription_update_products(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().features_subscription_update_products = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.features_subscription_update_products =
                    std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the configuration."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_headline` after provisioning.\nThe messaging shown to customers in the portal."]
    pub fn business_profile_headline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_headline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_privacy_policy_url` after provisioning.\nA link to the business’s publicly available privacy policy."]
    pub fn business_profile_privacy_policy_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_privacy_policy_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_terms_of_service_url` after provisioning.\nA link to the business’s publicly available terms of service."]
    pub fn business_profile_terms_of_service_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_terms_of_service_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_return_url` after provisioning.\nThe default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session."]
    pub fn default_return_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_return_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `features_customer_update_allowed_updates` after provisioning.\nThe types of customer updates that are supported. When empty, customers are not updateable."]
    pub fn features_customer_update_allowed_updates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_customer_update_allowed_updates", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_customer_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_customer_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_customer_update_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_invoice_history_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_invoice_history_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_invoice_history_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_payment_method_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_payment_method_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_payment_method_update_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_cancellation_reason_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_options` after provisioning.\nWhich cancellation reasons will be given as options to the customer."]
    pub fn features_subscription_cancel_cancellation_reason_options(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_mode` after provisioning.\nWhether to cancel subscriptions immediately or at the end of the billing period."]
    pub fn features_subscription_cancel_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_mode", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_proration_behavior` after provisioning.\nWhether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`."]
    pub fn features_subscription_cancel_proration_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_proration_behavior", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_pause_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_pause_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_pause_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_update_default_allowed_updates` after provisioning.\nThe types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable."]
    pub fn features_subscription_update_default_allowed_updates(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_default_allowed_updates", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_update_proration_behavior` after provisioning.\nDetermines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`."]
    pub fn features_subscription_update_proration_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_proration_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default` after provisioning.\nWhether the configuration is the default. If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session."]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `login_page_enabled` after provisioning.\nIf `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.\n\nIf `false`, the previously generated `url`, if any, will be deactivated."]
    pub fn login_page_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_page_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `login_page_url` after provisioning.\nA shareable URL to the hosted portal login page. Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal."]
    pub fn login_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_page_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nTime at which the object was last updated. Measured in seconds since the Unix epoch."]
    pub fn updated(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `features_subscription_update_products` after provisioning.\n"]
    pub fn features_subscription_update_products(
        &self,
    ) -> terrars::ListRef<BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_products", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BillingPortalConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BillingPortalConfiguration { }

impl terrars::ToListMappable for BillingPortalConfiguration {
    type O = terrars::ListRef<BillingPortalConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BillingPortalConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_billing_portal_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBillingPortalConfiguration {
    pub tf_id: std::string::String,
}

impl BuildBillingPortalConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> BillingPortalConfiguration {
        let out = BillingPortalConfiguration(std::rc::Rc::new(BillingPortalConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BillingPortalConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                business_profile_headline: core::default::Default::default(),
                business_profile_privacy_policy_url: core::default::Default::default(),
                business_profile_terms_of_service_url: core::default::Default::default(),
                default_return_url: core::default::Default::default(),
                features_customer_update_allowed_updates: core::default::Default::default(),
                features_customer_update_enabled: core::default::Default::default(),
                features_invoice_history_enabled: core::default::Default::default(),
                features_payment_method_update_enabled: core::default::Default::default(),
                features_subscription_cancel_cancellation_reason_enabled: core::default::Default::default(),
                features_subscription_cancel_cancellation_reason_options: core::default::Default::default(),
                features_subscription_cancel_enabled: core::default::Default::default(),
                features_subscription_cancel_mode: core::default::Default::default(),
                features_subscription_cancel_proration_behavior: core::default::Default::default(),
                features_subscription_pause_enabled: core::default::Default::default(),
                features_subscription_update_default_allowed_updates: core::default::Default::default(),
                features_subscription_update_enabled: core::default::Default::default(),
                features_subscription_update_proration_behavior: core::default::Default::default(),
                login_page_enabled: core::default::Default::default(),
                features_subscription_update_products: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BillingPortalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingPortalConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BillingPortalConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the configuration."]
    pub fn application(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_headline` after provisioning.\nThe messaging shown to customers in the portal."]
    pub fn business_profile_headline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_headline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_privacy_policy_url` after provisioning.\nA link to the business’s publicly available privacy policy."]
    pub fn business_profile_privacy_policy_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_privacy_policy_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_terms_of_service_url` after provisioning.\nA link to the business’s publicly available terms of service."]
    pub fn business_profile_terms_of_service_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_terms_of_service_url", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_return_url` after provisioning.\nThe default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session."]
    pub fn default_return_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_return_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `features_customer_update_allowed_updates` after provisioning.\nThe types of customer updates that are supported. When empty, customers are not updateable."]
    pub fn features_customer_update_allowed_updates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_customer_update_allowed_updates", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_customer_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_customer_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_customer_update_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_invoice_history_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_invoice_history_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_invoice_history_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_payment_method_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_payment_method_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_payment_method_update_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_cancellation_reason_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_options` after provisioning.\nWhich cancellation reasons will be given as options to the customer."]
    pub fn features_subscription_cancel_cancellation_reason_options(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_options", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_mode` after provisioning.\nWhether to cancel subscriptions immediately or at the end of the billing period."]
    pub fn features_subscription_cancel_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_mode", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_cancel_proration_behavior` after provisioning.\nWhether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`."]
    pub fn features_subscription_cancel_proration_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_proration_behavior", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_pause_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_pause_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_pause_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_update_default_allowed_updates` after provisioning.\nThe types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable."]
    pub fn features_subscription_update_default_allowed_updates(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_default_allowed_updates", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_update_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_enabled", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `features_subscription_update_proration_behavior` after provisioning.\nDetermines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`."]
    pub fn features_subscription_update_proration_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_proration_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_default` after provisioning.\nWhether the configuration is the default. If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session."]
    pub fn is_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `login_page_enabled` after provisioning.\nIf `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.\n\nIf `false`, the previously generated `url`, if any, will be deactivated."]
    pub fn login_page_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_page_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `login_page_url` after provisioning.\nA shareable URL to the hosted portal login page. Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal."]
    pub fn login_page_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_page_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated` after provisioning.\nTime at which the object was last updated. Measured in seconds since the Unix epoch."]
    pub fn updated(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `features_subscription_update_products` after provisioning.\n"]
    pub fn features_subscription_update_products(
        &self,
    ) -> terrars::ListRef<BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_products", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BillingPortalConfigurationDynamic {
    features_subscription_update_products: std
    ::option
    ::Option<terrars::DynamicBlock<BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts>>,
}

#[derive(Serialize)]
pub struct BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts {
    prices: terrars::ListField<terrars::PrimField<std::string::String>>,
    product: terrars::PrimField<std::string::String>,
}

impl BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts { }

impl terrars::ToListMappable for BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts {
    type O = terrars::BlockAssignable<BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingPortalConfiguration_FeaturesSubscriptionUpdateProducts {
    #[doc = "The list of price IDs which, when subscribed to, a subscription can be updated."]
    pub prices: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The product ID."]
    pub product: terrars::PrimField<std::string::String>,
}

impl BuildBillingPortalConfiguration_FeaturesSubscriptionUpdateProducts {
    pub fn build(self) -> BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts {
        BillingPortalConfiguration_FeaturesSubscriptionUpdateProducts {
            prices: self.prices,
            product: self.product,
        }
    }
}

#[allow(dead_code)]
pub struct BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef {
        BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingPortalConfiguration_FeaturesSubscriptionUpdateProductsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `prices` after provisioning.\nThe list of price IDs which, when subscribed to, a subscription can be updated."]
    pub fn prices(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prices", self.base))
    }

    #[doc = "Get a reference to the value of field `product` after provisioning.\nThe product ID."]
    pub fn product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product", self.base))
    }
}
