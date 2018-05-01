# \VersionIncrementalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**incremental_delete**](VersionIncrementalApi.md#incremental_delete) | **Delete** /increamental_version/{projectName} | Delete incremental version number (RESET)
[**incremental_generate**](VersionIncrementalApi.md#incremental_generate) | **Post** /increamental_version/{projectName} | Generate new incremental version number
[**incremental_update**](VersionIncrementalApi.md#incremental_update) | **Put** /increamental_version/{projectName} | Update incremental version number (Only for maintenance)


# **incremental_delete**
> incremental_delete(project_name)
Delete incremental version number (RESET)

Delete generated version 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **project_name** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incremental_generate**
> ::models::IncrementalVersionNumber incremental_generate(project_name)
Generate new incremental version number

Incremental Versioning 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **project_name** | **String**|  | 

### Return type

[**::models::IncrementalVersionNumber**](IncrementalVersionNumber.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **incremental_update**
> ::models::IncrementalVersionNumber incremental_update(project_name, optional)
Update incremental version number (Only for maintenance)

Incremental Versioning Update revision number 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **project_name** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **project_name** | **String**|  | 
 **body** | [**IncrementalVersionNumber**](IncrementalVersionNumber.md)|  | 

### Return type

[**::models::IncrementalVersionNumber**](IncrementalVersionNumber.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

