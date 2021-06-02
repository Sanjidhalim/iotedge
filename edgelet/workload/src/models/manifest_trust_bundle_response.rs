/*
 * IoT Edge Module Workload API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2020-10-10
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestTrustBundleResponse {
    /// Base64 encoded PEM formatted byte array containing the manifest trusted root certificate authority.
    #[serde(rename = "certificate")]
    certificate: String,
}

impl ManifestTrustBundleResponse {
    pub fn new(certificate: String) -> ManifestTrustBundleResponse {
        ManifestTrustBundleResponse { certificate }
    }

    pub fn set_certificate(&mut self, certificate: String) {
        self.certificate = certificate;
    }

    pub fn with_certificate(mut self, certificate: String) -> ManifestTrustBundleResponse {
        self.certificate = certificate;
        self
    }

    pub fn certificate(&self) -> &String {
        &self.certificate
    }
}
