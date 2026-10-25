#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderStripe;

#[derive(Serialize)]
struct AccountData_ {
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
    account_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bank_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_mcc: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_product_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_address_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_address_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_address_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_address_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_address_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_address_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_phone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_support_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_profile_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_acss_debit_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_affirm_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_afterpay_clearpay_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_au_becs_debit_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_bacs_debit_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_bancontact_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_bank_transfer_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_blik_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_boleto_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_card_issuing_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_card_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_cartes_bancaires_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_eps_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_fpx_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_giropay_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_grabpay_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_ideal_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_jcb_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_klarna_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_konbini_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_legacy_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_link_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_oxxo_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_p24_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_paynow_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_promptpay_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_sepa_debit_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_sofort_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_tax_reporting_us_1099_k_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_tax_reporting_us_1099_misc_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_transfers_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_treasury_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities_us_bank_account_ach_payments_requested: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kana_town: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_kanji_town: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_address_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_directors_provided: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_executives_provided: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name_kana: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name_kanji: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_owners_provided: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_ownership_declaration_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_ownership_declaration_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_ownership_declaration_user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_phone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_registration_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_structure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_tax_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_tax_id_registrar: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_vat_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_verification_document_back: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_verification_document_front: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_bank_account_ownership_verification_files: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_company_license_files: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_company_memorandum_of_association_files: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_company_ministerial_decree_files: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_company_registration_verification_files: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_company_tax_id_verification_files: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documents_proof_of_registration_files: std
    ::option
    ::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kana_town: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_kanji_town: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_address_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_dob_day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_dob_month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_dob_year: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_first_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_first_name_kana: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_first_name_kanji: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_full_name_aliases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_gender: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_id_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_id_number_secondary: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_last_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_last_name_kana: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_last_name_kanji: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_maiden_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_phone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_political_exposure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_registered_address_city: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_registered_address_country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_registered_address_line1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_registered_address_line2: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_registered_address_postal_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_registered_address_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_ssn_last_4: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_verification_additional_document_back: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_verification_additional_document_front: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_verification_document_back: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    individual_verification_document_front: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_branding_icon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_branding_logo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_branding_primary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_branding_secondary_color: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_issuing_tos_acceptance_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_issuing_tos_acceptance_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_issuing_tos_acceptance_user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_payments_decline_on_avs_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_payments_decline_on_cvc_failure: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_payments_statement_descriptor_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_payments_statement_descriptor_prefix_kana: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_card_payments_statement_descriptor_prefix_kanji: std
    ::option
    ::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payments_statement_descriptor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payments_statement_descriptor_kana: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payments_statement_descriptor_kanji: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payouts_debit_negative_balances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payouts_schedule_delay_days: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payouts_schedule_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payouts_schedule_monthly_anchor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payouts_schedule_weekly_anchor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_payouts_statement_descriptor: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_treasury_tos_acceptance_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_treasury_tos_acceptance_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    settings_treasury_tos_acceptance_user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tos_acceptance_date: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tos_acceptance_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tos_acceptance_service_agreement: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tos_acceptance_user_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct Account_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AccountData_>,
}

#[derive(Clone)]
pub struct Account(std::rc::Rc<Account_>);

