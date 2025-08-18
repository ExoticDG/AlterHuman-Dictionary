module.exports = {
    mode: "jit",
    content: {
      files: [
        "src/*.rs", 
        "src/**/*.rs", 
        "index.html"
      ],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
      extend: {
        fontFamily: {
          'sans':['"Nova Cut"'],
        },
        colors: {
          /// Main background color
          background: '#273130',
          /// Highlighted background color (used in component detail view)
          bg_highlight: '#294549',
          /// Primary border / text color
          primary: '#00ccffff',
          /// Secondary text color
          secondary: '#aaccd5ff',
          /// Button color
          button: '#008db1ff',
          /// Button hover color
          button_hover: '#00a0c9ff',
          /// Status good color
          status_good: '#66ff00ff',
          /// Status warning color
          status_warn: '#ffbd2aff',
          /// Status alert color
          status_bad: '#ff6d2aff',
          /// Status offline color
          status_offline: '#a55affff',
          /// Line chart color
          line_color: '#cbf3ffe4',
        }
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
  };