/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/*.html", "./components/*.html", "./components/**/*.html"],
	// theme: {
	// 	extend: {
	// 		fontFamily: {
	// 			sans: ["Poppins"],
	// 		},
	// 	},
	// },
	plugins: [require("daisyui"), require("@tailwindcss/typography")],
};
