import init, { find_blocks } from "./pkg/bkg_finder.js";

const form = document.querySelector("#search-form");
const measurement = document.querySelector("#measurement");
const maxCombinations = document.querySelector("#max-combinations");
const exclusions = document.querySelector("#exclusions");
const status = document.querySelector("#status");
const resultsList = document.querySelector("#results-list");
const installButton = document.querySelector("#install-button");

let wasmReady = false;
let installPrompt = null;

function formatNumber(value) {
  return Number(value).toFixed(4);
}

function renderResults(result) {
  resultsList.replaceChildren();

  if (result.combinations.length === 0) {
    status.value = "No combinations";
    return;
  }

  status.value = `${result.combinations.length} combination${
    result.combinations.length === 1 ? "" : "s"
  }`;

  result.combinations.forEach((combination, index) => {
    const article = document.createElement("article");
    article.className = "result-card";

    const title = document.createElement("h3");
    title.textContent = `Combination ${index + 1}`;

    const meta = document.createElement("p");
    meta.className = "meta";
    meta.textContent = `Sum ${formatNumber(combination.sum)} | Delta ${formatNumber(
      combination.delta,
    )} | ${combination.error}`;

    const blocks = document.createElement("div");
    blocks.className = "blocks";

    combination.blocks.forEach((block) => {
      const chip = document.createElement("span");
      chip.textContent = formatNumber(block);
      blocks.append(chip);
    });

    article.append(title, meta, blocks);
    resultsList.append(article);
  });
}

async function runSearch() {
  if (!wasmReady) {
    status.value = "Loading";
    return;
  }

  try {
    const result = find_blocks(
      Number(measurement.value),
      Number(maxCombinations.value),
      exclusions.value,
    );
    renderResults(result);
  } catch (error) {
    status.value = error instanceof Error ? error.message : String(error);
    resultsList.replaceChildren();
  }
}

form.addEventListener("submit", (event) => {
  event.preventDefault();
  runSearch();
});

window.addEventListener("beforeinstallprompt", (event) => {
  event.preventDefault();
  installPrompt = event;
  installButton.hidden = false;
});

installButton.addEventListener("click", async () => {
  if (!installPrompt) {
    return;
  }

  installPrompt.prompt();
  await installPrompt.userChoice;
  installPrompt = null;
  installButton.hidden = true;
});

if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => {
    navigator.serviceWorker.register("./sw.js");
  });
}

status.value = "Loading";
await init();
wasmReady = true;
status.value = "Ready";
runSearch();
