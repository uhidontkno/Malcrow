/** @type {import('tailwindcss').Config} */
import catppuccin from '@catppuccin/daisyui'
export default {
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

