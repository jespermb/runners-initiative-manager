/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}',
    './src/routes/**/*.{svelte,js,ts,jsx,tsx}',
    './src/lib/**/*.{svelte,js,ts,jsx,tsx}'
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      "night"
    ]
  }
}
