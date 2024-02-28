/** @type {import('tailwindcss').Config} */
export default {
	content: ["./static/*.html", "./components/*.html"],
	theme: {
		extend: {},
	},
	plugins: [require("daisyui"), require("@tailwindcss/typography")],
};
