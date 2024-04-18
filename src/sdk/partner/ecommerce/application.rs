// 参考文档: https://pay.weixin.qq.com/docs/partner/apis/ecommerce-merchant-application/applyment/submit-applyment.html

use crate::{Client, WeChatPayError};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnNull};

#[derive(Debug, Serialize, Deserialize)]
pub enum OrganizationType {
  // 小微商户
  #[serde(rename = "2401")]
  SmallMicroMerchant,

  // 个人卖家
  #[serde(rename = "2500")]
  IndividualSeller,

  // 个体工商户
  #[serde(rename = "4")]
  IndividualBusiness,

  // 企业
  #[serde(rename = "2")]
  Enterprise,

  // 事业单位
  #[serde(rename = "3")]
  Institution,

  // 政府机构
  #[serde(rename = "2502")]
  GovernmentAgency,

  // 社会组织
  #[serde(rename = "1708")]
  SocialOrganization,
}

#[derive(Debug, Serialize, Deserialize)]
enum CertType {
  // 事业单位法人证书
  #[serde(rename = "CERTIFICATE_TYPE_2388")]
  InstitutionLegalCertificate,

  // 统一社会信用代码证书
  #[serde(rename = "CERTIFICATE_TYPE_2389")]
  UnifiedSocialCreditCodeCertificate,

  // 社会团体法人登记证书
  #[serde(rename = "CERTIFICATE_TYPE_2394")]
  SocialGroupLegalCertificate,

  // 民办非企业单位登记证书
  #[serde(rename = "CERTIFICATE_TYPE_2395")]
  PrivateNonEnterpriseUnitCertificate,

  // 基金会法人登记证书
  #[serde(rename = "CERTIFICATE_TYPE_2396")]
  FoundationLegalCertificate,

  // 宗教活动场所登记证
  #[serde(rename = "CERTIFICATE_TYPE_2399")]
  ReligiousActivityPlaceCertificate,

  // 执业许可证/执业证
  #[serde(rename = "CERTIFICATE_TYPE_2520")]
  PracticingLicense,

  // 基层群众性自治组织特别法人统一社会信用代码证
  #[serde(rename = "CERTIFICATE_TYPE_2521")]
  GrassrootsMassCertificate,

  // 农村集体经济组织登记证
  #[serde(rename = "CERTIFICATE_TYPE_2522")]
  RuralCollectiveCertificate,

  // other
  #[serde(rename = "CERTIFICATE_TYPE_2400")]
  Other,
}

