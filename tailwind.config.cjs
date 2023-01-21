/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,ts,svelte,html}"],
  theme: {
    extend: {
      keyframes: {
        bounce: {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(5px)" },
        },
        leaveLeft: {
          "0%": { transform: "translateX(0)", opacity: 1 },
          "100%": {
            transform: "translateX(-100%)",
            opacity: 0,
            display: "none",
          },
        },
        enterLeft: {
          "0%": { transform: "translateX(-100%)", opacity: 0 },
          "100%": { transform: "translateX(0)", opacity: 1, display: "flex" },
        },
        initial: {
          "0%": { opacity: 0, display: "none" },
          "100%": { opacity: 1, display: "flex" },
        },
      },
      animation: {
        initial: "initial 1s ease-in-out fowards",
        bounce: "bounce .25s ease-in-out",
        leaveLeft: "leaveLeft .75s ease-in-out forwards",
        enterLeft: "enterLeft .75s ease-in-out forwards",
      },
    },
  },
  plugins: [],
};
