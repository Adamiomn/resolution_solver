import init, { try_parse_input, calculate_resolution } from './pkg/resolution_solver.js';

init().then(() => {
	const inputStringElement = document.getElementById("input-string");
	const inputValidationElement = document.getElementById("input-validation");
	const calculateButton = document.getElementById("calculate-btn");
	const resultElement = document.getElementById("result");

	handleInputChange();

	function adjustInputBoxSize() {
		inputStringElement.style.height = "auto";
		inputStringElement.style.height = inputStringElement.scrollHeight + "px";
	}

	function handleInputChange() {
		const input = inputStringElement.value;
		const result = try_parse_input(input);
		inputValidationElement.innerHTML = `${result}`;
		adjustInputBoxSize();
	}

	inputStringElement.addEventListener("input", handleInputChange);
	addEventListener("resize", adjustInputBoxSize);

	calculateButton.addEventListener("click", () => {
		const input = inputStringElement.value;
		if (try_parse_input(input) == "✅") {
			const resolution = calculate_resolution(input);
			resultElement.innerHTML = `${resolution}`;
		} else {
			resultElement.innerHTML = "Invalid input. Please correct your input and try again.";
		}
	});
});