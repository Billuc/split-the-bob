const expenseForms = document.querySelectorAll("article[popover] form[action$='/expenses/new#main'], article[popover] form[action$='/expenses/update#main']");

for (const form of expenseForms) {
    setupAmountSplitForm(form);
}

function setupAmountSplitForm(form) {
    const splitMethodSelect = form.querySelector("select[name='split_method']");
    const payerSelect = form.querySelector("select[name='payed_by']");
    const totalAmountInput = form.querySelector("input[name='amount']");
    const participantCheckboxes = form.querySelectorAll("input[name='payed_for[]']");
    const amountRows = form.querySelectorAll(".participant-amount-row");

    if (!splitMethodSelect || !payerSelect || !totalAmountInput || amountRows.length === 0) {
        return;
    }

    const updateAmountRows = () => {
        const usesAmounts = splitMethodSelect.value === "Amounts";

        const selectedParticipants = new Set(
            Array.from(participantCheckboxes)
                .filter(checkbox => checkbox.checked)
                .map(checkbox => checkbox.value)
        );

        for (const row of amountRows) {
            const participant = row.getAttribute("data-participant");
            const personInput = row.querySelector(".amount-person-input");
            const amountInput = row.querySelector(".participant-amount-input");
            const isSelected = participant && selectedParticipants.has(participant);

            if (!personInput || !amountInput) {
                continue;
            }

            const isEnabled = usesAmounts && isSelected;
            personInput.disabled = !isEnabled;
            amountInput.disabled = !isEnabled;

            if (!isEnabled) {
                amountInput.value = "0";
            }
        }

        updatePayerAmount();
    };

    const updatePayerAmount = () => {
        if (splitMethodSelect.value !== "Amounts") {
            setPayerReadOnly(false);
            return;
        }

        const payer = payerSelect.value;
        const payerRow = findAmountRow(amountRows, payer);
        if (!payerRow) {
            setPayerReadOnly(false);
            return;
        }

        const payerAmountInput = payerRow.querySelector(".participant-amount-input");
        if (!payerAmountInput || payerAmountInput.disabled) {
            setPayerReadOnly(false);
            return;
        }

        const total = parseFloat(totalAmountInput.value) || 0;
        let totalOthers = 0;

        for (const row of amountRows) {
            const participant = row.getAttribute("data-participant");
            if (participant === payer) {
                continue;
            }

            const amountInput = row.querySelector(".participant-amount-input");
            if (!amountInput || amountInput.disabled) {
                continue;
            }

            totalOthers += parseFloat(amountInput.value) || 0;
        }

        payerAmountInput.value = (total - totalOthers).toFixed(2);
        setPayerReadOnly(true);
    };

    const setPayerReadOnly = (readOnly) => {
        const payer = payerSelect.value;

        for (const row of amountRows) {
            const participant = row.getAttribute("data-participant");
            const amountInput = row.querySelector(".participant-amount-input");
            if (!amountInput) {
                continue;
            }

            amountInput.readOnly = readOnly && participant === payer;
        }
    };

    const amountInputHandler = (event) => {
        if (splitMethodSelect.value !== "Amounts") {
            return;
        }

        const row = event.target.closest(".participant-amount-row");
        const participant = row ? row.getAttribute("data-participant") : "";
        if (participant === payerSelect.value) {
            return;
        }

        updatePayerAmount();
    };

    splitMethodSelect.addEventListener("change", updateAmountRows);
    payerSelect.addEventListener("change", () => {
        setPayerReadOnly(false);
        updatePayerAmount();
    });
    totalAmountInput.addEventListener("input", updatePayerAmount);

    for (const checkbox of participantCheckboxes) {
        checkbox.addEventListener("change", updateAmountRows);
    }

    for (const row of amountRows) {
        const amountInput = row.querySelector(".participant-amount-input");
        if (!amountInput) {
            continue;
        }

        amountInput.addEventListener("input", amountInputHandler);
    }

    updateAmountRows();
}

function findAmountRow(amountRows, participant) {
    for (const row of amountRows) {
        if (row.getAttribute("data-participant") === participant) {
            return row;
        }
    }

    return null;
}
