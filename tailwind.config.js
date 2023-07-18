/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')
const plugin = require('tailwindcss/plugin')

module.exports = {
  darkMode: 'class',
  content: ["templates/**/*.html.hbs"],
  theme: {
    fontFamily: {
      'sans': ['"Nunito"'],
    },
    extend: {
      colors: {
        maincap: colors.teal[500],
        caption: colors.gray[500],
        input: colors.gray[600],
        inputbg: colors.white,
        stext: colors.gray[600],
        gtext: colors.gray[400],
        icon: colors.white,
        contentbg: colors.white,
        barbg: colors.white,
        barfg: colors.gray[200],
        barborder: colors.gray[200],
        highbg: colors.violet[500],
        highfg: colors.violet[100],
        highborder: colors.violet[600],
        highbghov: colors.violet[600],
        highbgact: colors.violet[400],
        highfgact: colors.violet[50],

        ccuser: colors.yellow[500],
        ccacctype: colors.orange[500],
        ccpass: colors.red[600],
        ccemail: colors.cyan[600],
        cctime: colors.blue[600],
        ccprog: colors.lime[500],
        cctable: colors.emerald[500],

        errorfg: colors.red[500],
        errorbg: colors.red[300],
        eiconbg: colors.red[400],

        successfg: colors.green[500],
        successbg: colors.green[300],
        siconbg: colors.green[400],

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
    }),
    require("@tailwindcss/forms")
  ],
}

