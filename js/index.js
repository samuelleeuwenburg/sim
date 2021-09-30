const onLoad = (_sim) => {
  console.log("sim loaded");
};

document.querySelector("button#start").addEventListener("click", () => {
  import("../pkg/index.js").then(onLoad).catch(console.error);
  document.querySelector("button#start").remove();
});
