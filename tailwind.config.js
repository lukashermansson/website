/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      aria: {
        current_page: 'current="page"',
      },
    },
  },
  plugins: [],
}

