use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct TerminalConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bbpos_wisepos_e_splashscreen: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_aud_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_aud_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_aud_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_cad_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_cad_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_cad_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_chf_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_chf_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_chf_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_czk_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_czk_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_czk_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_dkk_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_dkk_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_dkk_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_eur_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_eur_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_eur_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_gbp_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_gbp_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_gbp_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_hkd_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_hkd_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_hkd_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_myr_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_myr_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_myr_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_nok_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_nok_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_nok_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_nzd_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_nzd_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_nzd_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_sek_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_sek_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_sek_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_sgd_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_sgd_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_sgd_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_usd_fixed_amounts: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_usd_percentages: Option<ListField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipping_usd_smart_tip_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verifone_p400_splashscreen: Option<PrimField<String>>,
}

struct TerminalConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<TerminalConfigurationData>,
}

#[derive(Clone)]
pub struct TerminalConfiguration(Rc<TerminalConfiguration_>);

impl TerminalConfiguration {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `bbpos_wisepos_e_splashscreen`.\nA File ID representing an image you would like displayed on the reader."]
    pub fn set_bbpos_wisepos_e_splashscreen(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bbpos_wisepos_e_splashscreen = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_aud_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_aud_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_aud_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_aud_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_aud_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_aud_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_aud_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_aud_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_aud_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_cad_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_cad_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_cad_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_cad_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_cad_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_cad_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_cad_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_cad_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_cad_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_chf_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_chf_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_chf_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_chf_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_chf_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_chf_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_chf_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_chf_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_chf_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_czk_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_czk_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_czk_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_czk_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_czk_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_czk_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_czk_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_czk_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_czk_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_dkk_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_dkk_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_dkk_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_dkk_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_dkk_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_dkk_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_dkk_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_dkk_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_dkk_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_eur_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_eur_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_eur_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_eur_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_eur_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_eur_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_eur_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_eur_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_eur_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_gbp_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_gbp_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_gbp_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_gbp_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_gbp_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_gbp_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_gbp_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_gbp_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_gbp_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_hkd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_hkd_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_hkd_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_hkd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_hkd_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_hkd_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_hkd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_hkd_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_hkd_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_myr_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_myr_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_myr_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_myr_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_myr_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_myr_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_myr_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_myr_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_myr_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_nok_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_nok_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_nok_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_nok_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_nok_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_nok_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_nok_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_nok_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_nok_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_nzd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_nzd_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_nzd_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_nzd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_nzd_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_nzd_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_nzd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_nzd_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_nzd_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_sek_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_sek_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_sek_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_sek_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_sek_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_sek_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_sek_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_sek_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_sek_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_sgd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_sgd_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_sgd_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_sgd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_sgd_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_sgd_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_sgd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_sgd_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_sgd_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_usd_fixed_amounts`.\nFixed amounts displayed when collecting a tip"]
    pub fn set_tipping_usd_fixed_amounts(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_usd_fixed_amounts = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_usd_percentages`.\nPercentages displayed when collecting a tip"]
    pub fn set_tipping_usd_percentages(self, v: impl Into<ListField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().tipping_usd_percentages = Some(v.into());
        self
    }

    #[doc= "Set the field `tipping_usd_smart_tip_threshold`.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn set_tipping_usd_smart_tip_threshold(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tipping_usd_smart_tip_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `verifone_p400_splashscreen`.\nA File ID representing an image you would like displayed on the reader."]
    pub fn set_verifone_p400_splashscreen(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().verifone_p400_splashscreen = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `bbpos_wisepos_e_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn bbpos_wisepos_e_splashscreen(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bbpos_wisepos_e_splashscreen", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_account_default` after provisioning.\nWhether this Configuration is the default for your account"]
    pub fn is_account_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_account_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_aud_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_aud_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_aud_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_aud_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_aud_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_aud_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_aud_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_aud_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_aud_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_cad_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_cad_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_cad_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_cad_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_cad_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_cad_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_cad_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_cad_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_cad_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_chf_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_chf_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_chf_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_chf_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_chf_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_chf_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_chf_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_chf_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_chf_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_czk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_czk_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_czk_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_czk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_czk_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_czk_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_czk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_czk_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_czk_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_dkk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_dkk_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_dkk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_dkk_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_dkk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_dkk_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_dkk_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_eur_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_eur_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_eur_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_eur_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_eur_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_eur_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_eur_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_eur_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_eur_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_gbp_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_gbp_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_gbp_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_gbp_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_gbp_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_gbp_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_gbp_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_hkd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_hkd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_hkd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_hkd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_hkd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_hkd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_hkd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_myr_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_myr_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_myr_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_myr_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_myr_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_myr_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_myr_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_myr_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_myr_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nok_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nok_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nok_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nok_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nok_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nok_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nok_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nok_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_nok_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nzd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nzd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nzd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nzd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nzd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nzd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_nzd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sek_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sek_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sek_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sek_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sek_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sek_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sek_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sek_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_sek_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sgd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sgd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sgd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sgd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sgd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sgd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_sgd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_usd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_usd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_usd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_usd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_usd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_usd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_usd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_usd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_usd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verifone_p400_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn verifone_p400_splashscreen(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verifone_p400_splashscreen", self.extract_ref()))
    }
}

impl Resource for TerminalConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for TerminalConfiguration {
    type O = ListRef<TerminalConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for TerminalConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "stripe_terminal_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTerminalConfiguration {
    pub tf_id: String,
}

impl BuildTerminalConfiguration {
    pub fn build(self, stack: &mut Stack) -> TerminalConfiguration {
        let out = TerminalConfiguration(Rc::new(TerminalConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(TerminalConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for TerminalConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TerminalConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bbpos_wisepos_e_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn bbpos_wisepos_e_splashscreen(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bbpos_wisepos_e_splashscreen", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `is_account_default` after provisioning.\nWhether this Configuration is the default for your account"]
    pub fn is_account_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_account_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `livemode` after provisioning.\nHas the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    pub fn livemode(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.livemode", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_aud_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_aud_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_aud_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_aud_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_aud_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_aud_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_aud_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_aud_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_aud_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_cad_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_cad_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_cad_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_cad_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_cad_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_cad_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_cad_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_cad_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_cad_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_chf_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_chf_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_chf_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_chf_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_chf_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_chf_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_chf_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_chf_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_chf_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_czk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_czk_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_czk_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_czk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_czk_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_czk_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_czk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_czk_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_czk_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_dkk_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_dkk_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_dkk_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_dkk_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_dkk_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_dkk_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_dkk_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_dkk_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_eur_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_eur_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_eur_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_eur_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_eur_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_eur_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_eur_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_eur_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_eur_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_gbp_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_gbp_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_gbp_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_gbp_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_gbp_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_gbp_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_gbp_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_gbp_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_hkd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_hkd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_hkd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_hkd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_hkd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_hkd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_hkd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_hkd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_myr_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_myr_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_myr_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_myr_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_myr_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_myr_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_myr_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_myr_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_myr_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nok_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nok_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nok_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nok_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nok_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nok_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nok_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nok_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_nok_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nzd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_nzd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nzd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_nzd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_nzd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_nzd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_nzd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_nzd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sek_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sek_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sek_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sek_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sek_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sek_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sek_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sek_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_sek_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sgd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_sgd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sgd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_sgd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_sgd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_sgd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_sgd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_sgd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_usd_fixed_amounts` after provisioning.\nFixed amounts displayed when collecting a tip"]
    pub fn tipping_usd_fixed_amounts(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_usd_fixed_amounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_usd_percentages` after provisioning.\nPercentages displayed when collecting a tip"]
    pub fn tipping_usd_percentages(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.tipping_usd_percentages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tipping_usd_smart_tip_threshold` after provisioning.\nBelow this amount, fixed amounts will be displayed; above it, percentages will be displayed"]
    pub fn tipping_usd_smart_tip_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.tipping_usd_smart_tip_threshold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `verifone_p400_splashscreen` after provisioning.\nA File ID representing an image you would like displayed on the reader."]
    pub fn verifone_p400_splashscreen(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verifone_p400_splashscreen", self.extract_ref()))
    }
}
