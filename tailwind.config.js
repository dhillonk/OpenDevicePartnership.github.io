/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  darkMode: 'media', // or 'class' if you prefer manual control
  theme: {
    extend: {
      fontFamily: {
        Geist: ['Geist', 'sans-serif', 'mono'],
      },
      // Brand surface colors. Each surface has a light + dark token so the
      // .background_* / .header_background utilities can express the dark-mode
      // pair as `bg-surface-N dark:bg-surface-N-dark`. Keeping the literal
      // pairs in style/tailwind.css would force every future palette tweak
      // to chase nine hex codes through CSS; centralizing here makes the
      // palette one edit and exposes the tokens to devtools.
      // Page-rhythm spacing tokens. Tailwind's default scale already covers the
      // 8px-multiple values we use most (8 = 32px, 10 = 40px, 20 = 80px), but
      // the 60px and 120px steps that appear repeatedly in section padding,
      // row gaps, and column gaps fall outside the default scale and were
      // previously expressed as one-off `[60px]` / `[120px]` arbitrary values.
      // Naming them `15` and `30` keeps the conventional "1 unit = 4px" reading
      // (15 * 4 = 60, 30 * 4 = 120) so call sites stay readable as plain
      // Tailwind utilities (`md:px-30`, `gap-15`) instead of bracket escapes.
      spacing: {
        '15': '60px',
        '30': '120px',
      },
      colors: {
        surface: {
          primary: '#FFFFFF',
          'primary-dark': '#171717',
          secondary: '#F1F1F1',
          'secondary-dark': '#3B3B3B',
          tertiary: '#E2E2E2',
          'tertiary-dark': '#595959',
          quaternary: '#CACACA',
          'quaternary-dark': '#797979',
          announcement: '#FEEED6',
          'announcement-dark': '#FEEED6',
        },
      },
    },
  },
  plugins: [],
}
