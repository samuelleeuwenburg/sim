import init, {
  init_sim,
  allocate_u8_buffer,
  allocate_f32_buffer,
  get_u8_buffer,
  get_f32_buffer,
  sample,
  render_image,
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

    // left + right channel
    const audioBuffer = allocate_f32_buffer(bufferSize * 2);
    let bufferPos = audioCtx.currentTime;

    const audioCallback = () => {
      if (audioCtx.currentTime + bufferSizeInSeconds > bufferPos) {
        // move buffer position forward
        bufferPos += bufferSizeInSeconds;

        // load new samples from wasm and get the samples
        sample(audioBuffer, bufferSize * 2);
        const samples = get_f32_buffer(audioBuffer, bufferSize * 2);

        // create buffer
        const buffer = audioCtx.createBuffer(channels, bufferSize, sampleRate);
        buffer.copyToChannel(samples.slice(0, bufferSize), 0);
        buffer.copyToChannel(samples.slice(bufferSize), 1);

        // create source
        const source = audioCtx.createBufferSource();
        source.buffer = buffer;
        source.connect(audioCtx.destination);
        source.start(bufferPos);
      }
    };

    const canvas = document.getElementById("viewport") as HTMLCanvasElement;
    canvas.width = viewportWidth;
    canvas.height = viewportHeight;
    const ctx = canvas.getContext("2d")!;
    const renderBufferSize = viewportWidth * viewportHeight * 4;
    const renderBuffer = allocate_u8_buffer(renderBufferSize);

    const graphicsCallback = () => {
      render_image(renderBuffer, renderBufferSize);
      const buffer = get_u8_buffer(renderBuffer, renderBufferSize);
      const imageData = new ImageData(buffer, viewportWidth, viewportHeight);
      ctx.putImageData(imageData, 0, 0);

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
