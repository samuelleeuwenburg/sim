import init, { init_sim, sample } from "sim-web-client";

const channels = 2;
const sampleRate = 48000;
const bufferSize = 256;
const bufferSizeInSeconds = bufferSize / 48_000 / 2;

const startButton = document.querySelector("button#start");

startButton &&
  startButton.addEventListener("click", () => {
    const audioCtx = new AudioContext({
      latencyHint: "interactive",
      sampleRate,
    });

    let bufferPos = audioCtx.currentTime;

    function audioCallback() {
      if (audioCtx.currentTime + bufferSizeInSeconds > bufferPos) {
        // move buffer position forward
        bufferPos += bufferSizeInSeconds;

        // get the samples
        const samples = sample();

        // create buffer
        const buffer = audioCtx.createBuffer(channels, bufferSize, sampleRate);
        buffer.copyToChannel(samples.slice(0, bufferSize / 2), 0);
        buffer.copyToChannel(samples.slice(bufferSize / 2), 1);

        // create source
        const source = audioCtx.createBufferSource();
        source.buffer = buffer;
        source.connect(audioCtx.destination);
        source.start(bufferPos);
      }
    }

    window.setInterval(audioCallback, 0);
  });

init().then(() => {
  console.log("we have wasmwasm!");
  init_sim(sampleRate, bufferSize);
});
