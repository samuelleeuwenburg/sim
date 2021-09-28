const queueBuffer = (ctx, size, when, samples) => {
  const buffer = ctx.createBuffer(2, 48000 * size, 48000);

  for (var channel = 0; channel < buffer.numberOfChannels; channel++) {
    var nowBuffering = buffer.getChannelData(channel);
    for (var i = 0; i < buffer.length; i++) {
      nowBuffering[i] = samples[i * 2 + channel];
    }
  }

  const source = ctx.createBufferSource();
  source.buffer = buffer;
  source.connect(ctx.destination);
  source.start(when);
};

const ui_tick = (sim) => {
  window.requestAnimationFrame(() => {
    sim.request_animation_frame();
    ui_tick(sim);
  });
};

const audio_tick = (sim, ctx, bufferSizeInSeconds, pos) => {
  // keep track of where to queue the next buffer

  let new_pos = pos;

  if (ctx.currentTime > pos - bufferSizeInSeconds) {
    // fetch a new buffer from the wasm module
    const now = performance.now();
    let buffer = sim.request_buffer();
    console.log("wasm performance: ", performance.now() - now);

    // queue buffer to audio context
    queueBuffer(ctx, bufferSizeInSeconds, pos, buffer);

    // set the next position
    new_pos += bufferSizeInSeconds;
  }

  setTimeout(() => {
    audio_tick(sim, ctx, bufferSizeInSeconds, new_pos);
  }, 0);
};

const onLoad = (sim) => {
  document.querySelector("button#triangle").addEventListener("click", () => {
    sim.set_osc_output("triangle");
  });
  document.querySelector("button#saw").addEventListener("click", () => {
    sim.set_osc_output("saw");
  });
  document.querySelector("button#square").addEventListener("click", () => {
    sim.set_osc_output("square");
  });

  document.querySelector("button#start").addEventListener("click", () => {
    const bufferSizeInSeconds = 0.05;
    const ctx = new AudioContext();
    const pos = ctx.currentTime + bufferSizeInSeconds;

    ui_tick(sim);
    audio_tick(sim, ctx, bufferSizeInSeconds, pos);

    document.querySelector("button#start").remove();
  });
};

// load wasm
import("../pkg/index.js").then(onLoad).catch(console.error);
