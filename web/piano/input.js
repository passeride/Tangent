// Source tokens keep concurrent mouse/touch/keyboard/MIDI unisons independent.
export function bindInput({
  canvas,
  player,
  getGeometry,
  audio,
  onChange,
  onError,
  toggle,
}) {
  const pressed = new Map(),
    pointers = new Map(),
    keys = new Map();
  let access = null;
  const notify = () => onChange(new Set(pressed.values()));
  function off(id) {
    audio.liveOff(id);
    pressed.delete(id);
    notify();
  }
  function on(id, key, velocity = 95) {
    if (key < 0 || key > 127) return;
    if (pressed.get(id) === key) return;
    off(id);
    pressed.set(id, key);
    audio.liveOn(id, key, velocity);
    notify();
  }
  function clear() {
    pressed.clear();
    pointers.clear();
    keys.clear();
    audio.panic();
    notify();
  }
  function hit(e) {
    const r = canvas.getBoundingClientRect(),
      g = getGeometry();
    return player.hit_key(
      (e.clientX - r.left - g.margin) / g.unit,
      (e.clientY - r.top - g.strike) / g.keyHeight,
    );
  }
  canvas.addEventListener("pointerdown", (e) => {
    e.preventDefault();
    canvas.focus();
    canvas.setPointerCapture(e.pointerId);
    pointers.set(e.pointerId, hit(e));
    const id = `pointer:${e.pointerId}`;
    audio
      .activate()
      .then(() => {
        if (pointers.has(e.pointerId)) on(id, pointers.get(e.pointerId));
      })
      .catch(onError);
  });
  canvas.addEventListener("pointermove", (e) => {
    if (!pointers.has(e.pointerId)) return;
    const key = hit(e);
    pointers.set(e.pointerId, key);
    if (key < 0) off(`pointer:${e.pointerId}`);
    else on(`pointer:${e.pointerId}`, key);
  });
  for (const event of ["pointerup", "pointercancel", "lostpointercapture"])
    canvas.addEventListener(event, (e) => {
      pointers.delete(e.pointerId);
      off(`pointer:${e.pointerId}`);
    });
  const mapping = [
    "KeyA",
    "KeyW",
    "KeyS",
    "KeyE",
    "KeyD",
    "KeyF",
    "KeyT",
    "KeyG",
    "KeyY",
    "KeyH",
    "KeyU",
    "KeyJ",
    "KeyK",
  ];
  window.addEventListener("keydown", (e) => {
    if (
      e.repeat ||
      e.ctrlKey ||
      e.metaKey ||
      e.altKey ||
      /INPUT|SELECT|BUTTON|TEXTAREA/.test(e.target.tagName)
    )
      return;
    if (e.code === "Space") {
      e.preventDefault();
      toggle();
      return;
    }
    const i = mapping.indexOf(e.code);
    if (i < 0) return;
    e.preventDefault();
    keys.set(e.code, 60 + i);
    audio
      .activate()
      .then(() => {
        if (keys.has(e.code)) on(`key:${e.code}`, keys.get(e.code));
      })
      .catch(onError);
  });
  window.addEventListener("keyup", (e) => {
    keys.delete(e.code);
    off(`key:${e.code}`);
  });
  window.addEventListener("blur", clear);
  function midiMessage(port, data) {
    const [status, key, value = 0] = data,
      channel = status & 15,
      type = status & 240,
      id = `midi:${port}:${channel}:${key}`;
    if (type === 144 && value > 0) on(id, key, value);
    else if (type === 128 || (type === 144 && value === 0)) off(id);
    else if (type === 176 && (key === 120 || key === 123))
      for (const token of [...pressed.keys()])
        if (token.startsWith(`midi:${port}:${channel}:`)) off(token);
  }
  return {
    clear,
    pressed,
    async connect() {
      await audio.activate();
      if (!navigator.requestMIDIAccess)
        throw new Error(
          "Web MIDI is unavailable here. Use touch or A–K, or a browser with Web MIDI support.",
        );
      access ??= await navigator.requestMIDIAccess({ sysex: false });
      const attach = () => {
        for (const id of [...pressed.keys()])
          if (id.startsWith("midi:")) off(id);
        for (const port of access.inputs.values())
          port.onmidimessage = (e) => midiMessage(port.id, e.data);
      };
      attach();
      access.onstatechange = attach;
      return access.inputs.size;
    },
  };
}
