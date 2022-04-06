import browser from "webextension-polyfill";

const saveOpts = (e: SubmitEvent) => {
  console.log('a')
  e.preventDefault();
  browser.storage.sync.set({
    enabled: (document.getElementById("enabled") as HTMLInputElement)?.value,
  });
}

const restoreOptions = () => {
  const setCurrentChoice = (result: Record<string, any>) => {
    if (document.getElementById("enabled") as HTMLInputElement) {
      (document.getElementById("enabled") as HTMLInputElement).value = result.enabled;
    }
  }

  let getting = browser.storage.sync.get("enabled");
  // @ts-ignore
  getting.then((res) => console.log(res) || setCurrentChoice(res), console.log);
}

document.addEventListener("DOMContentLoaded", restoreOptions);
document.getElementById("form")?.addEventListener("submit", saveOpts);
