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
        contentbd: colors.gray[200],
        barbg: colors.white,
        barfg: colors.gray[200],
        barborder: colors.gray[200],
        highbg: colors.violet[500],
        highfg: colors.violet[100],
        highborder: colors.violet[600],
        highbghov: colors.violet[600],
        highbgact: colors.violet[400],
        highfgact: colors.violet[50],

        dark_caption: colors.gray[50],
        dark_input: colors.gray[50],
        dark_inputbg: colors.gray[500],
        dark_stext: colors.gray[50],
        dark_gtext: colors.gray[100],
        dark_icon: colors.white,
        dark_contentbg: colors.gray[900],
        dark_contentbd: colors.gray[950],
        dark_barbg: colors.gray[900],
        dark_barfg: colors.gray[100],
        dark_barborder: colors.gray[950],

        ccuser: colors.yellow[500],
        ccuserbd: colors.yellow[600],
        ccacctype: colors.orange[500],
        ccacctypebd: colors.orange[600],
        ccpass: colors.red[600],
        ccpassbd: colors.red[700],
        ccemail: colors.cyan[600],
        ccemailbd: colors.cyan[700],
        cctime: colors.blue[600],
        cctimebd: colors.blue[700],
        ccprog: colors.lime[500],
        ccprogbd: colors.lime[600],
        cctable: colors.emerald[500],
        cctablebd: colors.emerald[600],

        errorfg: colors.red[500],
        errorbg: colors.red[300],
        errorbd: colors.red[600],
        eiconbg: colors.red[400],

        successfg: colors.green[500],
        successbg: colors.green[300],
        successbd: colors.green[600],
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

