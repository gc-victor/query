import { assetPath } from "@/pages/lib/asset-path";

export const Scripts = () => (
    <>
        <script src={assetPath("dist/dark-light.island.js")} />
        <script type="module" src={assetPath("dist/search-modal.island.js")} />
        <script type="module" src={assetPath("dist/highlighter.island.js")} />
    </>
);
