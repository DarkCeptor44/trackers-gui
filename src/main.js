const { invoke } = window.__TAURI__.tauri;

let trackersEl;
let btnBest, btnAll, btnBestIp, btnAllIp;

/**
 * @param {string} list 
 */
async function getTrackers(list) {
    let trackers = await invoke("get_trackers", { list: list });

    if (typeof (trackers) !== "string" || trackers.trim() === "") {
        trackersEl.value = "Something went wrong";
        return;
    }

    trackersEl.value = trackers;
}

window.addEventListener("DOMContentLoaded", () => {
    trackersEl = document.querySelector("#trackers-field");
    btnBest = document.querySelector("#btnBest");
    btnAll = document.querySelector("#btnAll");
    btnBestIp = document.querySelector("#btnBestIp");
    btnAllIp = document.querySelector("#btnAllIp");

    btnBest.addEventListener("click", () => getTrackers("best"));
    btnAll.addEventListener("click", () => getTrackers("all"));
    btnBestIp.addEventListener("click", () => getTrackers("best_ip"));
    btnAllIp.addEventListener("click", () => getTrackers("all_ip"));
});