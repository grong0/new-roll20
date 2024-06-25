/** @type {import('tailwindcss').Config} */
export default {
	content: ["./static/*.html", "./components/*.html"],
	theme: {
		extend: {
			fontFamily: {
				sans: ["Poppins"],
			},
		},
	},
	plugins: [require("daisyui"), require("@tailwindcss/typography")],
};
