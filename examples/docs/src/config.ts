// Env
export const QUERY_APP_ALLOWED_ORIGIN: string | undefined = process.env.QUERY_APP_ALLOWED_ORIGIN;
export const QUERY_APP_ENV = process.env.QUERY_APP_ENV;
export const QUERY_APP_QUERY_SERVER = process.env.QUERY_APP_QUERY_SERVER;
// Query API
export const QUERY_API_USER_TOKEN_VALUE = "/_/user/token/value";
export const QUERY_API_QUERY = "/_/query";
// Development Mode
export const IS_DEVELOPMENT = QUERY_APP_ENV === "development";
// Pages
export const HOME_PATH = "/";
// Admin Pages
export const PAGE_ADMIN_PATH = "/admin";
export const PAGE_ADMIN_LOGIN_PATH = "/admin/login";
// API
export const API_PATH = "/api";
export const API_ADMIN_LOGIN_PATH = `${API_PATH}/admin/login`;
export const API_ADMIN_LOGOUT_PATH = `${API_PATH}/admin/logout`;
export const API_QUERY_PATH = `${API_PATH}/query`;
