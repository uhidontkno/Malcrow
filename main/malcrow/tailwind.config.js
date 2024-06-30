/** @type {import('tailwindcss').Config} */
import catppuccin from '@catppuccin/daisyui'
export default {
  content: ["./src/**/*.{html,css,js}","./src/index.html"],
  theme: {
    fontFamily: {

      "sans": ["Inter", "sans-serif"],
      "mono": ["IBM Plex Mono", "monospace"],
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      catppuccin("mocha","mauve"),
      "black",
      "dim",
      "dark",
      catppuccin("latte","mauve"),
      "cupcake",
      "light",
      "aqua",
      
    ]
  }
}

