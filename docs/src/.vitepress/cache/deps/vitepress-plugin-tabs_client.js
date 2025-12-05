import {
  reactive,
  watch
} from "./chunk-PSA4C5QZ.js";
import "./chunk-4MBMRILA.js";

// node_modules/.pnpm/vitepress-plugin-tabs@0.5.0_0f69fd453b8ff4a11841577d82a93aca/node_modules/vitepress-plugin-tabs/src/client/index.ts
import PluginTabs from "D:/OpenSource/rudis/docs/node_modules/.pnpm/vitepress-plugin-tabs@0.5.0_0f69fd453b8ff4a11841577d82a93aca/node_modules/vitepress-plugin-tabs/src/client/PluginTabs.vue";
import PluginTabsTab from "D:/OpenSource/rudis/docs/node_modules/.pnpm/vitepress-plugin-tabs@0.5.0_0f69fd453b8ff4a11841577d82a93aca/node_modules/vitepress-plugin-tabs/src/client/PluginTabsTab.vue";

// node_modules/.pnpm/vitepress-plugin-tabs@0.5.0_0f69fd453b8ff4a11841577d82a93aca/node_modules/vitepress-plugin-tabs/src/client/useTabsSelectedState.ts
var injectionKey = "vitepress:tabSharedState";
var ls = typeof localStorage !== "undefined" ? localStorage : null;
var localStorageKey = "vitepress:tabsSharedState";
var setLocalStorageValue = (v) => {
  if (!ls) return;
  ls.setItem(localStorageKey, JSON.stringify(v));
};
var provideTabsSharedState = (app) => {
  const state = reactive({});
  watch(
    () => state.content,
    (newStateContent, oldStateContent) => {
      if (newStateContent && oldStateContent) {
        setLocalStorageValue(newStateContent);
      }
    },
    { deep: true }
  );
  app.provide(injectionKey, state);
};

// node_modules/.pnpm/vitepress-plugin-tabs@0.5.0_0f69fd453b8ff4a11841577d82a93aca/node_modules/vitepress-plugin-tabs/src/client/index.ts
var enhanceAppWithTabs = (app) => {
  provideTabsSharedState(app);
  app.component("PluginTabs", PluginTabs);
  app.component("PluginTabsTab", PluginTabsTab);
};
export {
  enhanceAppWithTabs
};
//# sourceMappingURL=vitepress-plugin-tabs_client.js.map
