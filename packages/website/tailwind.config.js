/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./packages/website/templates/**/*.html",
    "./packages/website/theme/**/*.html"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography')
  ],
}
