const SPLITS_KEY = "splits";
const SEPARATOR = ",";
const JOINED_SPLITS_DIV_ID = "joined-splits";

function saveSplit() {
    const splitId = new URLSearchParams(window.location.search).get("split_id");
    if (!splitId) return;

    const idsString = localStorage.getItem(SPLITS_KEY) ?? "";
    const ids = new Set(idsString.split(SEPARATOR).filter(s => s != ""));
    ids.add(splitId);
    let newIds = Array.from(ids).join(SEPARATOR)
    localStorage.setItem(SPLITS_KEY, newIds);
}

async function loadSplits() {
    const idsString = localStorage.getItem(SPLITS_KEY) ?? "";
    const ids = new Set(idsString.split(SEPARATOR).filter(s => s != ""));
    const joinedSplitsDiv = document.getElementById(JOINED_SPLITS_DIV_ID);
    if (!joinedSplitsDiv) return;
    const parser = new DOMParser();

    for (const id of ids) {
        const url = new URL("./splits/details", window.location.href);
        url.hash = id;
        url.searchParams.append("split_id", id);

        fetch(url)
            .then(res => res.text())
            .then(txt => {
                const html = parser.parseFromString(txt, "text/html");
                const details = html.getElementById(id);
                joinedSplitsDiv.append(details);
            })
    }
}

if (window.location.pathname === "/splits") {
    saveSplit();
} else if (window.location.pathname === "/") {
    loadSplits();
}