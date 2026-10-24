// Presentation profiles only: changing mood never changes a song or its timing.
export const themes = {
  aurora: {
    bg: "#0a151e",
    horizon: "#17404a",
    colors: ["#8ce0c1", "#a8b9f4", "#e1c391", "#8dcae9"],
    white: "#e5e9e4",
    black: "#17252e",
    ink: "#dcece6",
    staff: "#99bbc047",
    glow: "#8ce0c1",
    page: "#11242d",
  },
  ember: {
    bg: "#1b141b",
    horizon: "#563039",
    colors: ["#efaf82", "#d6b2e8", "#e7ce8d", "#e89caa"],
    white: "#f1e8df",
    black: "#30232e",
    ink: "#eee0da",
    staff: "#c4a6bb44",
    glow: "#efaf82",
    page: "#2b2029",
  },
  paper: {
    bg: "#e0e5df",
    horizon: "#b9cebd",
    colors: ["#407e74", "#796aaf", "#ac723a", "#477eaa"],
    white: "#faf8ef",
    black: "#283e3b",
    ink: "#2f4641",
    staff: "#3e605a50",
    glow: "#407e74",
    page: "#f3f2e8",
  },
};
export const instruments = {
  felt: {
    harmonics: [1, 0.28, 0.12, 0.04],
    decay: 1.7,
    release: 0.24,
    brightness: 2600,
  },
  bright: {
    harmonics: [1, 0.55, 0.25, 0.13, 0.07],
    decay: 2.8,
    release: 0.4,
    brightness: 6500,
  },
};
