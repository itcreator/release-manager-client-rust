# \ProjectApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_project**](ProjectApi.md#create_project) | **Post** /projects | Create new projects
[**list_projects**](ProjectApi.md#list_projects) | **Get** /projects | List the projects
[**read_project**](ProjectApi.md#read_project) | **Get** /projects/{uuid} | Read the projects
[**update_project**](ProjectApi.md#update_project) | **Put** /projects/{uuid} | 


# **create_project**
> ::models::Project create_project(optional)
Create new projects

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**Project**](Project.md)|  | 

### Return type

[**::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_projects**
> Vec<::models::Project> list_projects()
List the projects

Get all projects list. 

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<::models::Project>**](Project.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_project**
> ::models::Project read_project(uuid)
Read the projects

Get all projects list 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **uuid** | [**String**](.md)| Project ID in UUID format | 

### Return type

[**::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_project**
> ::models::Project update_project(uuid, optional)


### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **uuid** | [**String**](.md)| Project ID in UUID format | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **uuid** | [**String**](.md)| Project ID in UUID format | 
 **body** | [**Project**](Project.md)|  | 

### Return type

[**::models::Project**](Project.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

