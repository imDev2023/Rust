import init, { calculate_tax } from "./pkg/tax_calculator_wasm"

async function run() {
    await init();
    function calculate_tax() {
        const income = parseFloat(document.getElementById("income").value);
        const tax = calculate_tax(income);
        document.getElementById("result").innerText = `Tax: $${tax.toFixed(2)}`;


    }
    document.getElementById("calculateButton").addEventListener(
        "click", calculateTax
    )
}
run();
