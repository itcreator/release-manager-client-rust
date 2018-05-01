# \VersionSemanticApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**semver_generate**](VersionSemanticApi.md#semver_generate) | **Post** /projects/{projectUuid}/version/semantic | Generate new semantic version number (based on gitflow)


# **semver_generate**
> ::models::SemverTagSet semver_generate(project_uuid, optional)
Generate new semantic version number (based on gitflow)

Semantic Versioning 2.0.0 See also http://semver.org/spec/v2.0.0.html Based on branching model [GitFlow](http://nvie.com/posts/a-successful-git-branching-model/) 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **project_uuid** | [**String**](.md)| Project ID in UUID format | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **project_uuid** | [**String**](.md)| Project ID in UUID format | 
 **body** | [**SemverGenerateParams**](SemverGenerateParams.md)|  | 

### Return type

[**::models::SemverTagSet**](SemverTagSet.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/release-manager.v1+json
 - **Accept**: application/release-manager.v1+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

