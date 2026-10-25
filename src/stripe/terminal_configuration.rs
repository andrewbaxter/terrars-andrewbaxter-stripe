#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct TerminalConfigurationData_ {
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
    bbpos_wisepos_e_splashscreen: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_aud_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_aud_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_aud_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_cad_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_cad_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_cad_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_chf_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_chf_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_chf_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_czk_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_czk_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_czk_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_dkk_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_dkk_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_dkk_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_eur_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_eur_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_eur_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_gbp_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_gbp_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_gbp_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_hkd_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_hkd_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_hkd_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_myr_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_myr_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_myr_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_nok_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_nok_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_nok_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_nzd_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_nzd_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_nzd_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_sek_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_sek_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_sek_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_sgd_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_sgd_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_sgd_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_usd_fixed_amounts: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_usd_percentages: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tipping_usd_smart_tip_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verifone_p400_splashscreen: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct TerminalConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TerminalConfigurationData_>,
}

#[derive(Clone)]
pub struct TerminalConfiguration(std::rc::Rc<TerminalConfiguration_>);

impl TerminalConfiguration {
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
        "Set the field `bbpos_wisepos_e_splashscreen`.\nA File ID representing an image you would like displayed on the reader."]
    pub fn set_bbpos_wisepos_e_splashscreen(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().bbpos_wisepos_e_splashscreen = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_aud_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_aud_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_aud_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_aud_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_aud_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_aud_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_aud_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_aud_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_aud_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_cad_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_cad_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_cad_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_cad_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_cad_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_cad_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_cad_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_cad_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_cad_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_chf_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_chf_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_chf_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_chf_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_chf_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_chf_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_chf_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_chf_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_chf_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_czk_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_czk_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_czk_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_czk_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_czk_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_czk_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_czk_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_czk_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_czk_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_dkk_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_dkk_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_dkk_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_dkk_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_dkk_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_dkk_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_dkk_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_dkk_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_dkk_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_eur_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_eur_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_eur_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_eur_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_eur_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_eur_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_eur_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_eur_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_eur_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_gbp_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_gbp_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_gbp_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_gbp_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_gbp_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_gbp_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_gbp_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_gbp_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_gbp_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_hkd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_hkd_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_hkd_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_hkd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_hkd_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_hkd_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_hkd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_hkd_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_hkd_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_myr_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_myr_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_myr_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_myr_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_myr_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_myr_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_myr_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_myr_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_myr_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_nok_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_nok_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_nok_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_nok_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_nok_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_nok_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_nok_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_nok_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_nok_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_nzd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_nzd_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_nzd_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_nzd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_nzd_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_nzd_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_nzd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_nzd_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_nzd_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_sek_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_sek_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_sek_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_sek_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_sek_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_sek_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_sek_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_sek_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_sek_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_sgd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_sgd_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_sgd_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_sgd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_sgd_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_sgd_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_sgd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_sgd_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_sgd_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_usd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_usd_fixed_amounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_usd_fixed_amounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tipping_usd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_usd_percentages(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.0.data.borrow_mut().tipping_usd_percentages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tipping_usd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_usd_smart_tip_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_usd_smart_tip_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `verifone_p400_splashscreen`.\nA File ID representing an image you would like displayed on the reader."]
    pub fn set_verifone_p400_splashscreen(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().verifone_p400_splashscreen = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `bbpos_wisepos_e_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn bbpos_wisepos_e_splashscreen(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bbpos_wisepos_e_splashscreen", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_account_default` after provisioning.\nWhether this Configuration is the default for your account"]
    pub fn is_account_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_account_default", self.extract_ref()))
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
        "Get a reference to the value of field `tipping_aud_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_aud_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_aud_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_aud_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_aud_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_aud_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_aud_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_aud_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_aud_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_cad_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_cad_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_cad_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_cad_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_cad_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_cad_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_cad_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_cad_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_cad_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_chf_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_chf_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_chf_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_chf_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_chf_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_chf_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_chf_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_chf_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_chf_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_czk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_czk_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_czk_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_czk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_czk_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_czk_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_czk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_czk_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_czk_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_dkk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_dkk_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_dkk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_dkk_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_dkk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_dkk_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_dkk_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_eur_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_eur_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_eur_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_eur_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_eur_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_eur_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_eur_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_eur_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_eur_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_gbp_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_gbp_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_gbp_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_gbp_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_gbp_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_gbp_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_gbp_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_hkd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_hkd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_hkd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_hkd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_hkd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_hkd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_hkd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_myr_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_myr_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_myr_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_myr_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_myr_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_myr_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_myr_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_myr_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_myr_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_nok_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nok_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nok_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nok_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nok_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nok_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nok_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nok_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_nok_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_nzd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nzd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nzd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nzd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nzd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nzd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_nzd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_sek_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sek_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sek_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sek_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sek_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sek_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sek_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sek_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_sek_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_sgd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sgd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sgd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sgd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sgd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sgd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_sgd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_usd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_usd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_usd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_usd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_usd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_usd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_usd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_usd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_usd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `verifone_p400_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn verifone_p400_splashscreen(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verifone_p400_splashscreen", self.extract_ref()))
    }
}

impl terrars::Referable for TerminalConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TerminalConfiguration { }

