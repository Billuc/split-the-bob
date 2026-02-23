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
    const expenseDateInput = input.parentElement.querySelector("input[name='expense_date_input']");

    if (!expenseDateInput) { continue; }
    expenseDateInput.value = toLocaleISOString(date);
}

/**
 * 
 * @param {Date} date 
 */
function toLocaleISOString(date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    return `${year}-${month}-${day}T${hours}:${minutes}`;
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