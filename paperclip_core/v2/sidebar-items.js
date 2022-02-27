initSidebarItems({"attr":[["api_v2_errors","Marker attribute for indicating that the marked object can represent non-2xx (error) status codes with optional descriptions."],["api_v2_errors_overlay","Marker attribute for indicating that the marked object can filter error responses from the the `#[api_v2_errors]` macro."],["api_v2_operation","Marker attribute for indicating that a function is an OpenAPI v2 compatible operation."],["api_v2_schema_struct","Converts your struct to support deserializing from an OpenAPI v2 Schema object (example). This adds the necessary fields (in addition to your own fields) and implements the `Schema` trait for parsing and codegen."],["delete","Creates route handler with `paperclip::actix::web::Resource`. In order to control the output type and status codes the return value/response must implement the trait actix_web::Responder."],["get","Creates route handler with `paperclip::actix::web::Resource`. In order to control the output type and status codes the return value/response must implement the trait actix_web::Responder."],["post","Creates route handler with `paperclip::actix::web::Resource`. In order to control the output type and status codes the return value/response must implement the trait actix_web::Responder."],["put","Creates route handler with `paperclip::actix::web::Resource`. In order to control the output type and status codes the return value/response must implement the trait actix_web::Responder."]],"derive":[["Apiv2Schema","Derive attribute for indicating that a type is an OpenAPI v2 compatible definition."],["Apiv2Security","Marker attribute for indicating that an object forbids public access to operation (for example AccessToken)."]],"mod":[["models","Models used by OpenAPI v2."],["schema","Traits used for code and spec generation."]],"struct":[["AcceptedJson",""],["CreatedJson",""],["NoContent",""],["ResponderWrapper","Wrapper for wrapping over `impl Responder` thingies (to avoid breakage)."],["ResponseWrapper","Wrapper for all response types from handlers. This holds the actual value returned by the handler and a unit struct (autogenerated by the plugin) which is used for generating operation information."]],"trait":[["OperationModifier","Actix-specific trait for indicating that this entity can modify an operation and/or update the global map of definitions."]]});