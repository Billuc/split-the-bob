const expenseDates = document.querySelectorAll(".expense-date-display");
for (const span of expenseDates) {
    const timestamp = parseFloat(span.getAttribute("data-timestamp"));
    if (!isNaN(timestamp)) {
        const date = new Date(timestamp * 1000);
        span.textContent = date.toLocaleString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }
}

const expenseDateValues = document.querySelectorAll("input[name='expense_date']");
for (const input of expenseDateValues) {
    const timestamp = parseFloat(input.value);

    if (isNaN(timestamp)) { continue; }

    const date = new Date(timestamp * 1000);
    const datetimeLocalValue = date.toLocaleDateString("en-US") + "T" + date.toLocaleTimeString("en-US", { hour12: false });
    const expenseDateInput = input.parentElement.querySelector("input[name='expense_date_input']");

    if (!expenseDateInput) { continue; }
    expenseDateInput.value = datetimeLocalValue;
}

document
    .querySelectorAll("input[name='expense_date_input']")
    .forEach(input => {
        input.addEventListener("change", function (e) {
            const dateValue = e.target.value;
            if (!dateValue) { return; }
            // Convert datetime-local (YYYY-MM-DDTHH:mm) to Unix timestamp in seconds
            const timestamp = Date.parse(dateValue) / 1000;
            const valueInput = input.parentElement.querySelector("input[name='expense_date']")

            if (!valueInput) { return; }
            valueInput.value = timestamp;
        });
    });