#[derive(Debug, Serialize, Deserialize)]
enum FinanceType {
  #[serde(rename = "BANK_AGENT")]
  BankAgent,
  #[serde(rename = "PAYMENT_AGENT")]
  PaymentAgent,
  #[serde(rename = "INSURANCE")]
  Insurance,
  #[serde(rename = "TRADE_AND_SETTLE")]
  TradeAndSettle,
  #[serde(rename = "OTHER")]
  Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinanceInstitutionInfo {
  finance_type: FinanceType,
  finance_license_pics: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessLicenseInfo {
  cert_type: Option<CertType>,
  business_license_copy: String,
  business_license_number: String,
  merchant_name: String,
  legal_person: String,
  company_address: Option<String>,
  business_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdHolderType {
  #[serde(rename = "LEGAL")]
  Legal,
  #[serde(rename = "SUPER")]
  Super,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IdDocType {
  #[serde(rename = "IDENTIFICATION_TYPE_MAINLAND_IDCARD")]
  MainlandIdcard,

  #[serde(rename = "IDENTIFICATION_TYPE_OVERSEA_PASSPORT")]
  OverseaPassport,

  #[serde(rename = "IDENTIFICATION_TYPE_HONGKONG")]
  Hongkong,

  #[serde(rename = "IDENTIFICATION_TYPE_MACAO")]
  Macao,

  #[serde(rename = "IDENTIFICATION_TYPE_TAIWAN")]
  Taiwan,

  #[serde(rename = "IDENTIFICATION_TYPE_FOREIGN_RESIDENT")]
  ForeignResident,

  #[serde(rename = "IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT")]
  HongkongMacaoResident,

  #[serde(rename = "IDENTIFICATION_TYPE_TAIWAN_RESIDENT")]
  TaiwanResident,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdCardInfo {
  id_card_copy: String,
  id_card_national: String,
  id_card_name: String,
  id_card_number: String,
  id_card_address: Option<String>,
  id_card_valid_time_begin: String,
  id_card_valid_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdDocInfo {
  id_doc_name: String,
  id_doc_number: String,
  id_doc_copy: String,
  id_doc_copy_back: Option<String>,
  id_doc_address: Option<String>,
  doc_period_begin: String,
  doc_period_end: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UboIdDocType {
  #[serde(rename = "IDENTIFICATION_TYPE_MAINLAND_IDCARD")]
  MainlandIdcard,

  #[serde(rename = "IDENTIFICATION_TYPE_OVERSEA_PASSPORT")]
  OverseaPassport,

  #[serde(rename = "IDENTIFICATION_TYPE_HONGKONG")]
  Hongkong,

  #[serde(rename = "IDENTIFICATION_TYPE_MACAO")]
  Macao,

  #[serde(rename = "IDENTIFICATION_TYPE_TAIWAN")]
  Taiwan,

  #[serde(rename = "IDENTIFICATION_TYPE_FOREIGN_RESIDENT")]
  ForeignResident,

  #[serde(rename = "IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT")]
  HongkongMacaoResident,

  #[serde(rename = "IDENTIFICATION_TYPE_TAIWAN_RESIDENT")]
  TaiwanResident,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UboInfo {
  ubo_id_doc_type: Option<UboIdDocType>,
  ubo_id_doc_copy: Option<String>,
  ubo_id_doc_copy_back: Option<String>,
  ubo_id_doc_name: Option<String>,
  ubo_id_doc_number: Option<String>,
  ubo_id_doc_address: Option<String>,
  ubo_id_doc_period_begin: Option<String>,
  ubo_id_doc_period_end: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
  account_type: String,
  account_bank: String,
  account_name: String,
  bank_address_code: String,
  bank_branch_id: Option<String>,
  bank_name: Option<String>,
  account_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInfo {
  contact_type: String,
  contact_name: String,
  contact_id_doc_type: Option<IdDocType>,
  contact_id_card_number: Option<String>,
  contact_id_doc_copy: Option<String>,
  contact_id_doc_copy_back: Option<String>,
  contact_id_doc_period_begin: Option<String>,
  contact_id_doc_period_end: Option<String>,
  business_authorization_letter: Option<String>,
  mobile_phone: String,
  contact_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesSceneInfo {
  store_name: String,
  store_url: Option<String>,
  store_qr_code: Option<String>,
  mini_program_sub_appid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementInfo {
  settlement_id: i32,
  qualification_type: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct MerchantSubmitInfo {
  pub organization_type: OrganizationType,

  #[serde(default)]
  #[serde_as(deserialize_as = "DefaultOnNull")]
  pub finance_institution: bool,

  pub business_license_info: Option<BusinessLicenseInfo>,
  pub finance_institution_info: Option<FinanceInstitutionInfo>,
  pub id_holder_type: Option<IdHolderType>,
  pub id_doc_type: IdDocType,
  pub authorize_letter_copy: Option<String>,
  pub id_card_info: Option<IdCardInfo>,
  pub id_doc_info: Option<IdDocInfo>,
  pub owner: Option<bool>,
  pub ubo_info_list: Option<Vec<UboInfo>>,
  pub account_info: AccountInfo,
  pub contact_info: ContactInfo,
  pub sales_scene_info: SalesSceneInfo,
  pub settlement_info: Option<SettlementInfo>,
  pub merchant_shortname: String,
  pub qualifications: Option<String>,
  pub business_addition_pics: Option<String>,
  pub business_addition_desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantApplyInfo {
  pub out_request_no: String,
  pub organization_type: OrganizationType,
  pub finance_institution: bool,
  pub business_license_info: Option<BusinessLicenseInfo>,
  pub finance_institution_info: Option<FinanceInstitutionInfo>,
  pub id_holder_type: Option<IdHolderType>,
  pub id_doc_type: IdDocType,
  pub authorize_letter_copy: Option<String>,
  pub id_card_info: Option<IdCardInfo>,
  pub id_doc_info: Option<IdDocInfo>,
  pub owner: Option<bool>,
  pub ubo_info_list: Option<Vec<UboInfo>>,
  pub account_info: AccountInfo,
  pub contact_info: ContactInfo,
  pub sales_scene_info: SalesSceneInfo,
  pub settlement_info: Option<SettlementInfo>,
  pub merchant_shortname: String,
  pub qualifications: Option<String>,
  pub business_addition_pics: Option<String>,
  pub business_addition_desc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MerchantApplyResponse {
  pub applyment_id: String,
  pub out_request_no: String,
}
impl Client {
  pub async fn merchant_apply(
    &self,
    req: &MerchantApplyInfo,
  ) -> Result<MerchantApplyResponse, WeChatPayError> {
    Ok(
      self
        .send_request(Method::POST, "/v3/ecommerce/applyments", None, Some(req))
        .await?
        .unwrap(),
    )
  }
}
