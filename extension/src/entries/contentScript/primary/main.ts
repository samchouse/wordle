// import browser from "webextension-polyfill";

/**
<input class="MuiBox-root css-1kzumi4" value="" disabled="">
<div class="MuiBox-root css-1vp8jwk">
  <div class="MuiBox-root css-zde0os">q</div>
  <div class="MuiBox-root css-4ilc5x">w</div>
  <div class="MuiBox-root css-zde0os">e</div>
  <div class="MuiBox-root css-4ilc5x">r</div>
  <div class="MuiBox-root css-4ilc5x">t</div>
  <div class="MuiBox-root css-zde0os">y</div>
  <div class="MuiBox-root css-zde0os">u</div>
  <div class="MuiBox-root css-zde0os">i</div>
  <div class="MuiBox-root css-nfcdu0">o</div>
  <div class="MuiBox-root css-zde0os">p</div>
</div>
<div class="MuiBox-root css-1vp8jwk">
  <div class="MuiBox-root css-1a2dfdl">a</div>
  <div class="MuiBox-root css-zde0os">s</div>
  <div class="MuiBox-root css-4ilc5x">d</div>
  <div class="MuiBox-root css-4ilc5x">f</div>
  <div class="MuiBox-root css-zde0os">g</div>
  <div class="MuiBox-root css-zde0os">h</div>
  <div class="MuiBox-root css-nfcdu0">j</div>
  <div class="MuiBox-root css-zde0os">k</div>
  <div class="MuiBox-root css-4ilc5x">l</div>
  <div class="MuiBox-root css-zde0os"><svg class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium css-vubbuv"
      focusable="false" aria-hidden="true" viewBox="0 0 24 24" data-testid="BackspaceIcon">
      <path
        d="M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-3 12.59L17.59 17 14 13.41 10.41 17 9 15.59 12.59 12 9 8.41 10.41 7 14 10.59 17.59 7 19 8.41 15.41 12 19 15.59z">
      </path>
    </svg></div>
</div>
<div class="MuiBox-root css-1vp8jwk">
  <div class="MuiBox-root css-4ilc5x">z</div>
  <div class="MuiBox-root css-zde0os">x</div>
  <div class="MuiBox-root css-zde0os">c</div>
  <div class="MuiBox-root css-zde0os">v</div>
  <div class="MuiBox-root css-zde0os">b</div>
  <div class="MuiBox-root css-zde0os">n</div>
  <div class="MuiBox-root css-zde0os">m</div>
  <div class="MuiBox-root css-1b7d4qo"><svg class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium css-vubbuv"
      focusable="false" aria-hidden="true" viewBox="0 0 24 24" data-testid="KeyboardReturnIcon">
      <path d="M19 7v4H5.83l3.58-3.59L8 6l-6 6 6 6 1.41-1.41L5.83 13H21V7z"></path>
    </svg></div>
</div>
*/

const waitForElm = (selector: string): Promise<Element | null> => {
  return new Promise((resolve) => {
    if (document.querySelector(selector)) {
      return resolve(document.querySelector(selector));
    }

    const observer = new MutationObserver((_) => {
      if (document.querySelector(selector)) {
        resolve(document.querySelector(selector));
        observer.disconnect();
      }
    });

    observer.observe(document.body, {
      childList: true,
      subtree: true,
    });
  });
};

const run = async () => {
  const input = await waitForElm("input");
  const children = input?.parentElement?.children;
  input?.addEventListener("submit", () => {
    for (const idx of [...Array(children?.length).map((_, i) => i)]) {
      const child = children?.item(idx);
      for (const idx of [...Array(child?.children?.length).map((_, i) => i)]) {
        const child = children?.item(idx);
        if (child?.children.namedItem("svg")) continue;
        console.log(child?.innerHTML);
      }
    }
  });
};

run();
