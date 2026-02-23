const addParticipantButton = document.getElementById("add-participant");

addParticipantButton.addEventListener("click", () => {
    const input = document.createElement("input");
    input.type = "text";
    input.name = "participants[]";
    input.placeholder = "Nom";
    input.required = true;
    addParticipantButton.before(input);
});