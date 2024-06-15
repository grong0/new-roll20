/** @type {import('tailwindcss').Config} */
export default {
	content: ["./static/*.html", "./components/*.html"],
	theme: {
		extend: {},
	},
	plugins: [require("daisyui"), require("@tailwindcss/typography")],
    safelist: [
        "bg-sky-500",
        "bg-red-500",
        "bg-amber-500",
        "bg-rose-500",
        "bg-yellow-500",
        "bg-lime-500",
        "bg-green-500",
        "bg-orange-500",
        "bg-violet-500",
        "bg-emerald-500",
        "bg-fuchsia-500",
        "bg-blue-500",
        "bg-purple-500",
        "bg-indigo-500",
        "bg-primary-500"
    ]
};
