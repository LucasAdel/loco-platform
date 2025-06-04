/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./assets/**/*.html",
    "../shared/src/**/*.rs",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        'aussie-blue': '#0066cc',
        'aussie-green': '#00a86b',
        'aussie-gold': '#ffcd00',
        'outback-red': '#a8312f',
        'ocean-blue': '#006994',
      },
      fontFamily: {
        'sans': ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
        'mono': ['SF Mono', 'Monaco', 'monospace'],
      },
      animation: {
        'shimmer': 'shimmer 2s infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        shimmer: {
          '0%': { 'background-position': '-200% 0' },
          '100%': { 'background-position': '200% 0' },
        },
        glow: {
          '0%': { 'box-shadow': '0 0 20px rgba(59, 130, 246, 0.4)' },
          '100%': { 'box-shadow': '0 0 40px rgba(59, 130, 246, 0.8)' },
        },
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(circle, var(--tw-gradient-stops))',
      },
    },
  },
  plugins: [],
}