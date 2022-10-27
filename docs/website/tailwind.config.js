const colors = require("tailwindcss/colors");
const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  darkMode: "class",
  content: [
    "./templates/**/*.html",
    "./theme/**/*.html"
  ],
  theme: {
    colors: {
        inherit: colors.inherit,
        current: colors.current,
        transparent: colors.transparent,

        text: getColor("text"),
        background: getColor("background"),

        accent: getColorVariant("accent", ["DEFAULT", "background"]),
        neutral: getColorVariant("neutral", [50, 100, 200, 300, 400, 500, 600, 700, 800, 900]),
    },
    fontFamily: {
      sans: ["Inter", ...defaultTheme.fontFamily.sans],
      mono: defaultTheme.fontFamily.mono,
    },
    screens: {
      md: "768px",
      lg: "1024px",
      xl: "1280px"
    },
    extend: {
      maxWidth: theme => ({
        "screen-xl": theme("screens.xl"),
      })
    }
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
};

function getColor(name) {
  return `rgb(var(--color-${name}) / <alpha-value>)`
}

function getColorVariant(name, keys) {
  return keys.reduce((prev, key) => ({
    ...prev,
    [key]: getColor(key === "DEFAULT" ? name : `${name}-${key}`),
  }), {});
}
