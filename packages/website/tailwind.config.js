const colors = require("tailwindcss/colors");
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  darkMode: "class",
  content: [
    "./packages/website/templates/**/*.html",
    "./packages/website/theme/**/*.html"
  ],
  theme: {
    colors: {
        inherit: colors.inherit,
        current: colors.current,
        transparent: colors.transparent,

        accent: getColor("accent"),
        text: getColor("text"),
        background: getColor("background"),

        neutral: getShades("neutral"),
    },
    fontFamily: {
      sans: ["Inter", ...defaultTheme.fontFamily.sans],
      mono: defaultTheme.fontFamily.mono,
    },
    screens: {
      "md": "768px",
      "lg": "1024px",
    }
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
};

function getColor(name) {
  return `rgb(var(--color-${name}) / <alpha-value>)`
}

function getShades(name) {
  const shades = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
  return shades.reduce((prev, key) => ({
    ...prev,
    [key]: getColor(`${name}-${key}`),
  }), {});
}
