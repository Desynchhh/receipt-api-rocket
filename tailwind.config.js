/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./static/templates/**/*.{html,tera}"],
  theme: {
    // Allows for breakpoints
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px'
    },
    extend: {
      // Custom colors (as css)
      colors: {
        lightGray: '#313641',
        lightBlack: '#1C1E23',
        gray: '#CCCDCF'
      }
    },
  },
  plugins: [],
}
