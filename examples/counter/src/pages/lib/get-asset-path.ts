import { assetPath } from "@/pages/lib/asset-path";

// Legacy getAssetPath function, now forwarding to assetPath
export function getAssetPath(fileName: string) {
    console.warn('getAssetPath() is deprecated, use assetPath() instead');
    return assetPath(fileName);
}