const splitLink = document.getElementById("split-link");
const copyLinkButton = document.getElementById("copy-split-link");
const copyCodeButton = document.getElementById("copy-split-code");
const url = new URL(window.location.href);
url.hash = ""; // Remove any existing hash

splitLink.innerHTML = url.toString();
splitLink.href = url.toString();

copyLinkButton.addEventListener("click", (ev) => {
    navigator.clipboard
        .writeText(url.toString())
        .then(() => {
            alert("Lien copié dans le presse-papiers !");
        })
        .catch((err) => {
            console.error("Erreur lors de la copie du lien : ", err);
        });
});

copyCodeButton.addEventListener("click", (ev) => {
    const code = copyCodeButton.dataset.code;
    navigator.clipboard
        .writeText(code)
        .then(() => {
            alert("Code copié dans le presse-papiers !");
        })
        .catch((err) => {
            console.error("Erreur lors de la copie du code : ", err);
        });
});