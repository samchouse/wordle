export default {
  content_scripts: [
    {
      js: ["src/entries/contentScript/primary/main.ts"],
      matches: ["*://*/*"],
    },
  ],
  browser_action: {
    default_icon: {
      48: "icons/48.png",
    },
    default_popup: "src/entries/popup/index.html",
  },
  icons: {
    48: "icons/48.png",
  },
  manifest_version: 2,
  permissions: ["storage", "*://*.squabble.me/*"],
  browser_specific_settings: {
    gecko: {
      id: "squabble@addons.com",
    },
  },
};
