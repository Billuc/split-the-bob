function htmz(frame) {
  if (frame.contentWindow.location.hash === "") {
    return;
  }

  const newUrl = frame.contentWindow.location.href;
  newUrl.hash = "";
  window.history.pushState(null, "", newUrl);

  setTimeout(() =>
    document
      .querySelector(frame.contentWindow.location.hash || null)
      ?.replaceChildren(
        ...frame.contentDocument.querySelector(
          frame.contentWindow.location.hash || null,
        ).childNodes,
      ),
  );
}

const htmzEl = document.createElement("iframe");
htmzEl.setAttribute("hidden", "true");
htmzEl.setAttribute("name", "htmz");
htmzEl.addEventListener("load", () => htmz(htmzEl));

document.body.appendChild(htmzEl);
