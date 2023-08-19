# contentfree

[![Crate](https://img.shields.io/crates/v/contentfree.svg)](https://crates.io/crates/contentfree)

このライブラリは[Contentful](https://www.contentful.com/)を rust で便利に使うために作られたものです。

# 使用例

## get_content

```
		let client = ContentfulClient::new("your-space".to_string(), "your-content-type".to_string());
		let response = client.get_content().await?;
		let json_response: ContentfulResponse<YourResponseJson> = response.json().await?;
```
