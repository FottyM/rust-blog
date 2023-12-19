/** @type {import('tailwindcss').Config} */

// require('./src/views/layout')
module.exports = {
  content: ['./src/views/**/*.{html,rs}'],
  theme: {
    extend: {
      colors: {
        teal: '#008080', // Custom teal color
        'soul-red': '#9F1F1F', // Custom Mazda Soul Red Crystal color
        // Define additional colors for your dark theme
        dark: '#121212', // Very dark background color
        gray: {
          darkest: '#1f2d3d',
          dark: '#3c4858',
          DEFAULT: '#c0ccda',
          light: '#e0e6ed',
          lightest: '#f9fafc',
        },
        primary: '#5a67d8', // Example primary color
        secondary: '#ed64a6', // Example secondary color
        accent: '#38b2ac', // Example accent color
      },
      backgroundColor: {
        page: '#0f172a', // Dark shade for the page background
        card: '#1e293b', // Dark shade for card backgrounds
      },
      borderColor: {
        DEFAULT: '#334155', // Border color for elements
      },
      textColor: {
        title: '#ffffff', // White text for titles
        body: '#d1d5db', // Light gray for body text
      },
    },
  },
  plugins: [],
}