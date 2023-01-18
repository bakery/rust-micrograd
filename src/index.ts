// @ts-ignore
import init, { greet } from "micrograd";
import { renderApp } from "./main";

init().then(() => {
  console.log("init wasm-pack");
  greet();
  renderApp();
});
