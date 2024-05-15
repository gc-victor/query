import { API_PATH } from "./shared.constants";

// Database
export const {{ tableConstantCase }}_DATABASE = "{{ database }}";
// Admin Pages
export const PAGE_ADMIN_{{ tableConstantCase }}_PATH = "/admin/{{ tableLowerCase }}";
// Public Pages
export const PAGE_{{ tableConstantCase }}_PATH = "/{{ tableLowerCase }}";
// API
export const API_ADMIN_{{ tableConstantCase }}_PATH = `${API_PATH}/admin/{{ tableLowerCase }}`;
