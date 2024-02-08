const colors = require('tailwindcss/colors');

// Umwelt Technik BW
const utbw = {
  colors: {
    highlight: '#ffed00',
    black: '#000000',
    white: '#ffffff',
    gray: {
      dark: '#555555',
      lines: '#eaeaea',
      devider: '#eaeaea',
    },
    bg: {
      dark: '#dcdcdc',
      light: '#f2f2f2'
    },
  }
};

module.exports = {
  content: [
     './src/**/*.rs',
     '../crates/app-components/src/**/*.rs',
     './content/**/*.md'
  ],
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography')
  ],
  theme: {
    colors: {
      // Aliases to configure the current theme
      transparent: 'transparent',
      current: 'currentColor',
      primary: utbw.colors.highlight,
      highlight: utbw.colors.highlight,
      black: utbw.colors.black,
      white: utbw.colors.white,
      yellow: utbw.colors.highlight,
      gray: {
         50: colors.gray['50'],
        100: colors.gray['100'],
        200: colors.gray['200'],
        300: colors.gray['300'],
        400: colors.gray['400'],
        500: utbw.colors.gray.dark,
        600: colors.gray['600'],
        700: colors.gray['700'],
        800: colors.gray['800'],
        900: utbw.colors.black,
        950: utbw.colors.black,
      },
      green: colors.green,
      red: colors.red,
      orange: colors.orange,
      blue: colors.blue,
    }
  }
}