impl terrars::ToListMappable for TerminalConfiguration {
    type O = terrars::ListRef<TerminalConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TerminalConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_terminal_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTerminalConfiguration {
    pub tf_id: std::string::String,
}

impl BuildTerminalConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> TerminalConfiguration {
        let out = TerminalConfiguration(std::rc::Rc::new(TerminalConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TerminalConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bbpos_wisepos_e_splashscreen: core::default::Default::default(),
                tipping_aud_fixed_amounts: core::default::Default::default(),
                tipping_aud_percentages: core::default::Default::default(),
                tipping_aud_smart_tip_threshold: core::default::Default::default(),
                tipping_cad_fixed_amounts: core::default::Default::default(),
                tipping_cad_percentages: core::default::Default::default(),
                tipping_cad_smart_tip_threshold: core::default::Default::default(),
                tipping_chf_fixed_amounts: core::default::Default::default(),
                tipping_chf_percentages: core::default::Default::default(),
                tipping_chf_smart_tip_threshold: core::default::Default::default(),
                tipping_czk_fixed_amounts: core::default::Default::default(),
                tipping_czk_percentages: core::default::Default::default(),
                tipping_czk_smart_tip_threshold: core::default::Default::default(),
                tipping_dkk_fixed_amounts: core::default::Default::default(),
                tipping_dkk_percentages: core::default::Default::default(),
                tipping_dkk_smart_tip_threshold: core::default::Default::default(),
                tipping_eur_fixed_amounts: core::default::Default::default(),
                tipping_eur_percentages: core::default::Default::default(),
                tipping_eur_smart_tip_threshold: core::default::Default::default(),
                tipping_gbp_fixed_amounts: core::default::Default::default(),
                tipping_gbp_percentages: core::default::Default::default(),
                tipping_gbp_smart_tip_threshold: core::default::Default::default(),
                tipping_hkd_fixed_amounts: core::default::Default::default(),
                tipping_hkd_percentages: core::default::Default::default(),
                tipping_hkd_smart_tip_threshold: core::default::Default::default(),
                tipping_myr_fixed_amounts: core::default::Default::default(),
                tipping_myr_percentages: core::default::Default::default(),
                tipping_myr_smart_tip_threshold: core::default::Default::default(),
                tipping_nok_fixed_amounts: core::default::Default::default(),
                tipping_nok_percentages: core::default::Default::default(),
                tipping_nok_smart_tip_threshold: core::default::Default::default(),
                tipping_nzd_fixed_amounts: core::default::Default::default(),
                tipping_nzd_percentages: core::default::Default::default(),
                tipping_nzd_smart_tip_threshold: core::default::Default::default(),
                tipping_sek_fixed_amounts: core::default::Default::default(),
                tipping_sek_percentages: core::default::Default::default(),
                tipping_sek_smart_tip_threshold: core::default::Default::default(),
                tipping_sgd_fixed_amounts: core::default::Default::default(),
                tipping_sgd_percentages: core::default::Default::default(),
                tipping_sgd_smart_tip_threshold: core::default::Default::default(),
                tipping_usd_fixed_amounts: core::default::Default::default(),
                tipping_usd_percentages: core::default::Default::default(),
                tipping_usd_smart_tip_threshold: core::default::Default::default(),
                verifone_p400_splashscreen: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TerminalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TerminalConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TerminalConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bbpos_wisepos_e_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn bbpos_wisepos_e_splashscreen(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bbpos_wisepos_e_splashscreen", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_account_default` after provisioning.\nWhether this Configuration is the default for your account"]
    pub fn is_account_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_account_default", self.extract_ref()))
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
        "Get a reference to the value of field `tipping_aud_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_aud_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_aud_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_aud_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_aud_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_aud_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_aud_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_aud_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_aud_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_cad_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_cad_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_cad_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_cad_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_cad_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_cad_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_cad_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_cad_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_cad_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_chf_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_chf_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_chf_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_chf_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_chf_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_chf_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_chf_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_chf_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_chf_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_czk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_czk_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_czk_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_czk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_czk_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_czk_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_czk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_czk_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_czk_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_dkk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_dkk_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_dkk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_dkk_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_dkk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_dkk_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_dkk_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_eur_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_eur_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_eur_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_eur_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_eur_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_eur_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_eur_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_eur_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_eur_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_gbp_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_gbp_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_gbp_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_gbp_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_gbp_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_gbp_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_gbp_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_hkd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_hkd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_hkd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_hkd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_hkd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_hkd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_hkd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_myr_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_myr_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_myr_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_myr_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_myr_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_myr_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_myr_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_myr_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_myr_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_nok_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nok_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nok_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nok_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nok_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nok_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nok_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nok_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_nok_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_nzd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nzd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nzd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nzd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_nzd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nzd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_nzd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_sek_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sek_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sek_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sek_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sek_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sek_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sek_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sek_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_sek_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_sgd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sgd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sgd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sgd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_sgd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sgd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_sgd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tipping_usd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_usd_fixed_amounts(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_usd_fixed_amounts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_usd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_usd_percentages(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tipping_usd_percentages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tipping_usd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_usd_smart_tip_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tipping_usd_smart_tip_threshold", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `verifone_p400_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn verifone_p400_splashscreen(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verifone_p400_splashscreen", self.extract_ref()))
    }
}
