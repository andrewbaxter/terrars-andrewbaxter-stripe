use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct BillingPortalConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile_headline: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile_privacy_policy_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile_terms_of_service_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_return_url: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_customer_update_allowed_updates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_customer_update_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_invoice_history_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_payment_method_update_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_cancel_cancellation_reason_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_cancel_cancellation_reason_options: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_cancel_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_cancel_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_cancel_proration_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_pause_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_update_default_allowed_updates: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_update_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_update_proration_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_page_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features_subscription_update_products: Option<Vec<BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl>>,
    dynamic: BillingPortalConfigurationDynamic,
}

struct BillingPortalConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BillingPortalConfigurationData>,
}

#[derive(Clone)]
pub struct BillingPortalConfiguration(Rc<BillingPortalConfiguration_>);

impl BillingPortalConfiguration {
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

    #[doc= "Set the field `business_profile_headline`.\nThe messaging shown to customers in the portal."]
    pub fn set_business_profile_headline(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().business_profile_headline = Some(v.into());
        self
    }

    #[doc= "Set the field `business_profile_privacy_policy_url`.\nA link to the business’s publicly available privacy policy."]
    pub fn set_business_profile_privacy_policy_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().business_profile_privacy_policy_url = Some(v.into());
        self
    }

    #[doc= "Set the field `business_profile_terms_of_service_url`.\nA link to the business’s publicly available terms of service."]
    pub fn set_business_profile_terms_of_service_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().business_profile_terms_of_service_url = Some(v.into());
        self
    }

    #[doc= "Set the field `default_return_url`.\nThe default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session."]
    pub fn set_default_return_url(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().default_return_url = Some(v.into());
        self
    }

    #[doc= "Set the field `features_customer_update_allowed_updates`.\nThe types of customer updates that are supported. When empty, customers are not updateable."]
    pub fn set_features_customer_update_allowed_updates(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().features_customer_update_allowed_updates = Some(v.into());
        self
    }

    #[doc= "Set the field `features_customer_update_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_customer_update_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_customer_update_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_invoice_history_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_invoice_history_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_invoice_history_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_payment_method_update_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_payment_method_update_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_payment_method_update_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_cancel_cancellation_reason_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_cancel_cancellation_reason_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_cancellation_reason_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_cancel_cancellation_reason_options`.\nWhich cancellation reasons will be given as options to the customer."]
    pub fn set_features_subscription_cancel_cancellation_reason_options(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_cancellation_reason_options = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_cancel_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_cancel_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_cancel_mode`.\nWhether to cancel subscriptions immediately or at the end of the billing period."]
    pub fn set_features_subscription_cancel_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_cancel_proration_behavior`.\nWhether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`."]
    pub fn set_features_subscription_cancel_proration_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().features_subscription_cancel_proration_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_pause_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_pause_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_pause_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_update_default_allowed_updates`.\nThe types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable."]
    pub fn set_features_subscription_update_default_allowed_updates(
        self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.0.data.borrow_mut().features_subscription_update_default_allowed_updates = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_update_enabled`.\nWhether the feature is enabled."]
    pub fn set_features_subscription_update_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().features_subscription_update_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_update_proration_behavior`.\nDetermines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`."]
    pub fn set_features_subscription_update_proration_behavior(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().features_subscription_update_proration_behavior = Some(v.into());
        self
    }

    #[doc= "Set the field `login_page_enabled`.\nIf `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.\n\nIf `false`, the previously generated `url`, if any, will be deactivated."]
    pub fn set_login_page_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().login_page_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `features_subscription_update_products`.\n"]
    pub fn set_features_subscription_update_products(
        self,
        v: impl Into<BlockAssignable<BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().features_subscription_update_products = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.features_subscription_update_products = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the configuration."]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `business_profile_headline` after provisioning.\nThe messaging shown to customers in the portal."]
    pub fn business_profile_headline(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business_profile_headline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `business_profile_privacy_policy_url` after provisioning.\nA link to the business’s publicly available privacy policy."]
    pub fn business_profile_privacy_policy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business_profile_privacy_policy_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `business_profile_terms_of_service_url` after provisioning.\nA link to the business’s publicly available terms of service."]
    pub fn business_profile_terms_of_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business_profile_terms_of_service_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_return_url` after provisioning.\nThe default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session."]
    pub fn default_return_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_return_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_customer_update_allowed_updates` after provisioning.\nThe types of customer updates that are supported. When empty, customers are not updateable."]
    pub fn features_customer_update_allowed_updates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.features_customer_update_allowed_updates", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_customer_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_customer_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_customer_update_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_invoice_history_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_invoice_history_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_invoice_history_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_payment_method_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_payment_method_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_payment_method_update_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_cancellation_reason_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_options` after provisioning.\nWhich cancellation reasons will be given as options to the customer."]
    pub fn features_subscription_cancel_cancellation_reason_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_options", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_cancel_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_mode` after provisioning.\nWhether to cancel subscriptions immediately or at the end of the billing period."]
    pub fn features_subscription_cancel_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_cancel_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_proration_behavior` after provisioning.\nWhether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`."]
    pub fn features_subscription_cancel_proration_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_proration_behavior", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_pause_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_pause_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_pause_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_default_allowed_updates` after provisioning.\nThe types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable."]
    pub fn features_subscription_update_default_allowed_updates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_default_allowed_updates", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_update_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_proration_behavior` after provisioning.\nDetermines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`."]
    pub fn features_subscription_update_proration_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_proration_behavior", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_default` after provisioning.\nWhether the configuration is the default. If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session."]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login_page_enabled` after provisioning.\nIf `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.\n\nIf `false`, the previously generated `url`, if any, will be deactivated."]
    pub fn login_page_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_page_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login_page_url` after provisioning.\nA shareable URL to the hosted portal login page. Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal."]
    pub fn login_page_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_page_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated` after provisioning.\nTime at which the object was last updated. Measured in seconds since the Unix epoch."]
    pub fn updated(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_products` after provisioning.\n"]
    pub fn features_subscription_update_products(
        &self,
    ) -> ListRef<BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features_subscription_update_products", self.extract_ref()))
    }
}

impl Resource for BillingPortalConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for BillingPortalConfiguration {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for BillingPortalConfiguration {
    type O = ListRef<BillingPortalConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for BillingPortalConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "stripe_billing_portal_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBillingPortalConfiguration {
    pub tf_id: String,
}

impl BuildBillingPortalConfiguration {
    pub fn build(self, stack: &mut Stack) -> BillingPortalConfiguration {
        let out = BillingPortalConfiguration(Rc::new(BillingPortalConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BillingPortalConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BillingPortalConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for BillingPortalConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BillingPortalConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `application` after provisioning.\nID of the Connect Application that created the configuration."]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `business_profile_headline` after provisioning.\nThe messaging shown to customers in the portal."]
    pub fn business_profile_headline(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business_profile_headline", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `business_profile_privacy_policy_url` after provisioning.\nA link to the business’s publicly available privacy policy."]
    pub fn business_profile_privacy_policy_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business_profile_privacy_policy_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `business_profile_terms_of_service_url` after provisioning.\nA link to the business’s publicly available terms of service."]
    pub fn business_profile_terms_of_service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.business_profile_terms_of_service_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_return_url` after provisioning.\nThe default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session."]
    pub fn default_return_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_return_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_customer_update_allowed_updates` after provisioning.\nThe types of customer updates that are supported. When empty, customers are not updateable."]
    pub fn features_customer_update_allowed_updates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.features_customer_update_allowed_updates", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_customer_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_customer_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_customer_update_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_invoice_history_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_invoice_history_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_invoice_history_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_payment_method_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_payment_method_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_payment_method_update_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_cancellation_reason_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_enabled", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_cancellation_reason_options` after provisioning.\nWhich cancellation reasons will be given as options to the customer."]
    pub fn features_subscription_cancel_cancellation_reason_options(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_cancellation_reason_options", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_cancel_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_cancel_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_mode` after provisioning.\nWhether to cancel subscriptions immediately or at the end of the billing period."]
    pub fn features_subscription_cancel_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_cancel_mode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_cancel_proration_behavior` after provisioning.\nWhether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`."]
    pub fn features_subscription_cancel_proration_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_cancel_proration_behavior", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_pause_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_pause_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_pause_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_default_allowed_updates` after provisioning.\nThe types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable."]
    pub fn features_subscription_update_default_allowed_updates(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_default_allowed_updates", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_enabled` after provisioning.\nWhether the feature is enabled."]
    pub fn features_subscription_update_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.features_subscription_update_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_proration_behavior` after provisioning.\nDetermines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`."]
    pub fn features_subscription_update_proration_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.features_subscription_update_proration_behavior", self.extract_ref()),
        )
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_default` after provisioning.\nWhether the configuration is the default. If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session."]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login_page_enabled` after provisioning.\nIf `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer portal.\n\nIf `false`, the previously generated `url`, if any, will be deactivated."]
    pub fn login_page_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_page_enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `login_page_url` after provisioning.\nA shareable URL to the hosted portal login page. Your customers will be able to log in with their [email](https://stripe.com/docs/api/customers/object#customer_object-email) and receive a link to their customer portal."]
    pub fn login_page_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.login_page_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `updated` after provisioning.\nTime at which the object was last updated. Measured in seconds since the Unix epoch."]
    pub fn updated(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features_subscription_update_products` after provisioning.\n"]
    pub fn features_subscription_update_products(
        &self,
    ) -> ListRef<BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features_subscription_update_products", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl {
    prices: ListField<PrimField<String>>,
    product: PrimField<String>,
}

impl BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl { }

impl ToListMappable for BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl {
    type O = BlockAssignable<BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl {
    #[doc= "The list of price IDs which, when subscribed to, a subscription can be updated."]
    pub prices: ListField<PrimField<String>>,
    #[doc= "The product ID."]
    pub product: PrimField<String>,
}

impl BuildBillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl {
    pub fn build(self) -> BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl {
        BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl {
            prices: self.prices,
            product: self.product,
        }
    }
}

pub struct BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef {
    fn new(shared: StackShared, base: String) -> BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef {
        BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingPortalConfigurationFeaturesSubscriptionUpdateProductsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `prices` after provisioning.\nThe list of price IDs which, when subscribed to, a subscription can be updated."]
    pub fn prices(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.prices", self.base))
    }

    #[doc= "Get a reference to the value of field `product` after provisioning.\nThe product ID."]
    pub fn product(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.product", self.base))
    }
}

#[derive(Serialize, Default)]
struct BillingPortalConfigurationDynamic {
    features_subscription_update_products: Option<
        DynamicBlock<BillingPortalConfigurationFeaturesSubscriptionUpdateProductsEl>,
    >,
}
