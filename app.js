import { validate_input, calculate_resolution } from './pkg/resolution_solver.js';

const inputStringElement = document.getElementById("input-string");
const inputValidationElement = document.getElementById("input-validation");
const calculateButton = document.getElementById("calculate-btn");
const resultElement = document.getElementById("result");

inputStringElement.addEventListener("input", () => {
	const input = inputStringElement.value;
	const isValid = validate_input(input);
	inputValidationElement.style.visibility = isValid ? "hidden" : "visible";
});

calculateButton.addEventListener("click", () => {
	const input = inputStringElement.value;
	if (validate_input(input)) {
		const resolution = calculate_resolution(input);
		resultElement.textContent = `The resolution is: ${resolution}`;
	} else {
		resultElement.textContent = "Invalid input. Please correct your input and try again.";
	}
});
