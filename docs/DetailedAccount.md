# DetailedAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **i32** |  | 
**message** | Option<**String**> |  | [optional]
**account_type** | **i32** |  | 
**index** | **i64** |  | 
**l1_address** | **String** |  | 
**cancel_all_time** | **i64** |  | 
**total_order_count** | **i64** |  | 
**total_isolated_order_count** | **i64** |  | 
**pending_order_count** | **i64** |  | 
**available_balance** | **String** |  | 
**status** | **i32** |  | 
**collateral** | **String** |  | 
**account_index** | **i64** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**can_invite** | **bool** |  Remove After FE uses L1 meta endpoint | 
**referral_points_percentage** | **String** |  Remove After FE uses L1 meta endpoint | 
**positions** | [**Vec<models::AccountPosition>**](AccountPosition.md) |  | 
**total_asset_value** | Option<**String**> |  | [optional]
**cross_asset_value** | Option<**String**> |  | [optional]
**pool_info** | Option<[**models::PublicPoolInfo**](PublicPoolInfo.md)> |  | [optional]
**shares** | Option<[**Vec<models::PublicPoolShare>**](PublicPoolShare.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


