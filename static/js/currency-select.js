if (!window.customElements.get('currency-select')) {
    class CurrencySelect extends HTMLElement {
        static options = undefined; // Cache partagé pour toutes les instances

        connectedCallback() {
            if (!CurrencySelect.options) {
                this.loadOptions();
            } else {
                this.render();
            }
        }

        async loadOptions() {
            const response = await fetch('./currencies');
            CurrencySelect.options = await response.json();
            this.render();
        }

        render() {
            const name = this.getAttribute('data-name');
            const selected = this.getAttribute('data-selected') ?? "EUR";
            this.innerHTML = `
      <select class="currency-select" name="${name}" autocomplete="off" required>
        <button><selectedcontent></selectedcontent></button>
        ${CurrencySelect.options.map(opt =>
                `<option value="${opt.code}" ${opt.code == selected ? 'selected' : ''}>
            ${opt.name}
            ${opt.country_code ? `<img src="https://flagcdn.com/16x12/${opt.country_code}.png" width="16" height="12" alt="${opt.country}" />` : ''}
            </option>`
            ).join('\n')}
      </select>
    `;
        }
    }

    customElements.define('currency-select', CurrencySelect);
}
