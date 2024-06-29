/** @type {import('tailwindcss').Config} */
import catppuccin from '@catppuccin/daisyui'
module.exports = {
  content: ["./src/**/*.{html,css,js}","./src/index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      catppuccin("mocha","mauve")
    ]
  }
}

