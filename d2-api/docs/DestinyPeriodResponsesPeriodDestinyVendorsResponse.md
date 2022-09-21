# DestinyPeriodResponsesPeriodDestinyVendorsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vendor_groups** | Option<[**crate::models::DestinyResponsesDestinyVendorsResponseVendorGroups**](Destiny_Responses_DestinyVendorsResponse_vendorGroups.md)> |  | [optional]
**vendors** | Option<[**crate::models::DestinyResponsesDestinyVendorsResponseVendors**](Destiny_Responses_DestinyVendorsResponse_vendors.md)> |  | [optional]
**categories** | Option<[**crate::models::DestinyResponsesDestinyVendorsResponseCategories**](Destiny_Responses_DestinyVendorsResponse_categories.md)> |  | [optional]
**sales** | Option<[**crate::models::DestinyResponsesDestinyVendorsResponseSales**](Destiny_Responses_DestinyVendorsResponse_sales.md)> |  | [optional]
**item_components** | Option<[**::std::collections::HashMap<String, crate::models::DestinyItemComponentSetOfint32>**](DestinyItemComponentSetOfint32.md)> | The set of item detail components, one set of item components per Vendor. These are keyed by the Vendor Hash, so you will get one Item Component Set per vendor returned.  The components contained inside are themselves keyed by the vendorSaleIndex, and will have whatever item-level components you requested (Sockets, Stats, Instance data etc...) per item being sold by the vendor. | [optional]
**currency_lookups** | Option<[**crate::models::DestinyResponsesDestinyCharacterResponseCurrencyLookups**](Destiny_Responses_DestinyCharacterResponse_currencyLookups.md)> |  | [optional]
**string_variables** | Option<[**crate::models::DestinyResponsesDestinyVendorsResponseStringVariables**](Destiny_Responses_DestinyVendorsResponse_stringVariables.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


