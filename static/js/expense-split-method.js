/**
 * @type {NodeListOf<HTMLFormElement>}
 */
const expenseForms = document.querySelectorAll("form.expense-form");

for (const form of expenseForms) {
    setupAmountSplitForm(form);
}

/**
 * 
 * @param {HTMLFormElement} form 
 * @returns 
 */
function setupAmountSplitForm(form) {
    const splitMethodSelect = form.querySelector("select[name='split_method']");
    const totalAmountInput = form.querySelector("input[name='amount']");
    const amountRows = form.querySelectorAll(".participant-amount-row");
    const participantCheckboxes = form.querySelectorAll("input[name='payed_for[]']");

    if (!splitMethodSelect || !totalAmountInput || amountRows.length === 0) {
        return;
    }

    const updateAmountRows = () => {
        const nbDebtors = Array.from(participantCheckboxes)
            .filter(checkbox => checkbox?.checked)
            .length;
        const totalAmount = parseFloat(totalAmountInput.value) || 0;
        let totalIndividualAmounts = 0;

        for (const row of amountRows) {
            const participantCheckbox = row.querySelector("input[name='payed_for[]']");
            const amountInput = row.querySelector("input[name='amounts_value[]']");

            if (!participantCheckbox || !amountInput) {
                continue;
            }

            switch (splitMethodSelect.value) {
                case "Evenly":
                    amountInput.disabled = true;
                    amountInput.value = participantCheckbox.checked ?
                        (totalAmount / nbDebtors).toFixed(2) :
                        "0";
                    break;
                case "Amounts":
                    if (!participantCheckbox.checked) {
                        amountInput.disabled = true;
                        amountInput.value = "0";
                    } else {
                        amountInput.disabled = false;
                        totalIndividualAmounts += parseFloat(amountInput.value) || 0;
                    }
                    break;
            }
        }

        const amountDifference = totalAmount - totalIndividualAmounts;

        if (splitMethodSelect.value === "Amounts" && Math.abs(amountDifference) >= 0.005) {
            const message = `La somme des montants individuels (${totalIndividualAmounts.toFixed(2)}) n'est pas égale au montant total (${totalAmount.toFixed(2)})`;
            totalAmountInput.setCustomValidity(message);
        } else {
            totalAmountInput.setCustomValidity("");
        }
    };

    splitMethodSelect.addEventListener("change", updateAmountRows);
    totalAmountInput.addEventListener("input", updateAmountRows);
    participantCheckboxes.forEach(checkbox => checkbox.addEventListener("change", updateAmountRows));
    amountRows.forEach(row => row.querySelector("input[name='amounts_value[]']")?.addEventListener("input", updateAmountRows));

    updateAmountRows();
}
