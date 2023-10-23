/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,rs}"],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui"),
  ],
  daisyui: {
    themes: [
      "dark",
      {
        light: {
          "primary": "#000000",
          "secondary": "#cfa4f9",
          "accent": "#7bce4e",
          "neutral": "#22292f",
          "base-100": "#ffffff",
          "info": "#a5d4e3",
          "success": "#1ee6cb",
          "warning": "#f4ac48",
          "error": "#fa1946",
        }
      }
    ]
  }
}

