/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')
const plugin = require('tailwindcss/plugin')

module.exports = {
  darkMode: 'class',
  content: ["templates/**/*.html.hbs"],
  theme: {
    extend: {
      colors: {
        maincap: colors.violet[500],
        caption: colors.gray[500],
        input: colors.gray[600],
        inputbg: colors.white,
        stext: colors.gray[600],
        icon: colors.white,
        iconbg: colors.gray[300],
        contentbg: colors.gray[100],
        contentdiv: colors.gray[200],
        barbg: colors.gray[200],
        barfg: colors.gray[400],
        highbg: colors.violet[500],
        highfg: colors.violet[100],
        highbghov: colors.violet[600],
        highbgact: colors.violet[400],
        highfgact: colors.violet[50],

        ccuser: colors.yellow[500],
        ccpass: colors.red[600],

        errorfg: colors.red[500],
        errorbg: colors.red[300],
        eiconbg: colors.red[400],

        cccolq: colors.emerald,
        ccexam: colors.amber,
        cchomw: colors.sky,
      }
    },
  },
  plugins: [
    plugin(function({ addVariant }) {
      addVariant('progress', ['&::-moz-progress-bar', '&::-webkit-progress-value'])
      addVariant('progress-bg', ['&::-webkit-progress-bar'])
    })
  ],
}

