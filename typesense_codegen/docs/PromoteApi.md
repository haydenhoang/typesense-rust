# \PromoteApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_search_override_0**](PromoteApi.md#delete_search_override_0) | **DELETE** /collections/{collectionName}/overrides/{overrideId} | Delete an override associated with a collection
[**get_search_overrides_0**](PromoteApi.md#get_search_overrides_0) | **GET** /collections/{collectionName}/overrides | List all collection overrides
[**upsert_search_override_0**](PromoteApi.md#upsert_search_override_0) | **PUT** /collections/{collectionName}/overrides/{overrideId} | Create or update an override to promote certain documents over others



## delete_search_override_0

> models::SearchOverride delete_search_override_0(collection_name, override_id)
Delete an override associated with a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The ID of the search override to delete | [required] |

### Return type

[**models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_overrides_0

> models::SearchOverridesResponse get_search_overrides_0(collection_name)
List all collection overrides

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |

### Return type

[**models::SearchOverridesResponse**](SearchOverridesResponse.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_search_override_0

> models::SearchOverride upsert_search_override_0(collection_name, override_id, search_override_schema)
Create or update an override to promote certain documents over others

Create or update an override to promote certain documents over others. Using overrides, you can include or exclude specific documents for a given query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_name** | **String** | The name of the collection | [required] |
**override_id** | **String** | The ID of the search override to create/update | [required] |
**search_override_schema** | [**SearchOverrideSchema**](SearchOverrideSchema.md) | The search override object to be created/updated | [required] |

### Return type

[**models::SearchOverride**](SearchOverride.md)

### Authorization

[api_key_header](../README.md#api_key_header)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

