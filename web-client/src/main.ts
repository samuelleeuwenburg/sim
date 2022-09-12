import init, {
  init_sim,
  sample,
  get_render_instructions,
  handle_key_down,
  handle_key_up,
} from "sim-web-client";

const channels = 2;
const sampleRate = 48000;
const bufferSize = 256;
const bufferSizeInSeconds = bufferSize / 48_000 / 2;

const viewportWidth = 400;
const viewportHeight = 400;

const startButton = document.querySelector("button#start");

const handleKeyDown = (e: KeyboardEvent) => handle_key_down(e.key);
const handleKeyUp = (e: KeyboardEvent) => handle_key_up(e.key);

startButton &&
  startButton.addEventListener("click", () => {
    const audioCtx = new AudioContext({
      latencyHint: "interactive",
      sampleRate,
    });

    let bufferPos = audioCtx.currentTime;

    const audioCallback = () => {
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
    };

    const canvas = document.getElementById("viewport") as HTMLCanvasElement;
    canvas.width = viewportWidth;
    canvas.height = viewportWidth;
    const ctx = canvas.getContext("2d")!;

    const graphicsCallback = () => {
      const json = get_render_instructions();
      const instructions = json && JSON.parse(json);

      instructions &&
        instructions.forEach((instruction: any) => {
          if (instruction === "Clear") {
            ctx.clearRect(0, 0, viewportWidth, viewportHeight);
          } else if (instruction.Rect) {
            const [x, y, w, h] = instruction.Rect;
            ctx.fillRect(x, y, w, h);
          } else if (instruction.Image) {
            const [x, y, w, h, data] = instruction.Image;
            const imageData = new ImageData(Uint8ClampedArray.from(data), w, h);
            ctx.putImageData(imageData, x, y);
          }
        });

      window.requestAnimationFrame(graphicsCallback);
    };

    window.setInterval(audioCallback, 0);
    window.requestAnimationFrame(graphicsCallback);
  });

init().then(() => {
  console.log("we have wasm!");
  init_sim(sampleRate, bufferSize, viewportWidth, viewportHeight);

  window.addEventListener("keyup", handleKeyUp);
  window.addEventListener("keydown", handleKeyDown);
});
