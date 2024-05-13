// Env
export const QUERY_APP_ALLOWED_ORIGIN: string | undefined = process.env.QUERY_APP_ALLOWED_ORIGIN;
export const QUERY_APP_ENV = process.env.QUERY_APP_ENV;
export const QUERY_APP_QUERY_SERVER = process.env.QUERY_APP_QUERY_SERVER;
// Query API
export const QUERY_API_USER_TOKEN_VALUE = "/_/user/token/value";
export const QUERY_API_ASSET_BUILDER = "/_/asset-builder";
export const QUERY_API_QUERY = "/_/query";
// IS
export const IS_DEVELOPMENT = QUERY_APP_ENV === "development";