impl Account {
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
        "Set the field `account_token`.\nAn [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account."]
    pub fn set_account_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bank_account`.\nEither a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary containing a user's bank account details."]
    pub fn set_bank_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bank_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_mcc`.\n[The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc). MCCs are used to classify businesses based on the goods or services they provide."]
    pub fn set_business_profile_mcc(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().business_profile_mcc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_profile_name`.\nThe customer-facing business name."]
    pub fn set_business_profile_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().business_profile_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_product_description`.\nInternal-only description of the product sold or service provided by the business. It's used by Stripe for risk and underwriting purposes."]
    pub fn set_business_profile_product_description(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_product_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_profile_support_address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_business_profile_support_address_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_address_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_support_address_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_business_profile_support_address_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_address_country = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_support_address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_business_profile_support_address_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_address_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_support_address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_business_profile_support_address_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_address_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_profile_support_address_postal_code`.\nZIP or postal code."]
    pub fn set_business_profile_support_address_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_address_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_profile_support_address_state`.\nState, county, province, or region."]
    pub fn set_business_profile_support_address_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_address_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_support_email`.\nA publicly available email address for sending support issues to."]
    pub fn set_business_profile_support_email(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `business_profile_support_phone`.\nA publicly available phone number to call with support issues."]
    pub fn set_business_profile_support_phone(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_phone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_profile_support_url`.\nA publicly available website for handling support issues."]
    pub fn set_business_profile_support_url(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().business_profile_support_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_profile_url`.\nThe business's publicly available website."]
    pub fn set_business_profile_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().business_profile_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `business_type`.\nThe business type."]
    pub fn set_business_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().business_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_acss_debit_payments_requested`.\n"]
    pub fn set_capabilities_acss_debit_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_acss_debit_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_affirm_payments_requested`.\n"]
    pub fn set_capabilities_affirm_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_affirm_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_afterpay_clearpay_payments_requested`.\n"]
    pub fn set_capabilities_afterpay_clearpay_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_afterpay_clearpay_payments_requested =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_au_becs_debit_payments_requested`.\n"]
    pub fn set_capabilities_au_becs_debit_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_au_becs_debit_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_bacs_debit_payments_requested`.\n"]
    pub fn set_capabilities_bacs_debit_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_bacs_debit_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_bancontact_payments_requested`.\n"]
    pub fn set_capabilities_bancontact_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_bancontact_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_bank_transfer_payments_requested`.\n"]
    pub fn set_capabilities_bank_transfer_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_bank_transfer_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_blik_payments_requested`.\n"]
    pub fn set_capabilities_blik_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_blik_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_boleto_payments_requested`.\n"]
    pub fn set_capabilities_boleto_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_boleto_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_card_issuing_requested`.\n"]
    pub fn set_capabilities_card_issuing_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_card_issuing_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_card_payments_requested`.\n"]
    pub fn set_capabilities_card_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_card_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_cartes_bancaires_payments_requested`.\n"]
    pub fn set_capabilities_cartes_bancaires_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_cartes_bancaires_payments_requested =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_eps_payments_requested`.\n"]
    pub fn set_capabilities_eps_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_eps_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_fpx_payments_requested`.\n"]
    pub fn set_capabilities_fpx_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_fpx_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_giropay_payments_requested`.\n"]
    pub fn set_capabilities_giropay_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_giropay_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_grabpay_payments_requested`.\n"]
    pub fn set_capabilities_grabpay_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_grabpay_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_ideal_payments_requested`.\n"]
    pub fn set_capabilities_ideal_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_ideal_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_jcb_payments_requested`.\n"]
    pub fn set_capabilities_jcb_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_jcb_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_klarna_payments_requested`.\n"]
    pub fn set_capabilities_klarna_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_klarna_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_konbini_payments_requested`.\n"]
    pub fn set_capabilities_konbini_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_konbini_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_legacy_payments_requested`.\n"]
    pub fn set_capabilities_legacy_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_legacy_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_link_payments_requested`.\n"]
    pub fn set_capabilities_link_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_link_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_oxxo_payments_requested`.\n"]
    pub fn set_capabilities_oxxo_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_oxxo_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_p24_payments_requested`.\n"]
    pub fn set_capabilities_p24_payments_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_p24_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_paynow_payments_requested`.\n"]
    pub fn set_capabilities_paynow_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_paynow_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_promptpay_payments_requested`.\n"]
    pub fn set_capabilities_promptpay_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_promptpay_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_sepa_debit_payments_requested`.\n"]
    pub fn set_capabilities_sepa_debit_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_sepa_debit_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_sofort_payments_requested`.\n"]
    pub fn set_capabilities_sofort_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_sofort_payments_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_tax_reporting_us_1099_k_requested`.\n"]
    pub fn set_capabilities_tax_reporting_us_1099_k_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_tax_reporting_us_1099_k_requested =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_tax_reporting_us_1099_misc_requested`.\n"]
    pub fn set_capabilities_tax_reporting_us_1099_misc_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_tax_reporting_us_1099_misc_requested =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_transfers_requested`.\n"]
    pub fn set_capabilities_transfers_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_transfers_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_treasury_requested`.\n"]
    pub fn set_capabilities_treasury_requested(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().capabilities_treasury_requested = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities_us_bank_account_ach_payments_requested`.\n"]
    pub fn set_capabilities_us_bank_account_ach_payments_requested(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().capabilities_us_bank_account_ach_payments_requested =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_company_address_city(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_address_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_address_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_company_address_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kana_city`.\nCity/Ward."]
    pub fn set_company_address_kana_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_address_kana_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_company_address_kana_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kana_line1`.\nBlock/Building number."]
    pub fn set_company_address_kana_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kana_line2`.\nBuilding details."]
    pub fn set_company_address_kana_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kana_postal_code`.\nZIP or postal code."]
    pub fn set_company_address_kana_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kana_state`.\nPrefecture."]
    pub fn set_company_address_kana_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kana_town`.\nTown/cho-me."]
    pub fn set_company_address_kana_town(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kana_town = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kanji_city`.\nCity/Ward."]
    pub fn set_company_address_kanji_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_address_kanji_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_company_address_kanji_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kanji_line1`.\nBlock/Building number."]
    pub fn set_company_address_kanji_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kanji_line2`.\nBuilding details."]
    pub fn set_company_address_kanji_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kanji_postal_code`.\nZIP or postal code."]
    pub fn set_company_address_kanji_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kanji_state`.\nPrefecture."]
    pub fn set_company_address_kanji_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_kanji_town`.\nTown/cho-me."]
    pub fn set_company_address_kanji_town(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_kanji_town = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_company_address_line1(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_address_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_company_address_line2(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_address_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_postal_code`.\nZIP or postal code."]
    pub fn set_company_address_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_address_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_address_state`.\nState, county, province, or region."]
    pub fn set_company_address_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_address_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_directors_provided`.\nWhether the company's directors have been provided. This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided)."]
    pub fn set_company_directors_provided(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().company_directors_provided = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_executives_provided`.\nWhether the company's executives have been provided. This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided."]
    pub fn set_company_executives_provided(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().company_executives_provided = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_name`.\nThe company's legal name."]
    pub fn set_company_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_name_kana`.\nThe Kana variation of the company's legal name (Japan only)."]
    pub fn set_company_name_kana(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_name_kana = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_name_kanji`.\nThe Kanji variation of the company's legal name (Japan only)."]
    pub fn set_company_name_kanji(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_name_kanji = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_owners_provided`.\nWhether the company's owners have been provided. This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided. Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together)."]
    pub fn set_company_owners_provided(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().company_owners_provided = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_ownership_declaration_date`.\nThe Unix timestamp marking when the beneficial owner attestation was made."]
    pub fn set_company_ownership_declaration_date(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().company_ownership_declaration_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_ownership_declaration_ip`.\nThe IP address from which the beneficial owner attestation was made."]
    pub fn set_company_ownership_declaration_ip(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_ownership_declaration_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_ownership_declaration_user_agent`.\nThe user-agent string from the browser where the beneficial owner attestation was made."]
    pub fn set_company_ownership_declaration_user_agent(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_ownership_declaration_user_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_phone`.\nThe company's phone number (used for verification)."]
    pub fn set_company_phone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_phone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_registration_number`.\n"]
    pub fn set_company_registration_number(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_registration_number = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_structure`.\nThe category identifying the legal structure of the company or legal entity. See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details."]
    pub fn set_company_structure(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_structure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_tax_id`.\n"]
    pub fn set_company_tax_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_tax_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_tax_id_registrar`.\nThe jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    pub fn set_company_tax_id_registrar(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_tax_id_registrar = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `company_vat_id`.\n"]
    pub fn set_company_vat_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().company_vat_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_verification_document_back`.\nThe back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`."]
    pub fn set_company_verification_document_back(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_verification_document_back = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `company_verification_document_front`.\nThe front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`."]
    pub fn set_company_verification_document_front(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().company_verification_document_front = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `country`.\nThe country in which the account holder resides, or in which the business is legally established. This should be an ISO 3166-1 alpha-2 country code. For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created. Available countries include [Stripe's global markets](https://stripe.com/global) as well as countries where [cross-border payouts](https://stripe.com/docs/connect/cross-border-payouts) are supported."]
    pub fn set_country(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().country = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_currency`.\nThree-letter ISO currency code representing the default currency for the account. This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts)."]
    pub fn set_default_currency(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_bank_account_ownership_verification_files`.\n"]
    pub fn set_documents_bank_account_ownership_verification_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_bank_account_ownership_verification_files =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_company_license_files`.\n"]
    pub fn set_documents_company_license_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_company_license_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_company_memorandum_of_association_files`.\n"]
    pub fn set_documents_company_memorandum_of_association_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_company_memorandum_of_association_files =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_company_ministerial_decree_files`.\n"]
    pub fn set_documents_company_ministerial_decree_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_company_ministerial_decree_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_company_registration_verification_files`.\n"]
    pub fn set_documents_company_registration_verification_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_company_registration_verification_files =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_company_tax_id_verification_files`.\n"]
    pub fn set_documents_company_tax_id_verification_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_company_tax_id_verification_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documents_proof_of_registration_files`.\n"]
    pub fn set_documents_proof_of_registration_files(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().documents_proof_of_registration_files = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `email`.\nThe email address of the account holder. This is only to make the account easier to identify to you. Stripe only emails Custom accounts with your consent."]
    pub fn set_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `external_account`.\nA card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups). You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation. <br><br>By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists. To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs."]
    pub fn set_external_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().external_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_individual_address_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_address_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_individual_address_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kana_city`.\nCity/Ward."]
    pub fn set_individual_address_kana_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_address_kana_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_individual_address_kana_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kana_line1`.\nBlock/Building number."]
    pub fn set_individual_address_kana_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kana_line2`.\nBuilding details."]
    pub fn set_individual_address_kana_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kana_postal_code`.\nZIP or postal code."]
    pub fn set_individual_address_kana_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kana_state`.\nPrefecture."]
    pub fn set_individual_address_kana_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kana_town`.\nTown/cho-me."]
    pub fn set_individual_address_kana_town(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kana_town = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kanji_city`.\nCity/Ward."]
    pub fn set_individual_address_kanji_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_address_kanji_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_individual_address_kanji_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kanji_line1`.\nBlock/Building number."]
    pub fn set_individual_address_kanji_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kanji_line2`.\nBuilding details."]
    pub fn set_individual_address_kanji_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kanji_postal_code`.\nZIP or postal code."]
    pub fn set_individual_address_kanji_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kanji_state`.\nPrefecture."]
    pub fn set_individual_address_kanji_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_kanji_town`.\nTown/cho-me."]
    pub fn set_individual_address_kanji_town(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_kanji_town = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_individual_address_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_individual_address_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_postal_code`.\nZIP or postal code."]
    pub fn set_individual_address_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_address_state`.\nState, county, province, or region."]
    pub fn set_individual_address_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_address_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_dob_day`.\nThe day of birth, between 1 and 31."]
    pub fn set_individual_dob_day(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().individual_dob_day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_dob_month`.\nThe month of birth, between 1 and 12."]
    pub fn set_individual_dob_month(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().individual_dob_month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_dob_year`.\nThe four-digit year of birth."]
    pub fn set_individual_dob_year(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().individual_dob_year = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_email`.\nThe person's email address."]
    pub fn set_individual_email(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_first_name`.\nThe person's first name."]
    pub fn set_individual_first_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_first_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_first_name_kana`.\nThe Kana variation of the person's first name (Japan only)."]
    pub fn set_individual_first_name_kana(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_first_name_kana = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_first_name_kanji`.\nThe Kanji variation of the person's first name (Japan only)."]
    pub fn set_individual_first_name_kanji(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_first_name_kanji = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_full_name_aliases`.\nA list of alternate names or aliases that the person is known by."]
    pub fn set_individual_full_name_aliases(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_full_name_aliases = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_gender`.\nThe person's gender (International regulations require either \"male\" or \"female\")."]
    pub fn set_individual_gender(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_gender = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_id_number`.\n"]
    pub fn set_individual_id_number(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_id_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_id_number_secondary`.\n"]
    pub fn set_individual_id_number_secondary(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_id_number_secondary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_last_name`.\nThe person's last name."]
    pub fn set_individual_last_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_last_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_last_name_kana`.\nThe Kana variation of the person's last name (Japan only)."]
    pub fn set_individual_last_name_kana(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_last_name_kana = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_last_name_kanji`.\nThe Kanji variation of the person's last name (Japan only)."]
    pub fn set_individual_last_name_kanji(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_last_name_kanji = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_maiden_name`.\nThe person's maiden name."]
    pub fn set_individual_maiden_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_maiden_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_metadata`.\nSet of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format."]
    pub fn set_individual_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_phone`.\nThe person's phone number."]
    pub fn set_individual_phone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_phone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_political_exposure`.\nIndicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction."]
    pub fn set_individual_political_exposure(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_political_exposure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_registered_address_city`.\nCity, district, suburb, town, or village."]
    pub fn set_individual_registered_address_city(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_registered_address_city = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_registered_address_country`.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn set_individual_registered_address_country(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_registered_address_country = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_registered_address_line1`.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn set_individual_registered_address_line1(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_registered_address_line1 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_registered_address_line2`.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn set_individual_registered_address_line2(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_registered_address_line2 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_registered_address_postal_code`.\nZIP or postal code."]
    pub fn set_individual_registered_address_postal_code(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_registered_address_postal_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_registered_address_state`.\nState, county, province, or region."]
    pub fn set_individual_registered_address_state(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_registered_address_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `individual_ssn_last_4`.\n"]
    pub fn set_individual_ssn_last_4(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().individual_ssn_last_4 = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_verification_additional_document_back`.\nThe back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn set_individual_verification_additional_document_back(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_verification_additional_document_back =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_verification_additional_document_front`.\nThe front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn set_individual_verification_additional_document_front(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_verification_additional_document_front =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_verification_document_back`.\nThe back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn set_individual_verification_document_back(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_verification_document_back = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `individual_verification_document_front`.\nThe front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn set_individual_verification_document_front(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().individual_verification_document_front = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_branding_icon`.\n(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account. Must be square and at least 128px x 128px."]
    pub fn set_settings_branding_icon(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_branding_icon = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_branding_logo`.\n(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided. Must be at least 128px x 128px."]
    pub fn set_settings_branding_logo(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_branding_logo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_branding_primary_color`.\nA CSS hex color value representing the primary branding color for this account"]
    pub fn set_settings_branding_primary_color(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_branding_primary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_branding_secondary_color`.\nA CSS hex color value representing the secondary branding color for this account"]
    pub fn set_settings_branding_secondary_color(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_branding_secondary_color = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_issuing_tos_acceptance_date`.\nThe Unix timestamp marking when the account representative accepted the service agreement."]
    pub fn set_settings_card_issuing_tos_acceptance_date(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_issuing_tos_acceptance_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_issuing_tos_acceptance_ip`.\nThe IP address from which the account representative accepted the service agreement."]
    pub fn set_settings_card_issuing_tos_acceptance_ip(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_issuing_tos_acceptance_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_issuing_tos_acceptance_user_agent`.\nThe user agent of the browser from which the account representative accepted the service agreement."]
    pub fn set_settings_card_issuing_tos_acceptance_user_agent(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_issuing_tos_acceptance_user_agent =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_payments_decline_on_avs_failure`.\nWhether Stripe automatically declines charges with an incorrect ZIP or postal code. This setting only applies when a ZIP or postal code is provided and they fail bank verification."]
    pub fn set_settings_card_payments_decline_on_avs_failure(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_payments_decline_on_avs_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_payments_decline_on_cvc_failure`.\nWhether Stripe automatically declines charges with an incorrect CVC. This setting only applies when a CVC is provided and it fails bank verification."]
    pub fn set_settings_card_payments_decline_on_cvc_failure(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_payments_decline_on_cvc_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_payments_statement_descriptor_prefix`.\nThe default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge. `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn set_settings_card_payments_statement_descriptor_prefix(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_payments_statement_descriptor_prefix =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_payments_statement_descriptor_prefix_kana`.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn set_settings_card_payments_statement_descriptor_prefix_kana(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_payments_statement_descriptor_prefix_kana =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_card_payments_statement_descriptor_prefix_kanji`.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn set_settings_card_payments_statement_descriptor_prefix_kanji(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_card_payments_statement_descriptor_prefix_kanji =
            std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payments_statement_descriptor`.\nThe default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge."]
    pub fn set_settings_payments_statement_descriptor(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payments_statement_descriptor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payments_statement_descriptor_kana`.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)"]
    pub fn set_settings_payments_statement_descriptor_kana(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payments_statement_descriptor_kana = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payments_statement_descriptor_kanji`.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)"]
    pub fn set_settings_payments_statement_descriptor_kanji(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payments_statement_descriptor_kanji = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payouts_debit_negative_balances`.\nA Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account. See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details. Default value is `false` for Custom accounts, otherwise `true`."]
    pub fn set_settings_payouts_debit_negative_balances(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payouts_debit_negative_balances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payouts_schedule_delay_days`.\nThe number of days charges for the account will be held before being paid out."]
    pub fn set_settings_payouts_schedule_delay_days(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payouts_schedule_delay_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payouts_schedule_interval`.\nHow frequently funds will be paid out. One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`."]
    pub fn set_settings_payouts_schedule_interval(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payouts_schedule_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payouts_schedule_monthly_anchor`.\nThe day of the month funds will be paid out. Only shown if `interval` is monthly. Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months."]
    pub fn set_settings_payouts_schedule_monthly_anchor(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payouts_schedule_monthly_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payouts_schedule_weekly_anchor`.\nThe day of the week funds will be paid out, of the style 'monday', 'tuesday', etc. Only shown if `interval` is weekly."]
    pub fn set_settings_payouts_schedule_weekly_anchor(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payouts_schedule_weekly_anchor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_payouts_statement_descriptor`.\nThe text that appears on the bank account statement for payouts. If not set, this defaults to the platform's bank descriptor as set in the Dashboard."]
    pub fn set_settings_payouts_statement_descriptor(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_payouts_statement_descriptor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_treasury_tos_acceptance_date`.\nThe Unix timestamp marking when the account representative accepted the service agreement."]
    pub fn set_settings_treasury_tos_acceptance_date(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().settings_treasury_tos_acceptance_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_treasury_tos_acceptance_ip`.\nThe IP address from which the account representative accepted the service agreement."]
    pub fn set_settings_treasury_tos_acceptance_ip(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_treasury_tos_acceptance_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `settings_treasury_tos_acceptance_user_agent`.\nThe user agent of the browser from which the account representative accepted the service agreement."]
    pub fn set_settings_treasury_tos_acceptance_user_agent(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().settings_treasury_tos_acceptance_user_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tos_acceptance_date`.\nThe Unix timestamp marking when the account representative accepted their service agreement"]
    pub fn set_tos_acceptance_date(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tos_acceptance_date = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tos_acceptance_ip`.\nThe IP address from which the account representative accepted their service agreement"]
    pub fn set_tos_acceptance_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tos_acceptance_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tos_acceptance_service_agreement`.\nThe user's service agreement type"]
    pub fn set_tos_acceptance_service_agreement(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tos_acceptance_service_agreement = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tos_acceptance_user_agent`.\nThe user agent of the browser from which the account representative accepted their service agreement"]
    pub fn set_tos_acceptance_user_agent(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().tos_acceptance_user_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of Stripe account to create. May be one of `custom`, `express` or `standard`."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_token` after provisioning.\nAn [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account."]
    pub fn account_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bank_account` after provisioning.\nEither a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary containing a user's bank account details."]
    pub fn bank_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bank_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_mcc` after provisioning.\n[The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc). MCCs are used to classify businesses based on the goods or services they provide."]
    pub fn business_profile_mcc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_mcc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_name` after provisioning.\nThe customer-facing business name."]
    pub fn business_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_product_description` after provisioning.\nInternal-only description of the product sold or service provided by the business. It's used by Stripe for risk and underwriting purposes."]
    pub fn business_profile_product_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_product_description", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn business_profile_support_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn business_profile_support_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn business_profile_support_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn business_profile_support_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn business_profile_support_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_state` after provisioning.\nState, county, province, or region."]
    pub fn business_profile_support_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_email` after provisioning.\nA publicly available email address for sending support issues to."]
    pub fn business_profile_support_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_phone` after provisioning.\nA publicly available phone number to call with support issues."]
    pub fn business_profile_support_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_phone", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_url` after provisioning.\nA publicly available website for handling support issues."]
    pub fn business_profile_support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_support_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_url` after provisioning.\nThe business's publicly available website."]
    pub fn business_profile_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_type` after provisioning.\nThe business type."]
    pub fn business_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capabilities_acss_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_acss_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_acss_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_affirm_payments_requested` after provisioning.\n"]
    pub fn capabilities_affirm_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_affirm_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_afterpay_clearpay_payments_requested` after provisioning.\n"]
    pub fn capabilities_afterpay_clearpay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_afterpay_clearpay_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_au_becs_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_au_becs_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_au_becs_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_bacs_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_bacs_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_bacs_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_bancontact_payments_requested` after provisioning.\n"]
    pub fn capabilities_bancontact_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_bancontact_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_bank_transfer_payments_requested` after provisioning.\n"]
    pub fn capabilities_bank_transfer_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_bank_transfer_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_blik_payments_requested` after provisioning.\n"]
    pub fn capabilities_blik_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_blik_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_boleto_payments_requested` after provisioning.\n"]
    pub fn capabilities_boleto_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_boleto_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_card_issuing_requested` after provisioning.\n"]
    pub fn capabilities_card_issuing_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_card_issuing_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_card_payments_requested` after provisioning.\n"]
    pub fn capabilities_card_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_card_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_cartes_bancaires_payments_requested` after provisioning.\n"]
    pub fn capabilities_cartes_bancaires_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_cartes_bancaires_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_eps_payments_requested` after provisioning.\n"]
    pub fn capabilities_eps_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_eps_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_fpx_payments_requested` after provisioning.\n"]
    pub fn capabilities_fpx_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_fpx_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_giropay_payments_requested` after provisioning.\n"]
    pub fn capabilities_giropay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_giropay_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_grabpay_payments_requested` after provisioning.\n"]
    pub fn capabilities_grabpay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_grabpay_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_ideal_payments_requested` after provisioning.\n"]
    pub fn capabilities_ideal_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_ideal_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_jcb_payments_requested` after provisioning.\n"]
    pub fn capabilities_jcb_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_jcb_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_klarna_payments_requested` after provisioning.\n"]
    pub fn capabilities_klarna_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_klarna_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_konbini_payments_requested` after provisioning.\n"]
    pub fn capabilities_konbini_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_konbini_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_legacy_payments_requested` after provisioning.\n"]
    pub fn capabilities_legacy_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_legacy_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_link_payments_requested` after provisioning.\n"]
    pub fn capabilities_link_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_link_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_oxxo_payments_requested` after provisioning.\n"]
    pub fn capabilities_oxxo_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_oxxo_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_p24_payments_requested` after provisioning.\n"]
    pub fn capabilities_p24_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_p24_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_paynow_payments_requested` after provisioning.\n"]
    pub fn capabilities_paynow_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_paynow_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_promptpay_payments_requested` after provisioning.\n"]
    pub fn capabilities_promptpay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_promptpay_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_sepa_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_sepa_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_sepa_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_sofort_payments_requested` after provisioning.\n"]
    pub fn capabilities_sofort_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_sofort_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_tax_reporting_us_1099_k_requested` after provisioning.\n"]
    pub fn capabilities_tax_reporting_us_1099_k_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_tax_reporting_us_1099_k_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_tax_reporting_us_1099_misc_requested` after provisioning.\n"]
    pub fn capabilities_tax_reporting_us_1099_misc_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_tax_reporting_us_1099_misc_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_transfers_requested` after provisioning.\n"]
    pub fn capabilities_transfers_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_transfers_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_treasury_requested` after provisioning.\n"]
    pub fn capabilities_treasury_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_treasury_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_us_bank_account_ach_payments_requested` after provisioning.\n"]
    pub fn capabilities_us_bank_account_ach_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_us_bank_account_ach_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `charges_enabled` after provisioning.\nWhether the account can create live charges."]
    pub fn charges_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.charges_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn company_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn company_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kana_city` after provisioning.\nCity/Ward."]
    pub fn company_address_kana_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kana_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn company_address_kana_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kana_line1` after provisioning.\nBlock/Building number."]
    pub fn company_address_kana_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_line1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kana_line2` after provisioning.\nBuilding details."]
    pub fn company_address_kana_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kana_postal_code` after provisioning.\nZIP or postal code."]
    pub fn company_address_kana_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_address_kana_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `company_address_kana_state` after provisioning.\nPrefecture."]
    pub fn company_address_kana_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kana_town` after provisioning.\nTown/cho-me."]
    pub fn company_address_kana_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_town", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kanji_city` after provisioning.\nCity/Ward."]
    pub fn company_address_kanji_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn company_address_kanji_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_address_kanji_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_line1` after provisioning.\nBlock/Building number."]
    pub fn company_address_kanji_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_line2` after provisioning.\nBuilding details."]
    pub fn company_address_kanji_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_postal_code` after provisioning.\nZIP or postal code."]
    pub fn company_address_kanji_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_address_kanji_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `company_address_kanji_state` after provisioning.\nPrefecture."]
    pub fn company_address_kanji_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kanji_town` after provisioning.\nTown/cho-me."]
    pub fn company_address_kanji_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_town", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn company_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn company_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn company_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_state` after provisioning.\nState, county, province, or region."]
    pub fn company_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_directors_provided` after provisioning.\nWhether the company's directors have been provided. This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided)."]
    pub fn company_directors_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_directors_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_executives_provided` after provisioning.\nWhether the company's executives have been provided. This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided."]
    pub fn company_executives_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_executives_provided", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_name` after provisioning.\nThe company's legal name."]
    pub fn company_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_name_kana` after provisioning.\nThe Kana variation of the company's legal name (Japan only)."]
    pub fn company_name_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name_kana", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_name_kanji` after provisioning.\nThe Kanji variation of the company's legal name (Japan only)."]
    pub fn company_name_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name_kanji", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_owners_provided` after provisioning.\nWhether the company's owners have been provided. This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided. Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together)."]
    pub fn company_owners_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_owners_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_ownership_declaration_date` after provisioning.\nThe Unix timestamp marking when the beneficial owner attestation was made."]
    pub fn company_ownership_declaration_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_ownership_declaration_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_ownership_declaration_ip` after provisioning.\nThe IP address from which the beneficial owner attestation was made."]
    pub fn company_ownership_declaration_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_ownership_declaration_ip", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_ownership_declaration_user_agent` after provisioning.\nThe user-agent string from the browser where the beneficial owner attestation was made."]
    pub fn company_ownership_declaration_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_ownership_declaration_user_agent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_phone` after provisioning.\nThe company's phone number (used for verification)."]
    pub fn company_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_registration_number` after provisioning.\n"]
    pub fn company_registration_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_registration_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_structure` after provisioning.\nThe category identifying the legal structure of the company or legal entity. See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details."]
    pub fn company_structure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_structure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_tax_id` after provisioning.\n"]
    pub fn company_tax_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_tax_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_tax_id_provided` after provisioning.\nWhether the company's business ID number was provided."]
    pub fn company_tax_id_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_tax_id_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_tax_id_registrar` after provisioning.\nThe jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    pub fn company_tax_id_registrar(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_tax_id_registrar", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_vat_id` after provisioning.\n"]
    pub fn company_vat_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_vat_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_vat_id_provided` after provisioning.\nWhether the company's business VAT number was provided."]
    pub fn company_vat_id_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_vat_id_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_back` after provisioning.\nThe back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`."]
    pub fn company_verification_document_back(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_back", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_details` after provisioning.\nA user-displayable string describing the verification state of this document."]
    pub fn company_verification_document_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_details_code` after provisioning.\nOne of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`. A machine-readable code specifying the verification state for this document."]
    pub fn company_verification_document_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_front` after provisioning.\nThe front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`."]
    pub fn company_verification_document_front(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_front", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `controller_is_controller` after provisioning.\n`true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts). Otherwise, this field is null."]
    pub fn controller_is_controller(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.controller_is_controller", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `controller_type` after provisioning.\nThe controller type. Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself."]
    pub fn controller_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.controller_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `country` after provisioning.\nThe country in which the account holder resides, or in which the business is legally established. This should be an ISO 3166-1 alpha-2 country code. For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created. Available countries include [Stripe's global markets](https://stripe.com/global) as well as countries where [cross-border payouts](https://stripe.com/docs/connect/cross-border-payouts) are supported."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the account was connected. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_currency` after provisioning.\nThree-letter ISO currency code representing the default currency for the account. This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts)."]
    pub fn default_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `details_submitted` after provisioning.\nWhether account details have been submitted. Standard accounts cannot receive payouts before this is true."]
    pub fn details_submitted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details_submitted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `documents_bank_account_ownership_verification_files` after provisioning.\n"]
    pub fn documents_bank_account_ownership_verification_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_bank_account_ownership_verification_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_company_license_files` after provisioning.\n"]
    pub fn documents_company_license_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_license_files", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `documents_company_memorandum_of_association_files` after provisioning.\n"]
    pub fn documents_company_memorandum_of_association_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_memorandum_of_association_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_company_ministerial_decree_files` after provisioning.\n"]
    pub fn documents_company_ministerial_decree_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_ministerial_decree_files", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `documents_company_registration_verification_files` after provisioning.\n"]
    pub fn documents_company_registration_verification_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_registration_verification_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_company_tax_id_verification_files` after provisioning.\n"]
    pub fn documents_company_tax_id_verification_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_tax_id_verification_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_proof_of_registration_files` after provisioning.\n"]
    pub fn documents_proof_of_registration_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_proof_of_registration_files", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email address of the account holder. This is only to make the account easier to identify to you. Stripe only emails Custom accounts with your consent."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_account` after provisioning.\nA card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups). You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation. <br><br>By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists. To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs."]
    pub fn external_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_accounts_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn external_accounts_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_accounts_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_accounts_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn external_accounts_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_accounts_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_accounts_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn external_accounts_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_accounts_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn future_requirements_alternatives(&self) -> terrars::ListRef<Account_FutureRequirementsAlternativesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_alternatives", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_current_deadline` after provisioning.\nDate on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty. After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning."]
    pub fn future_requirements_current_deadline(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.future_requirements_current_deadline", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_currently_due` after provisioning.\nFields that need to be collected to keep the account enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash."]
    pub fn future_requirements_currently_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_currently_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_disabled_reason` after provisioning.\nThis is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account."]
    pub fn future_requirements_disabled_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.future_requirements_disabled_reason", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn future_requirements_errors(&self) -> terrars::ListRef<Account_FutureRequirementsErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.future_requirements_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well."]
    pub fn future_requirements_eventually_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_eventually_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_past_due` after provisioning.\nFields that weren't collected by `requirements.current_deadline`. These fields need to be collected to enable the capability on the account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`."]
    pub fn future_requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.future_requirements_past_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`."]
    pub fn future_requirements_pending_verification(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_account` after provisioning.\nThe account the person is associated with."]
    pub fn individual_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn individual_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_address_kana_city` after provisioning.\nCity/Ward."]
    pub fn individual_address_kana_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_kana_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_address_kana_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_line1` after provisioning.\nBlock/Building number."]
    pub fn individual_address_kana_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_line2` after provisioning.\nBuilding details."]
    pub fn individual_address_kana_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_address_kana_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kana_state` after provisioning.\nPrefecture."]
    pub fn individual_address_kana_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kana_town` after provisioning.\nTown/cho-me."]
    pub fn individual_address_kana_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_kana_town", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_address_kanji_city` after provisioning.\nCity/Ward."]
    pub fn individual_address_kanji_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_address_kanji_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_line1` after provisioning.\nBlock/Building number."]
    pub fn individual_address_kanji_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_line2` after provisioning.\nBuilding details."]
    pub fn individual_address_kanji_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_address_kanji_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kanji_state` after provisioning.\nPrefecture."]
    pub fn individual_address_kanji_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kanji_town` after provisioning.\nTown/cho-me."]
    pub fn individual_address_kanji_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_town", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn individual_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn individual_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_state` after provisioning.\nState, county, province, or region."]
    pub fn individual_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn individual_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_dob_day` after provisioning.\nThe day of birth, between 1 and 31."]
    pub fn individual_dob_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_dob_day", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_dob_month` after provisioning.\nThe month of birth, between 1 and 12."]
    pub fn individual_dob_month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_dob_month", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_dob_year` after provisioning.\nThe four-digit year of birth."]
    pub fn individual_dob_year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_dob_year", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_email` after provisioning.\nThe person's email address."]
    pub fn individual_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_first_name` after provisioning.\nThe person's first name."]
    pub fn individual_first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_first_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_first_name_kana` after provisioning.\nThe Kana variation of the person's first name (Japan only)."]
    pub fn individual_first_name_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_first_name_kana", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_first_name_kanji` after provisioning.\nThe Kanji variation of the person's first name (Japan only)."]
    pub fn individual_first_name_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_first_name_kanji", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_full_name_aliases` after provisioning.\nA list of alternate names or aliases that the person is known by."]
    pub fn individual_full_name_aliases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.individual_full_name_aliases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn individual_future_requirements_alternatives(
        &self,
    ) -> terrars::ListRef<Account_IndividualFutureRequirementsAlternativesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_alternatives", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_currently_due` after provisioning.\nFields that need to be collected to keep the person's account enabled. If not collected by the account's `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash, and may immediately become `past_due`, but the account may also be given a grace period depending on the account's enablement state prior to transition."]
    pub fn individual_future_requirements_currently_due(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_currently_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn individual_future_requirements_errors(
        &self,
    ) -> terrars::ListRef<Account_IndividualFutureRequirementsErrorsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_errors", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and the account's `future_requirements[current_deadline]` becomes set."]
    pub fn individual_future_requirements_eventually_due(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_eventually_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_past_due` after provisioning.\nFields that weren't collected by the account's `requirements.current_deadline`. These fields need to be collected to enable the person's account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`."]
    pub fn individual_future_requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_past_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`."]
    pub fn individual_future_requirements_pending_verification(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_gender` after provisioning.\nThe person's gender (International regulations require either \"male\" or \"female\")."]
    pub fn individual_gender(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_gender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_id` after provisioning.\nUnique identifier for the object."]
    pub fn individual_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_id_number` after provisioning.\n"]
    pub fn individual_id_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_id_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_id_number_provided` after provisioning.\nWhether the person's `id_number` was provided."]
    pub fn individual_id_number_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_id_number_provided", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_id_number_secondary` after provisioning.\n"]
    pub fn individual_id_number_secondary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_id_number_secondary", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_id_number_secondary_provided` after provisioning.\nWhether the person's `id_number_secondary` was provided."]
    pub fn individual_id_number_secondary_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_id_number_secondary_provided", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_last_name` after provisioning.\nThe person's last name."]
    pub fn individual_last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_last_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_last_name_kana` after provisioning.\nThe Kana variation of the person's last name (Japan only)."]
    pub fn individual_last_name_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_last_name_kana", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_last_name_kanji` after provisioning.\nThe Kanji variation of the person's last name (Japan only)."]
    pub fn individual_last_name_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_last_name_kanji", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_maiden_name` after provisioning.\nThe person's maiden name."]
    pub fn individual_maiden_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_maiden_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_metadata` after provisioning.\nSet of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format."]
    pub fn individual_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.individual_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_nationality` after provisioning.\nThe country where the person is a national."]
    pub fn individual_nationality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_nationality", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn individual_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_phone` after provisioning.\nThe person's phone number."]
    pub fn individual_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_political_exposure` after provisioning.\nIndicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction."]
    pub fn individual_political_exposure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_political_exposure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn individual_registered_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_registered_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn individual_registered_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn individual_registered_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_registered_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_state` after provisioning.\nState, county, province, or region."]
    pub fn individual_registered_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_director` after provisioning.\nWhether the person is a director of the account's legal entity. Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations."]
    pub fn individual_relationship_director(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_director", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_executive` after provisioning.\nWhether the person has significant responsibility to control, manage, or direct the organization."]
    pub fn individual_relationship_executive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_executive", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_owner` after provisioning.\nWhether the person is an owner of the account’s legal entity."]
    pub fn individual_relationship_owner(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_owner", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_percent_ownership` after provisioning.\nThe percent owned by the person of the account's legal entity."]
    pub fn individual_relationship_percent_ownership(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_percent_ownership", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_representative` after provisioning.\nWhether the person is authorized as the primary representative of the account. This is the person nominated by the business to provide information about themselves, and general information about the account. There can only be one representative at any given time. At the time the account is created, this person should be set to the person responsible for opening the account."]
    pub fn individual_relationship_representative(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_representative", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_title` after provisioning.\nThe person's title (e.g., CEO, Support Engineer)."]
    pub fn individual_relationship_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_title", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn individual_requirements_alternatives(
        &self,
    ) -> terrars::ListRef<Account_IndividualRequirementsAlternativesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_alternatives", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_currently_due` after provisioning.\nFields that need to be collected to keep the person's account enabled. If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled."]
    pub fn individual_requirements_currently_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_currently_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn individual_requirements_errors(&self) -> terrars::ListRef<Account_IndividualRequirementsErrorsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_errors", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set."]
    pub fn individual_requirements_eventually_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_eventually_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_past_due` after provisioning.\nFields that weren't collected by the account's `current_deadline`. These fields need to be collected to enable the person's account."]
    pub fn individual_requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_past_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`."]
    pub fn individual_requirements_pending_verification(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_ssn_last_4` after provisioning.\n"]
    pub fn individual_ssn_last_4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_ssn_last_4", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_ssn_last_4_provided` after provisioning.\nWhether the last four digits of the person's Social Security number have been provided (U.S. only)."]
    pub fn individual_ssn_last_4_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_ssn_last_4_provided", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_back` after provisioning.\nThe back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_additional_document_back(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_back", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_details` after provisioning.\nA user-displayable string describing the verification state of this document. For example, if a document is uploaded and the picture is too fuzzy, this may say \"Identity document is too unclear to read\"."]
    pub fn individual_verification_additional_document_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_details_code` after provisioning.\nOne of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`. A machine-readable code specifying the verification state for this document."]
    pub fn individual_verification_additional_document_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_front` after provisioning.\nThe front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_additional_document_front(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_front", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_details` after provisioning.\nA user-displayable string describing the verification state for the person. For example, this may say \"Provided identity information could not be verified\"."]
    pub fn individual_verification_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_details_code` after provisioning.\nOne of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`. A machine-readable code specifying the verification state for the person."]
    pub fn individual_verification_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_back` after provisioning.\nThe back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_document_back(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_back", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_details` after provisioning.\nA user-displayable string describing the verification state of this document. For example, if a document is uploaded and the picture is too fuzzy, this may say \"Identity document is too unclear to read\"."]
    pub fn individual_verification_document_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_details_code` after provisioning.\nOne of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`. A machine-readable code specifying the verification state for this document."]
    pub fn individual_verification_document_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_front` after provisioning.\nThe front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_document_front(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_front", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_status` after provisioning.\nThe state of verification for the person. Possible values are `unverified`, `pending`, or `verified`."]
    pub fn individual_verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_status", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payouts_enabled` after provisioning.\nWhether Stripe can send payouts to this account."]
    pub fn payouts_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payouts_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn requirements_alternatives(&self) -> terrars::ListRef<Account_RequirementsAlternativesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_alternatives", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_current_deadline` after provisioning.\nDate by which the fields in `currently_due` must be collected to keep the account enabled. These fields may disable the account sooner if the next threshold is reached before they are collected."]
    pub fn requirements_current_deadline(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.requirements_current_deadline", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `requirements_currently_due` after provisioning.\nFields that need to be collected to keep the account enabled. If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled."]
    pub fn requirements_currently_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_currently_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_disabled_reason` after provisioning.\nIf the account is disabled, this string describes why. Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`."]
    pub fn requirements_disabled_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirements_disabled_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn requirements_errors(&self) -> terrars::ListRef<Account_RequirementsErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set."]
    pub fn requirements_eventually_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_eventually_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_past_due` after provisioning.\nFields that weren't collected by `current_deadline`. These fields need to be collected to enable the account."]
    pub fn requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_past_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`."]
    pub fn requirements_pending_verification(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_bacs_debit_payments_display_name` after provisioning.\nThe Bacs Direct Debit Display Name for this account. For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor."]
    pub fn settings_bacs_debit_payments_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_bacs_debit_payments_display_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_icon` after provisioning.\n(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account. Must be square and at least 128px x 128px."]
    pub fn settings_branding_icon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_branding_icon", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_logo` after provisioning.\n(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided. Must be at least 128px x 128px."]
    pub fn settings_branding_logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_branding_logo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_primary_color` after provisioning.\nA CSS hex color value representing the primary branding color for this account"]
    pub fn settings_branding_primary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_branding_primary_color", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_secondary_color` after provisioning.\nA CSS hex color value representing the secondary branding color for this account"]
    pub fn settings_branding_secondary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_branding_secondary_color", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_issuing_tos_acceptance_date` after provisioning.\nThe Unix timestamp marking when the account representative accepted the service agreement."]
    pub fn settings_card_issuing_tos_acceptance_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_issuing_tos_acceptance_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_issuing_tos_acceptance_ip` after provisioning.\nThe IP address from which the account representative accepted the service agreement."]
    pub fn settings_card_issuing_tos_acceptance_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_issuing_tos_acceptance_ip", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_issuing_tos_acceptance_user_agent` after provisioning.\nThe user agent of the browser from which the account representative accepted the service agreement."]
    pub fn settings_card_issuing_tos_acceptance_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_issuing_tos_acceptance_user_agent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_decline_on_avs_failure` after provisioning.\nWhether Stripe automatically declines charges with an incorrect ZIP or postal code. This setting only applies when a ZIP or postal code is provided and they fail bank verification."]
    pub fn settings_card_payments_decline_on_avs_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_decline_on_avs_failure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_decline_on_cvc_failure` after provisioning.\nWhether Stripe automatically declines charges with an incorrect CVC. This setting only applies when a CVC is provided and it fails bank verification."]
    pub fn settings_card_payments_decline_on_cvc_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_decline_on_cvc_failure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_statement_descriptor_prefix` after provisioning.\nThe default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge. `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_card_payments_statement_descriptor_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_statement_descriptor_prefix", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_statement_descriptor_prefix_kana` after provisioning.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_card_payments_statement_descriptor_prefix_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_statement_descriptor_prefix_kana", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_statement_descriptor_prefix_kanji` after provisioning.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_card_payments_statement_descriptor_prefix_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_statement_descriptor_prefix_kanji", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_dashboard_display_name` after provisioning.\nThe display name for this account. This is used on the Stripe Dashboard to differentiate between accounts."]
    pub fn settings_dashboard_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_dashboard_display_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_dashboard_timezone` after provisioning.\nThe timezone used in the Stripe Dashboard for this account. A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones)."]
    pub fn settings_dashboard_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_dashboard_timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor` after provisioning.\nThe default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge."]
    pub fn settings_payments_statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_kana` after provisioning.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)"]
    pub fn settings_payments_statement_descriptor_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_kana", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_kanji` after provisioning.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)"]
    pub fn settings_payments_statement_descriptor_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_kanji", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_prefix_kana` after provisioning.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_payments_statement_descriptor_prefix_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_prefix_kana", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_prefix_kanji` after provisioning.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_payments_statement_descriptor_prefix_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_prefix_kanji", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_debit_negative_balances` after provisioning.\nA Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account. See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details. Default value is `false` for Custom accounts, otherwise `true`."]
    pub fn settings_payouts_debit_negative_balances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_debit_negative_balances", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_delay_days` after provisioning.\nThe number of days charges for the account will be held before being paid out."]
    pub fn settings_payouts_schedule_delay_days(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_delay_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_interval` after provisioning.\nHow frequently funds will be paid out. One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`."]
    pub fn settings_payouts_schedule_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_monthly_anchor` after provisioning.\nThe day of the month funds will be paid out. Only shown if `interval` is monthly. Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months."]
    pub fn settings_payouts_schedule_monthly_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_monthly_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_weekly_anchor` after provisioning.\nThe day of the week funds will be paid out, of the style 'monday', 'tuesday', etc. Only shown if `interval` is weekly."]
    pub fn settings_payouts_schedule_weekly_anchor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_weekly_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_statement_descriptor` after provisioning.\nThe text that appears on the bank account statement for payouts. If not set, this defaults to the platform's bank descriptor as set in the Dashboard."]
    pub fn settings_payouts_statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_statement_descriptor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_sepa_debit_payments_creditor_id` after provisioning.\nSEPA creditor identifier that identifies the company making the payment."]
    pub fn settings_sepa_debit_payments_creditor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_sepa_debit_payments_creditor_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_treasury_tos_acceptance_date` after provisioning.\nThe Unix timestamp marking when the account representative accepted the service agreement."]
    pub fn settings_treasury_tos_acceptance_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_treasury_tos_acceptance_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_treasury_tos_acceptance_ip` after provisioning.\nThe IP address from which the account representative accepted the service agreement."]
    pub fn settings_treasury_tos_acceptance_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_treasury_tos_acceptance_ip", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_treasury_tos_acceptance_user_agent` after provisioning.\nThe user agent of the browser from which the account representative accepted the service agreement."]
    pub fn settings_treasury_tos_acceptance_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_treasury_tos_acceptance_user_agent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_date` after provisioning.\nThe Unix timestamp marking when the account representative accepted their service agreement"]
    pub fn tos_acceptance_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_acceptance_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_ip` after provisioning.\nThe IP address from which the account representative accepted their service agreement"]
    pub fn tos_acceptance_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_acceptance_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_service_agreement` after provisioning.\nThe user's service agreement type"]
    pub fn tos_acceptance_service_agreement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tos_acceptance_service_agreement", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_user_agent` after provisioning.\nThe user agent of the browser from which the account representative accepted their service agreement"]
    pub fn tos_acceptance_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_acceptance_user_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of Stripe account to create. May be one of `custom`, `express` or `standard`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for Account {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Account { }

impl terrars::ToListMappable for Account {
    type O = terrars::ListRef<AccountRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Account_ {
    fn extract_resource_type(&self) -> std::string::String {
        "stripe_account".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccount {
    pub tf_id: std::string::String,
}

impl BuildAccount {
    pub fn build(self, stack: &mut terrars::Stack) -> Account {
        let out = Account(std::rc::Rc::new(Account_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AccountData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_token: core::default::Default::default(),
                bank_account: core::default::Default::default(),
                business_profile_mcc: core::default::Default::default(),
                business_profile_name: core::default::Default::default(),
                business_profile_product_description: core::default::Default::default(),
                business_profile_support_address_city: core::default::Default::default(),
                business_profile_support_address_country: core::default::Default::default(),
                business_profile_support_address_line1: core::default::Default::default(),
                business_profile_support_address_line2: core::default::Default::default(),
                business_profile_support_address_postal_code: core::default::Default::default(),
                business_profile_support_address_state: core::default::Default::default(),
                business_profile_support_email: core::default::Default::default(),
                business_profile_support_phone: core::default::Default::default(),
                business_profile_support_url: core::default::Default::default(),
                business_profile_url: core::default::Default::default(),
                business_type: core::default::Default::default(),
                capabilities_acss_debit_payments_requested: core::default::Default::default(),
                capabilities_affirm_payments_requested: core::default::Default::default(),
                capabilities_afterpay_clearpay_payments_requested: core::default::Default::default(),
                capabilities_au_becs_debit_payments_requested: core::default::Default::default(),
                capabilities_bacs_debit_payments_requested: core::default::Default::default(),
                capabilities_bancontact_payments_requested: core::default::Default::default(),
                capabilities_bank_transfer_payments_requested: core::default::Default::default(),
                capabilities_blik_payments_requested: core::default::Default::default(),
                capabilities_boleto_payments_requested: core::default::Default::default(),
                capabilities_card_issuing_requested: core::default::Default::default(),
                capabilities_card_payments_requested: core::default::Default::default(),
                capabilities_cartes_bancaires_payments_requested: core::default::Default::default(),
                capabilities_eps_payments_requested: core::default::Default::default(),
                capabilities_fpx_payments_requested: core::default::Default::default(),
                capabilities_giropay_payments_requested: core::default::Default::default(),
                capabilities_grabpay_payments_requested: core::default::Default::default(),
                capabilities_ideal_payments_requested: core::default::Default::default(),
                capabilities_jcb_payments_requested: core::default::Default::default(),
                capabilities_klarna_payments_requested: core::default::Default::default(),
                capabilities_konbini_payments_requested: core::default::Default::default(),
                capabilities_legacy_payments_requested: core::default::Default::default(),
                capabilities_link_payments_requested: core::default::Default::default(),
                capabilities_oxxo_payments_requested: core::default::Default::default(),
                capabilities_p24_payments_requested: core::default::Default::default(),
                capabilities_paynow_payments_requested: core::default::Default::default(),
                capabilities_promptpay_payments_requested: core::default::Default::default(),
                capabilities_sepa_debit_payments_requested: core::default::Default::default(),
                capabilities_sofort_payments_requested: core::default::Default::default(),
                capabilities_tax_reporting_us_1099_k_requested: core::default::Default::default(),
                capabilities_tax_reporting_us_1099_misc_requested: core::default::Default::default(),
                capabilities_transfers_requested: core::default::Default::default(),
                capabilities_treasury_requested: core::default::Default::default(),
                capabilities_us_bank_account_ach_payments_requested: core::default::Default::default(),
                company_address_city: core::default::Default::default(),
                company_address_country: core::default::Default::default(),
                company_address_kana_city: core::default::Default::default(),
                company_address_kana_country: core::default::Default::default(),
                company_address_kana_line1: core::default::Default::default(),
                company_address_kana_line2: core::default::Default::default(),
                company_address_kana_postal_code: core::default::Default::default(),
                company_address_kana_state: core::default::Default::default(),
                company_address_kana_town: core::default::Default::default(),
                company_address_kanji_city: core::default::Default::default(),
                company_address_kanji_country: core::default::Default::default(),
                company_address_kanji_line1: core::default::Default::default(),
                company_address_kanji_line2: core::default::Default::default(),
                company_address_kanji_postal_code: core::default::Default::default(),
                company_address_kanji_state: core::default::Default::default(),
                company_address_kanji_town: core::default::Default::default(),
                company_address_line1: core::default::Default::default(),
                company_address_line2: core::default::Default::default(),
                company_address_postal_code: core::default::Default::default(),
                company_address_state: core::default::Default::default(),
                company_directors_provided: core::default::Default::default(),
                company_executives_provided: core::default::Default::default(),
                company_name: core::default::Default::default(),
                company_name_kana: core::default::Default::default(),
                company_name_kanji: core::default::Default::default(),
                company_owners_provided: core::default::Default::default(),
                company_ownership_declaration_date: core::default::Default::default(),
                company_ownership_declaration_ip: core::default::Default::default(),
                company_ownership_declaration_user_agent: core::default::Default::default(),
                company_phone: core::default::Default::default(),
                company_registration_number: core::default::Default::default(),
                company_structure: core::default::Default::default(),
                company_tax_id: core::default::Default::default(),
                company_tax_id_registrar: core::default::Default::default(),
                company_vat_id: core::default::Default::default(),
                company_verification_document_back: core::default::Default::default(),
                company_verification_document_front: core::default::Default::default(),
                country: core::default::Default::default(),
                default_currency: core::default::Default::default(),
                documents_bank_account_ownership_verification_files: core::default::Default::default(),
                documents_company_license_files: core::default::Default::default(),
                documents_company_memorandum_of_association_files: core::default::Default::default(),
                documents_company_ministerial_decree_files: core::default::Default::default(),
                documents_company_registration_verification_files: core::default::Default::default(),
                documents_company_tax_id_verification_files: core::default::Default::default(),
                documents_proof_of_registration_files: core::default::Default::default(),
                email: core::default::Default::default(),
                external_account: core::default::Default::default(),
                individual_address_city: core::default::Default::default(),
                individual_address_country: core::default::Default::default(),
                individual_address_kana_city: core::default::Default::default(),
                individual_address_kana_country: core::default::Default::default(),
                individual_address_kana_line1: core::default::Default::default(),
                individual_address_kana_line2: core::default::Default::default(),
                individual_address_kana_postal_code: core::default::Default::default(),
                individual_address_kana_state: core::default::Default::default(),
                individual_address_kana_town: core::default::Default::default(),
                individual_address_kanji_city: core::default::Default::default(),
                individual_address_kanji_country: core::default::Default::default(),
                individual_address_kanji_line1: core::default::Default::default(),
                individual_address_kanji_line2: core::default::Default::default(),
                individual_address_kanji_postal_code: core::default::Default::default(),
                individual_address_kanji_state: core::default::Default::default(),
                individual_address_kanji_town: core::default::Default::default(),
                individual_address_line1: core::default::Default::default(),
                individual_address_line2: core::default::Default::default(),
                individual_address_postal_code: core::default::Default::default(),
                individual_address_state: core::default::Default::default(),
                individual_dob_day: core::default::Default::default(),
                individual_dob_month: core::default::Default::default(),
                individual_dob_year: core::default::Default::default(),
                individual_email: core::default::Default::default(),
                individual_first_name: core::default::Default::default(),
                individual_first_name_kana: core::default::Default::default(),
                individual_first_name_kanji: core::default::Default::default(),
                individual_full_name_aliases: core::default::Default::default(),
                individual_gender: core::default::Default::default(),
                individual_id_number: core::default::Default::default(),
                individual_id_number_secondary: core::default::Default::default(),
                individual_last_name: core::default::Default::default(),
                individual_last_name_kana: core::default::Default::default(),
                individual_last_name_kanji: core::default::Default::default(),
                individual_maiden_name: core::default::Default::default(),
                individual_metadata: core::default::Default::default(),
                individual_phone: core::default::Default::default(),
                individual_political_exposure: core::default::Default::default(),
                individual_registered_address_city: core::default::Default::default(),
                individual_registered_address_country: core::default::Default::default(),
                individual_registered_address_line1: core::default::Default::default(),
                individual_registered_address_line2: core::default::Default::default(),
                individual_registered_address_postal_code: core::default::Default::default(),
                individual_registered_address_state: core::default::Default::default(),
                individual_ssn_last_4: core::default::Default::default(),
                individual_verification_additional_document_back: core::default::Default::default(),
                individual_verification_additional_document_front: core::default::Default::default(),
                individual_verification_document_back: core::default::Default::default(),
                individual_verification_document_front: core::default::Default::default(),
                settings_branding_icon: core::default::Default::default(),
                settings_branding_logo: core::default::Default::default(),
                settings_branding_primary_color: core::default::Default::default(),
                settings_branding_secondary_color: core::default::Default::default(),
                settings_card_issuing_tos_acceptance_date: core::default::Default::default(),
                settings_card_issuing_tos_acceptance_ip: core::default::Default::default(),
                settings_card_issuing_tos_acceptance_user_agent: core::default::Default::default(),
                settings_card_payments_decline_on_avs_failure: core::default::Default::default(),
                settings_card_payments_decline_on_cvc_failure: core::default::Default::default(),
                settings_card_payments_statement_descriptor_prefix: core::default::Default::default(),
                settings_card_payments_statement_descriptor_prefix_kana: core::default::Default::default(),
                settings_card_payments_statement_descriptor_prefix_kanji: core::default::Default::default(),
                settings_payments_statement_descriptor: core::default::Default::default(),
                settings_payments_statement_descriptor_kana: core::default::Default::default(),
                settings_payments_statement_descriptor_kanji: core::default::Default::default(),
                settings_payouts_debit_negative_balances: core::default::Default::default(),
                settings_payouts_schedule_delay_days: core::default::Default::default(),
                settings_payouts_schedule_interval: core::default::Default::default(),
                settings_payouts_schedule_monthly_anchor: core::default::Default::default(),
                settings_payouts_schedule_weekly_anchor: core::default::Default::default(),
                settings_payouts_statement_descriptor: core::default::Default::default(),
                settings_treasury_tos_acceptance_date: core::default::Default::default(),
                settings_treasury_tos_acceptance_ip: core::default::Default::default(),
                settings_treasury_tos_acceptance_user_agent: core::default::Default::default(),
                tos_acceptance_date: core::default::Default::default(),
                tos_acceptance_ip: core::default::Default::default(),
                tos_acceptance_service_agreement: core::default::Default::default(),
                tos_acceptance_user_agent: core::default::Default::default(),
                type_: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccountRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_token` after provisioning.\nAn [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account."]
    pub fn account_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_token", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bank_account` after provisioning.\nEither a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary containing a user's bank account details."]
    pub fn bank_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bank_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_mcc` after provisioning.\n[The merchant category code for the account](https://stripe.com/docs/connect/setting-mcc). MCCs are used to classify businesses based on the goods or services they provide."]
    pub fn business_profile_mcc(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_mcc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_name` after provisioning.\nThe customer-facing business name."]
    pub fn business_profile_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_product_description` after provisioning.\nInternal-only description of the product sold or service provided by the business. It's used by Stripe for risk and underwriting purposes."]
    pub fn business_profile_product_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_product_description", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn business_profile_support_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn business_profile_support_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn business_profile_support_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn business_profile_support_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn business_profile_support_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_address_state` after provisioning.\nState, county, province, or region."]
    pub fn business_profile_support_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_address_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_email` after provisioning.\nA publicly available email address for sending support issues to."]
    pub fn business_profile_support_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_phone` after provisioning.\nA publicly available phone number to call with support issues."]
    pub fn business_profile_support_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.business_profile_support_phone", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `business_profile_support_url` after provisioning.\nA publicly available website for handling support issues."]
    pub fn business_profile_support_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_support_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `business_profile_url` after provisioning.\nThe business's publicly available website."]
    pub fn business_profile_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_profile_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `business_type` after provisioning.\nThe business type."]
    pub fn business_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.business_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capabilities_acss_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_acss_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_acss_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_affirm_payments_requested` after provisioning.\n"]
    pub fn capabilities_affirm_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_affirm_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_afterpay_clearpay_payments_requested` after provisioning.\n"]
    pub fn capabilities_afterpay_clearpay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_afterpay_clearpay_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_au_becs_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_au_becs_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_au_becs_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_bacs_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_bacs_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_bacs_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_bancontact_payments_requested` after provisioning.\n"]
    pub fn capabilities_bancontact_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_bancontact_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_bank_transfer_payments_requested` after provisioning.\n"]
    pub fn capabilities_bank_transfer_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_bank_transfer_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_blik_payments_requested` after provisioning.\n"]
    pub fn capabilities_blik_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_blik_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_boleto_payments_requested` after provisioning.\n"]
    pub fn capabilities_boleto_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_boleto_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_card_issuing_requested` after provisioning.\n"]
    pub fn capabilities_card_issuing_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_card_issuing_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_card_payments_requested` after provisioning.\n"]
    pub fn capabilities_card_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_card_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_cartes_bancaires_payments_requested` after provisioning.\n"]
    pub fn capabilities_cartes_bancaires_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_cartes_bancaires_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_eps_payments_requested` after provisioning.\n"]
    pub fn capabilities_eps_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_eps_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_fpx_payments_requested` after provisioning.\n"]
    pub fn capabilities_fpx_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_fpx_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_giropay_payments_requested` after provisioning.\n"]
    pub fn capabilities_giropay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_giropay_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_grabpay_payments_requested` after provisioning.\n"]
    pub fn capabilities_grabpay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_grabpay_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_ideal_payments_requested` after provisioning.\n"]
    pub fn capabilities_ideal_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_ideal_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_jcb_payments_requested` after provisioning.\n"]
    pub fn capabilities_jcb_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_jcb_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_klarna_payments_requested` after provisioning.\n"]
    pub fn capabilities_klarna_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_klarna_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_konbini_payments_requested` after provisioning.\n"]
    pub fn capabilities_konbini_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_konbini_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_legacy_payments_requested` after provisioning.\n"]
    pub fn capabilities_legacy_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_legacy_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_link_payments_requested` after provisioning.\n"]
    pub fn capabilities_link_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_link_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_oxxo_payments_requested` after provisioning.\n"]
    pub fn capabilities_oxxo_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_oxxo_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_p24_payments_requested` after provisioning.\n"]
    pub fn capabilities_p24_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_p24_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_paynow_payments_requested` after provisioning.\n"]
    pub fn capabilities_paynow_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_paynow_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_promptpay_payments_requested` after provisioning.\n"]
    pub fn capabilities_promptpay_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_promptpay_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_sepa_debit_payments_requested` after provisioning.\n"]
    pub fn capabilities_sepa_debit_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_sepa_debit_payments_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_sofort_payments_requested` after provisioning.\n"]
    pub fn capabilities_sofort_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_sofort_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_tax_reporting_us_1099_k_requested` after provisioning.\n"]
    pub fn capabilities_tax_reporting_us_1099_k_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_tax_reporting_us_1099_k_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_tax_reporting_us_1099_misc_requested` after provisioning.\n"]
    pub fn capabilities_tax_reporting_us_1099_misc_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_tax_reporting_us_1099_misc_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_transfers_requested` after provisioning.\n"]
    pub fn capabilities_transfers_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_transfers_requested", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capabilities_treasury_requested` after provisioning.\n"]
    pub fn capabilities_treasury_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_treasury_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `capabilities_us_bank_account_ach_payments_requested` after provisioning.\n"]
    pub fn capabilities_us_bank_account_ach_payments_requested(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capabilities_us_bank_account_ach_payments_requested", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `charges_enabled` after provisioning.\nWhether the account can create live charges."]
    pub fn charges_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.charges_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn company_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn company_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kana_city` after provisioning.\nCity/Ward."]
    pub fn company_address_kana_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kana_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn company_address_kana_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kana_line1` after provisioning.\nBlock/Building number."]
    pub fn company_address_kana_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_line1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kana_line2` after provisioning.\nBuilding details."]
    pub fn company_address_kana_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kana_postal_code` after provisioning.\nZIP or postal code."]
    pub fn company_address_kana_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_address_kana_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `company_address_kana_state` after provisioning.\nPrefecture."]
    pub fn company_address_kana_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kana_town` after provisioning.\nTown/cho-me."]
    pub fn company_address_kana_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kana_town", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kanji_city` after provisioning.\nCity/Ward."]
    pub fn company_address_kanji_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn company_address_kanji_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_address_kanji_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_line1` after provisioning.\nBlock/Building number."]
    pub fn company_address_kanji_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_line2` after provisioning.\nBuilding details."]
    pub fn company_address_kanji_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_kanji_postal_code` after provisioning.\nZIP or postal code."]
    pub fn company_address_kanji_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_address_kanji_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `company_address_kanji_state` after provisioning.\nPrefecture."]
    pub fn company_address_kanji_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_address_kanji_town` after provisioning.\nTown/cho-me."]
    pub fn company_address_kanji_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_kanji_town", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn company_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn company_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn company_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_postal_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_address_state` after provisioning.\nState, county, province, or region."]
    pub fn company_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_directors_provided` after provisioning.\nWhether the company's directors have been provided. This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-directors_provided)."]
    pub fn company_directors_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_directors_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_executives_provided` after provisioning.\nWhether the company's executives have been provided. This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided."]
    pub fn company_executives_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_executives_provided", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_name` after provisioning.\nThe company's legal name."]
    pub fn company_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_name_kana` after provisioning.\nThe Kana variation of the company's legal name (Japan only)."]
    pub fn company_name_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name_kana", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_name_kanji` after provisioning.\nThe Kanji variation of the company's legal name (Japan only)."]
    pub fn company_name_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name_kanji", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_owners_provided` after provisioning.\nWhether the company's owners have been provided. This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://stripe.com/docs/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided. Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together)."]
    pub fn company_owners_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_owners_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_ownership_declaration_date` after provisioning.\nThe Unix timestamp marking when the beneficial owner attestation was made."]
    pub fn company_ownership_declaration_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_ownership_declaration_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_ownership_declaration_ip` after provisioning.\nThe IP address from which the beneficial owner attestation was made."]
    pub fn company_ownership_declaration_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_ownership_declaration_ip", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_ownership_declaration_user_agent` after provisioning.\nThe user-agent string from the browser where the beneficial owner attestation was made."]
    pub fn company_ownership_declaration_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_ownership_declaration_user_agent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_phone` after provisioning.\nThe company's phone number (used for verification)."]
    pub fn company_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_registration_number` after provisioning.\n"]
    pub fn company_registration_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_registration_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_structure` after provisioning.\nThe category identifying the legal structure of the company or legal entity. See [Business structure](https://stripe.com/docs/connect/identity-verification#business-structure) for more details."]
    pub fn company_structure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_structure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_tax_id` after provisioning.\n"]
    pub fn company_tax_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_tax_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_tax_id_provided` after provisioning.\nWhether the company's business ID number was provided."]
    pub fn company_tax_id_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_tax_id_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_tax_id_registrar` after provisioning.\nThe jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    pub fn company_tax_id_registrar(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_tax_id_registrar", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_vat_id` after provisioning.\n"]
    pub fn company_vat_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_vat_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_vat_id_provided` after provisioning.\nWhether the company's business VAT number was provided."]
    pub fn company_vat_id_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_vat_id_provided", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_back` after provisioning.\nThe back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`."]
    pub fn company_verification_document_back(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_back", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_details` after provisioning.\nA user-displayable string describing the verification state of this document."]
    pub fn company_verification_document_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_details_code` after provisioning.\nOne of `document_corrupt`, `document_expired`, `document_failed_copy`, `document_failed_greyscale`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_not_readable`, `document_not_uploaded`, `document_type_not_supported`, or `document_too_large`. A machine-readable code specifying the verification state for this document."]
    pub fn company_verification_document_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `company_verification_document_front` after provisioning.\nThe front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`."]
    pub fn company_verification_document_front(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.company_verification_document_front", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `controller_is_controller` after provisioning.\n`true` if the Connect application retrieving the resource controls the account and can therefore exercise [platform controls](https://stripe.com/docs/connect/platform-controls-for-standard-accounts). Otherwise, this field is null."]
    pub fn controller_is_controller(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.controller_is_controller", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `controller_type` after provisioning.\nThe controller type. Can be `application`, if a Connect application controls the account, or `account`, if the account controls itself."]
    pub fn controller_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.controller_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `country` after provisioning.\nThe country in which the account holder resides, or in which the business is legally established. This should be an ISO 3166-1 alpha-2 country code. For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created. Available countries include [Stripe's global markets](https://stripe.com/global) as well as countries where [cross-border payouts](https://stripe.com/docs/connect/cross-border-payouts) are supported."]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created` after provisioning.\nTime at which the account was connected. Measured in seconds since the Unix epoch."]
    pub fn created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_currency` after provisioning.\nThree-letter ISO currency code representing the default currency for the account. This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts)."]
    pub fn default_currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_currency", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `details_submitted` after provisioning.\nWhether account details have been submitted. Standard accounts cannot receive payouts before this is true."]
    pub fn details_submitted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.details_submitted", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `documents_bank_account_ownership_verification_files` after provisioning.\n"]
    pub fn documents_bank_account_ownership_verification_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_bank_account_ownership_verification_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_company_license_files` after provisioning.\n"]
    pub fn documents_company_license_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_license_files", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `documents_company_memorandum_of_association_files` after provisioning.\n"]
    pub fn documents_company_memorandum_of_association_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_memorandum_of_association_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_company_ministerial_decree_files` after provisioning.\n"]
    pub fn documents_company_ministerial_decree_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_ministerial_decree_files", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `documents_company_registration_verification_files` after provisioning.\n"]
    pub fn documents_company_registration_verification_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_registration_verification_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_company_tax_id_verification_files` after provisioning.\n"]
    pub fn documents_company_tax_id_verification_files(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_company_tax_id_verification_files", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `documents_proof_of_registration_files` after provisioning.\n"]
    pub fn documents_proof_of_registration_files(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.documents_proof_of_registration_files", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nThe email address of the account holder. This is only to make the account easier to identify to you. Stripe only emails Custom accounts with your consent."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_account` after provisioning.\nA card or bank account to attach to the account for receiving [payouts](https://stripe.com/docs/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups). You can provide either a token, like the ones returned by [Stripe.js](https://stripe.com/docs/js), or a dictionary, as documented in the `external_account` parameter for [bank account](https://stripe.com/docs/api#account_create_bank_account) creation. <br><br>By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists. To add additional external accounts without replacing the existing default for the currency, use the [bank account](https://stripe.com/docs/api#account_create_bank_account) or [card creation](https://stripe.com/docs/api#account_create_card) APIs."]
    pub fn external_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_accounts_has_more` after provisioning.\nTrue if this list has another page of items after this one that can be fetched."]
    pub fn external_accounts_has_more(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_accounts_has_more", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_accounts_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value. Always has the value `list`."]
    pub fn external_accounts_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_accounts_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `external_accounts_url` after provisioning.\nThe URL where this list can be accessed."]
    pub fn external_accounts_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_accounts_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn future_requirements_alternatives(&self) -> terrars::ListRef<Account_FutureRequirementsAlternativesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_alternatives", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_current_deadline` after provisioning.\nDate on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty. After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on its enablement state prior to transitioning."]
    pub fn future_requirements_current_deadline(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.future_requirements_current_deadline", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_currently_due` after provisioning.\nFields that need to be collected to keep the account enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash."]
    pub fn future_requirements_currently_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_currently_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_disabled_reason` after provisioning.\nThis is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account."]
    pub fn future_requirements_disabled_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.future_requirements_disabled_reason", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn future_requirements_errors(&self) -> terrars::ListRef<Account_FutureRequirementsErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.future_requirements_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well."]
    pub fn future_requirements_eventually_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_eventually_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_past_due` after provisioning.\nFields that weren't collected by `requirements.current_deadline`. These fields need to be collected to enable the capability on the account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`."]
    pub fn future_requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.future_requirements_past_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `future_requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`."]
    pub fn future_requirements_pending_verification(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.future_requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nUnique identifier for the object."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_account` after provisioning.\nThe account the person is associated with."]
    pub fn individual_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn individual_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_country", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_address_kana_city` after provisioning.\nCity/Ward."]
    pub fn individual_address_kana_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_kana_city", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_address_kana_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_line1` after provisioning.\nBlock/Building number."]
    pub fn individual_address_kana_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_line2` after provisioning.\nBuilding details."]
    pub fn individual_address_kana_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kana_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_address_kana_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kana_state` after provisioning.\nPrefecture."]
    pub fn individual_address_kana_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kana_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kana_town` after provisioning.\nTown/cho-me."]
    pub fn individual_address_kana_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_kana_town", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_address_kanji_city` after provisioning.\nCity/Ward."]
    pub fn individual_address_kanji_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_address_kanji_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_line1` after provisioning.\nBlock/Building number."]
    pub fn individual_address_kanji_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_line2` after provisioning.\nBuilding details."]
    pub fn individual_address_kanji_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_kanji_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_address_kanji_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_postal_code", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kanji_state` after provisioning.\nPrefecture."]
    pub fn individual_address_kanji_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_address_kanji_town` after provisioning.\nTown/cho-me."]
    pub fn individual_address_kanji_town(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_kanji_town", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn individual_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_line1", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn individual_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_line2", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_address_state` after provisioning.\nState, county, province, or region."]
    pub fn individual_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_address_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_created` after provisioning.\nTime at which the object was created. Measured in seconds since the Unix epoch."]
    pub fn individual_created(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_created", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_dob_day` after provisioning.\nThe day of birth, between 1 and 31."]
    pub fn individual_dob_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_dob_day", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_dob_month` after provisioning.\nThe month of birth, between 1 and 12."]
    pub fn individual_dob_month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_dob_month", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_dob_year` after provisioning.\nThe four-digit year of birth."]
    pub fn individual_dob_year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_dob_year", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_email` after provisioning.\nThe person's email address."]
    pub fn individual_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_email", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_first_name` after provisioning.\nThe person's first name."]
    pub fn individual_first_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_first_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_first_name_kana` after provisioning.\nThe Kana variation of the person's first name (Japan only)."]
    pub fn individual_first_name_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_first_name_kana", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_first_name_kanji` after provisioning.\nThe Kanji variation of the person's first name (Japan only)."]
    pub fn individual_first_name_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_first_name_kanji", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_full_name_aliases` after provisioning.\nA list of alternate names or aliases that the person is known by."]
    pub fn individual_full_name_aliases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.individual_full_name_aliases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn individual_future_requirements_alternatives(
        &self,
    ) -> terrars::ListRef<Account_IndividualFutureRequirementsAlternativesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_alternatives", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_currently_due` after provisioning.\nFields that need to be collected to keep the person's account enabled. If not collected by the account's `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash, and may immediately become `past_due`, but the account may also be given a grace period depending on the account's enablement state prior to transition."]
    pub fn individual_future_requirements_currently_due(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_currently_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn individual_future_requirements_errors(
        &self,
    ) -> terrars::ListRef<Account_IndividualFutureRequirementsErrorsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_errors", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and the account's `future_requirements[current_deadline]` becomes set."]
    pub fn individual_future_requirements_eventually_due(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_eventually_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_past_due` after provisioning.\nFields that weren't collected by the account's `requirements.current_deadline`. These fields need to be collected to enable the person's account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`."]
    pub fn individual_future_requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_past_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_future_requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`."]
    pub fn individual_future_requirements_pending_verification(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_future_requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_gender` after provisioning.\nThe person's gender (International regulations require either \"male\" or \"female\")."]
    pub fn individual_gender(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_gender", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_id` after provisioning.\nUnique identifier for the object."]
    pub fn individual_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_id_number` after provisioning.\n"]
    pub fn individual_id_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_id_number", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_id_number_provided` after provisioning.\nWhether the person's `id_number` was provided."]
    pub fn individual_id_number_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_id_number_provided", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_id_number_secondary` after provisioning.\n"]
    pub fn individual_id_number_secondary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_id_number_secondary", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_id_number_secondary_provided` after provisioning.\nWhether the person's `id_number_secondary` was provided."]
    pub fn individual_id_number_secondary_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_id_number_secondary_provided", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_last_name` after provisioning.\nThe person's last name."]
    pub fn individual_last_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_last_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_last_name_kana` after provisioning.\nThe Kana variation of the person's last name (Japan only)."]
    pub fn individual_last_name_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_last_name_kana", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_last_name_kanji` after provisioning.\nThe Kanji variation of the person's last name (Japan only)."]
    pub fn individual_last_name_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_last_name_kanji", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_maiden_name` after provisioning.\nThe person's maiden name."]
    pub fn individual_maiden_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_maiden_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_metadata` after provisioning.\nSet of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format."]
    pub fn individual_metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.individual_metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_nationality` after provisioning.\nThe country where the person is a national."]
    pub fn individual_nationality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_nationality", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn individual_object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `individual_phone` after provisioning.\nThe person's phone number."]
    pub fn individual_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_phone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_political_exposure` after provisioning.\nIndicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction."]
    pub fn individual_political_exposure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_political_exposure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_city` after provisioning.\nCity, district, suburb, town, or village."]
    pub fn individual_registered_address_city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_city", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_country` after provisioning.\nTwo-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub fn individual_registered_address_country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_country", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_line1` after provisioning.\nAddress line 1 (e.g., street, PO Box, or company name)."]
    pub fn individual_registered_address_line1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_line1", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_line2` after provisioning.\nAddress line 2 (e.g., apartment, suite, unit, or building)."]
    pub fn individual_registered_address_line2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_line2", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_postal_code` after provisioning.\nZIP or postal code."]
    pub fn individual_registered_address_postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_postal_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_registered_address_state` after provisioning.\nState, county, province, or region."]
    pub fn individual_registered_address_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_registered_address_state", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_director` after provisioning.\nWhether the person is a director of the account's legal entity. Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations."]
    pub fn individual_relationship_director(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_director", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_executive` after provisioning.\nWhether the person has significant responsibility to control, manage, or direct the organization."]
    pub fn individual_relationship_executive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_executive", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_owner` after provisioning.\nWhether the person is an owner of the account’s legal entity."]
    pub fn individual_relationship_owner(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_owner", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_percent_ownership` after provisioning.\nThe percent owned by the person of the account's legal entity."]
    pub fn individual_relationship_percent_ownership(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_percent_ownership", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_representative` after provisioning.\nWhether the person is authorized as the primary representative of the account. This is the person nominated by the business to provide information about themselves, and general information about the account. There can only be one representative at any given time. At the time the account is created, this person should be set to the person responsible for opening the account."]
    pub fn individual_relationship_representative(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_representative", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_relationship_title` after provisioning.\nThe person's title (e.g., CEO, Support Engineer)."]
    pub fn individual_relationship_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_relationship_title", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn individual_requirements_alternatives(
        &self,
    ) -> terrars::ListRef<Account_IndividualRequirementsAlternativesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_alternatives", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_currently_due` after provisioning.\nFields that need to be collected to keep the person's account enabled. If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled."]
    pub fn individual_requirements_currently_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_currently_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn individual_requirements_errors(&self) -> terrars::ListRef<Account_IndividualRequirementsErrorsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_errors", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and the account's `current_deadline` becomes set."]
    pub fn individual_requirements_eventually_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_eventually_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_past_due` after provisioning.\nFields that weren't collected by the account's `current_deadline`. These fields need to be collected to enable the person's account."]
    pub fn individual_requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_past_due", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`."]
    pub fn individual_requirements_pending_verification(
        &self,
    ) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.individual_requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `individual_ssn_last_4` after provisioning.\n"]
    pub fn individual_ssn_last_4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.individual_ssn_last_4", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `individual_ssn_last_4_provided` after provisioning.\nWhether the last four digits of the person's Social Security number have been provided (U.S. only)."]
    pub fn individual_ssn_last_4_provided(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_ssn_last_4_provided", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_back` after provisioning.\nThe back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_additional_document_back(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_back", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_details` after provisioning.\nA user-displayable string describing the verification state of this document. For example, if a document is uploaded and the picture is too fuzzy, this may say \"Identity document is too unclear to read\"."]
    pub fn individual_verification_additional_document_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_details_code` after provisioning.\nOne of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`. A machine-readable code specifying the verification state for this document."]
    pub fn individual_verification_additional_document_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_additional_document_front` after provisioning.\nThe front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_additional_document_front(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_additional_document_front", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_details` after provisioning.\nA user-displayable string describing the verification state for the person. For example, this may say \"Provided identity information could not be verified\"."]
    pub fn individual_verification_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_details_code` after provisioning.\nOne of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`. A machine-readable code specifying the verification state for the person."]
    pub fn individual_verification_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_back` after provisioning.\nThe back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_document_back(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_back", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_details` after provisioning.\nA user-displayable string describing the verification state of this document. For example, if a document is uploaded and the picture is too fuzzy, this may say \"Identity document is too unclear to read\"."]
    pub fn individual_verification_document_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_details", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_details_code` after provisioning.\nOne of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`. A machine-readable code specifying the verification state for this document."]
    pub fn individual_verification_document_details_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_details_code", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_document_front` after provisioning.\nThe front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`."]
    pub fn individual_verification_document_front(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_document_front", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `individual_verification_status` after provisioning.\nThe state of verification for the person. Possible values are `unverified`, `pending`, or `verified`."]
    pub fn individual_verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.individual_verification_status", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nString representing the object's type. Objects of the same type share the same value."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payouts_enabled` after provisioning.\nWhether Stripe can send payouts to this account."]
    pub fn payouts_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payouts_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_alternatives` after provisioning.\nFields that are due and can be satisfied by providing the corresponding alternative fields instead."]
    pub fn requirements_alternatives(&self) -> terrars::ListRef<Account_RequirementsAlternativesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_alternatives", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_current_deadline` after provisioning.\nDate by which the fields in `currently_due` must be collected to keep the account enabled. These fields may disable the account sooner if the next threshold is reached before they are collected."]
    pub fn requirements_current_deadline(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.requirements_current_deadline", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `requirements_currently_due` after provisioning.\nFields that need to be collected to keep the account enabled. If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled."]
    pub fn requirements_currently_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_currently_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_disabled_reason` after provisioning.\nIf the account is disabled, this string describes why. Can be `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.listed`, `rejected.terms_of_service`, `rejected.other`, `under_review`, or `other`."]
    pub fn requirements_disabled_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirements_disabled_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_errors` after provisioning.\nFields that are `currently_due` and need to be collected again because validation or verification failed."]
    pub fn requirements_errors(&self) -> terrars::ListRef<Account_RequirementsErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_errors", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_eventually_due` after provisioning.\nFields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set."]
    pub fn requirements_eventually_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_eventually_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_past_due` after provisioning.\nFields that weren't collected by `current_deadline`. These fields need to be collected to enable the account."]
    pub fn requirements_past_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requirements_past_due", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `requirements_pending_verification` after provisioning.\nFields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`."]
    pub fn requirements_pending_verification(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.requirements_pending_verification", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_bacs_debit_payments_display_name` after provisioning.\nThe Bacs Direct Debit Display Name for this account. For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor."]
    pub fn settings_bacs_debit_payments_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_bacs_debit_payments_display_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_icon` after provisioning.\n(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account. Must be square and at least 128px x 128px."]
    pub fn settings_branding_icon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_branding_icon", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_logo` after provisioning.\n(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided. Must be at least 128px x 128px."]
    pub fn settings_branding_logo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_branding_logo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_primary_color` after provisioning.\nA CSS hex color value representing the primary branding color for this account"]
    pub fn settings_branding_primary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_branding_primary_color", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_branding_secondary_color` after provisioning.\nA CSS hex color value representing the secondary branding color for this account"]
    pub fn settings_branding_secondary_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_branding_secondary_color", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_issuing_tos_acceptance_date` after provisioning.\nThe Unix timestamp marking when the account representative accepted the service agreement."]
    pub fn settings_card_issuing_tos_acceptance_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_issuing_tos_acceptance_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_issuing_tos_acceptance_ip` after provisioning.\nThe IP address from which the account representative accepted the service agreement."]
    pub fn settings_card_issuing_tos_acceptance_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_issuing_tos_acceptance_ip", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_issuing_tos_acceptance_user_agent` after provisioning.\nThe user agent of the browser from which the account representative accepted the service agreement."]
    pub fn settings_card_issuing_tos_acceptance_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_issuing_tos_acceptance_user_agent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_decline_on_avs_failure` after provisioning.\nWhether Stripe automatically declines charges with an incorrect ZIP or postal code. This setting only applies when a ZIP or postal code is provided and they fail bank verification."]
    pub fn settings_card_payments_decline_on_avs_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_decline_on_avs_failure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_decline_on_cvc_failure` after provisioning.\nWhether Stripe automatically declines charges with an incorrect CVC. This setting only applies when a CVC is provided and it fails bank verification."]
    pub fn settings_card_payments_decline_on_cvc_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_decline_on_cvc_failure", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_statement_descriptor_prefix` after provisioning.\nThe default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge. `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_card_payments_statement_descriptor_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_statement_descriptor_prefix", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_statement_descriptor_prefix_kana` after provisioning.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_card_payments_statement_descriptor_prefix_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_statement_descriptor_prefix_kana", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_card_payments_statement_descriptor_prefix_kanji` after provisioning.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_card_payments_statement_descriptor_prefix_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_card_payments_statement_descriptor_prefix_kanji", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_dashboard_display_name` after provisioning.\nThe display name for this account. This is used on the Stripe Dashboard to differentiate between accounts."]
    pub fn settings_dashboard_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_dashboard_display_name", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_dashboard_timezone` after provisioning.\nThe timezone used in the Stripe Dashboard for this account. A list of possible time zone values is maintained at the [IANA Time Zone Database](http://www.iana.org/time-zones)."]
    pub fn settings_dashboard_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.settings_dashboard_timezone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor` after provisioning.\nThe default text that appears on credit card statements when a charge is made. This field prefixes any dynamic `statement_descriptor` specified on the charge."]
    pub fn settings_payments_statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_kana` after provisioning.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only)"]
    pub fn settings_payments_statement_descriptor_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_kana", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_kanji` after provisioning.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only)"]
    pub fn settings_payments_statement_descriptor_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_kanji", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_prefix_kana` after provisioning.\nThe Kana variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge. `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_payments_statement_descriptor_prefix_kana(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_prefix_kana", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payments_statement_descriptor_prefix_kanji` after provisioning.\nThe Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only). This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge. `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion."]
    pub fn settings_payments_statement_descriptor_prefix_kanji(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payments_statement_descriptor_prefix_kanji", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_debit_negative_balances` after provisioning.\nA Boolean indicating if Stripe should try to reclaim negative balances from an attached bank account. See our [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances) documentation for details. Default value is `false` for Custom accounts, otherwise `true`."]
    pub fn settings_payouts_debit_negative_balances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_debit_negative_balances", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_delay_days` after provisioning.\nThe number of days charges for the account will be held before being paid out."]
    pub fn settings_payouts_schedule_delay_days(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_delay_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_interval` after provisioning.\nHow frequently funds will be paid out. One of `manual` (payouts only created via API call), `daily`, `weekly`, or `monthly`."]
    pub fn settings_payouts_schedule_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_interval", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_monthly_anchor` after provisioning.\nThe day of the month funds will be paid out. Only shown if `interval` is monthly. Payouts scheduled between the 29th and 31st of the month are sent on the last day of shorter months."]
    pub fn settings_payouts_schedule_monthly_anchor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_monthly_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_schedule_weekly_anchor` after provisioning.\nThe day of the week funds will be paid out, of the style 'monday', 'tuesday', etc. Only shown if `interval` is weekly."]
    pub fn settings_payouts_schedule_weekly_anchor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_schedule_weekly_anchor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_payouts_statement_descriptor` after provisioning.\nThe text that appears on the bank account statement for payouts. If not set, this defaults to the platform's bank descriptor as set in the Dashboard."]
    pub fn settings_payouts_statement_descriptor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_payouts_statement_descriptor", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_sepa_debit_payments_creditor_id` after provisioning.\nSEPA creditor identifier that identifies the company making the payment."]
    pub fn settings_sepa_debit_payments_creditor_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_sepa_debit_payments_creditor_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_treasury_tos_acceptance_date` after provisioning.\nThe Unix timestamp marking when the account representative accepted the service agreement."]
    pub fn settings_treasury_tos_acceptance_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_treasury_tos_acceptance_date", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_treasury_tos_acceptance_ip` after provisioning.\nThe IP address from which the account representative accepted the service agreement."]
    pub fn settings_treasury_tos_acceptance_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_treasury_tos_acceptance_ip", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `settings_treasury_tos_acceptance_user_agent` after provisioning.\nThe user agent of the browser from which the account representative accepted the service agreement."]
    pub fn settings_treasury_tos_acceptance_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.settings_treasury_tos_acceptance_user_agent", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_date` after provisioning.\nThe Unix timestamp marking when the account representative accepted their service agreement"]
    pub fn tos_acceptance_date(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_acceptance_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_ip` after provisioning.\nThe IP address from which the account representative accepted their service agreement"]
    pub fn tos_acceptance_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_acceptance_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_service_agreement` after provisioning.\nThe user's service agreement type"]
    pub fn tos_acceptance_service_agreement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tos_acceptance_service_agreement", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `tos_acceptance_user_agent` after provisioning.\nThe user agent of the browser from which the account representative accepted their service agreement"]
    pub fn tos_acceptance_user_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tos_acceptance_user_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of Stripe account to create. May be one of `custom`, `express` or `standard`."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Account_FutureRequirementsAlternatives {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Account_FutureRequirementsAlternatives {
    #[doc = "Set the field `alternative_fields_due`.\n"]
    pub fn set_alternative_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alternative_fields_due = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `original_fields_due`.\n"]
    pub fn set_original_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.original_fields_due = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_FutureRequirementsAlternatives {
    type O = terrars::BlockAssignable<Account_FutureRequirementsAlternatives>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_FutureRequirementsAlternatives {}

impl BuildAccount_FutureRequirementsAlternatives {
    pub fn build(self) -> Account_FutureRequirementsAlternatives {
        Account_FutureRequirementsAlternatives {
            alternative_fields_due: core::default::Default::default(),
            original_fields_due: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_FutureRequirementsAlternativesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_FutureRequirementsAlternativesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_FutureRequirementsAlternativesRef {
        Account_FutureRequirementsAlternativesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_FutureRequirementsAlternativesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternative_fields_due` after provisioning.\n"]
    pub fn alternative_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternative_fields_due", self.base))
    }

    #[doc = "Get a reference to the value of field `original_fields_due` after provisioning.\n"]
    pub fn original_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.original_fields_due", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_FutureRequirementsErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Account_FutureRequirementsErrors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirement`.\n"]
    pub fn set_requirement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.requirement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_FutureRequirementsErrors {
    type O = terrars::BlockAssignable<Account_FutureRequirementsErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_FutureRequirementsErrors {}

impl BuildAccount_FutureRequirementsErrors {
    pub fn build(self) -> Account_FutureRequirementsErrors {
        Account_FutureRequirementsErrors {
            code: core::default::Default::default(),
            reason: core::default::Default::default(),
            requirement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_FutureRequirementsErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_FutureRequirementsErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_FutureRequirementsErrorsRef {
        Account_FutureRequirementsErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_FutureRequirementsErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_IndividualFutureRequirementsAlternatives {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Account_IndividualFutureRequirementsAlternatives {
    #[doc = "Set the field `alternative_fields_due`.\n"]
    pub fn set_alternative_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alternative_fields_due = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `original_fields_due`.\n"]
    pub fn set_original_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.original_fields_due = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_IndividualFutureRequirementsAlternatives {
    type O = terrars::BlockAssignable<Account_IndividualFutureRequirementsAlternatives>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_IndividualFutureRequirementsAlternatives {}

impl BuildAccount_IndividualFutureRequirementsAlternatives {
    pub fn build(self) -> Account_IndividualFutureRequirementsAlternatives {
        Account_IndividualFutureRequirementsAlternatives {
            alternative_fields_due: core::default::Default::default(),
            original_fields_due: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_IndividualFutureRequirementsAlternativesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_IndividualFutureRequirementsAlternativesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Account_IndividualFutureRequirementsAlternativesRef {
        Account_IndividualFutureRequirementsAlternativesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_IndividualFutureRequirementsAlternativesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternative_fields_due` after provisioning.\n"]
    pub fn alternative_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternative_fields_due", self.base))
    }

    #[doc = "Get a reference to the value of field `original_fields_due` after provisioning.\n"]
    pub fn original_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.original_fields_due", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_IndividualFutureRequirementsErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Account_IndividualFutureRequirementsErrors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirement`.\n"]
    pub fn set_requirement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.requirement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_IndividualFutureRequirementsErrors {
    type O = terrars::BlockAssignable<Account_IndividualFutureRequirementsErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_IndividualFutureRequirementsErrors {}

impl BuildAccount_IndividualFutureRequirementsErrors {
    pub fn build(self) -> Account_IndividualFutureRequirementsErrors {
        Account_IndividualFutureRequirementsErrors {
            code: core::default::Default::default(),
            reason: core::default::Default::default(),
            requirement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_IndividualFutureRequirementsErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_IndividualFutureRequirementsErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_IndividualFutureRequirementsErrorsRef {
        Account_IndividualFutureRequirementsErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_IndividualFutureRequirementsErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_IndividualRequirementsAlternatives {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Account_IndividualRequirementsAlternatives {
    #[doc = "Set the field `alternative_fields_due`.\n"]
    pub fn set_alternative_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alternative_fields_due = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `original_fields_due`.\n"]
    pub fn set_original_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.original_fields_due = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_IndividualRequirementsAlternatives {
    type O = terrars::BlockAssignable<Account_IndividualRequirementsAlternatives>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_IndividualRequirementsAlternatives {}

impl BuildAccount_IndividualRequirementsAlternatives {
    pub fn build(self) -> Account_IndividualRequirementsAlternatives {
        Account_IndividualRequirementsAlternatives {
            alternative_fields_due: core::default::Default::default(),
            original_fields_due: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_IndividualRequirementsAlternativesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_IndividualRequirementsAlternativesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_IndividualRequirementsAlternativesRef {
        Account_IndividualRequirementsAlternativesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_IndividualRequirementsAlternativesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternative_fields_due` after provisioning.\n"]
    pub fn alternative_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternative_fields_due", self.base))
    }

    #[doc = "Get a reference to the value of field `original_fields_due` after provisioning.\n"]
    pub fn original_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.original_fields_due", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_IndividualRequirementsErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Account_IndividualRequirementsErrors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirement`.\n"]
    pub fn set_requirement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.requirement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_IndividualRequirementsErrors {
    type O = terrars::BlockAssignable<Account_IndividualRequirementsErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_IndividualRequirementsErrors {}

impl BuildAccount_IndividualRequirementsErrors {
    pub fn build(self) -> Account_IndividualRequirementsErrors {
        Account_IndividualRequirementsErrors {
            code: core::default::Default::default(),
            reason: core::default::Default::default(),
            requirement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_IndividualRequirementsErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_IndividualRequirementsErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_IndividualRequirementsErrorsRef {
        Account_IndividualRequirementsErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_IndividualRequirementsErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_RequirementsAlternatives {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternative_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    original_fields_due: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Account_RequirementsAlternatives {
    #[doc = "Set the field `alternative_fields_due`.\n"]
    pub fn set_alternative_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alternative_fields_due = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `original_fields_due`.\n"]
    pub fn set_original_fields_due(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.original_fields_due = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_RequirementsAlternatives {
    type O = terrars::BlockAssignable<Account_RequirementsAlternatives>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_RequirementsAlternatives {}

impl BuildAccount_RequirementsAlternatives {
    pub fn build(self) -> Account_RequirementsAlternatives {
        Account_RequirementsAlternatives {
            alternative_fields_due: core::default::Default::default(),
            original_fields_due: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_RequirementsAlternativesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_RequirementsAlternativesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_RequirementsAlternativesRef {
        Account_RequirementsAlternativesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_RequirementsAlternativesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternative_fields_due` after provisioning.\n"]
    pub fn alternative_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternative_fields_due", self.base))
    }

    #[doc = "Get a reference to the value of field `original_fields_due` after provisioning.\n"]
    pub fn original_fields_due(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.original_fields_due", self.base))
    }
}

#[derive(Serialize)]
pub struct Account_RequirementsErrors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requirement: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Account_RequirementsErrors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requirement`.\n"]
    pub fn set_requirement(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.requirement = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Account_RequirementsErrors {
    type O = terrars::BlockAssignable<Account_RequirementsErrors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccount_RequirementsErrors {}

impl BuildAccount_RequirementsErrors {
    pub fn build(self) -> Account_RequirementsErrors {
        Account_RequirementsErrors {
            code: core::default::Default::default(),
            reason: core::default::Default::default(),
            requirement: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Account_RequirementsErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Account_RequirementsErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Account_RequirementsErrorsRef {
        Account_RequirementsErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Account_RequirementsErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }
}
