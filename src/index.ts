// @ts-ignore
import init, { Playground } from "micrograd";
import { renderApp } from "./main";

init().then(() => {
  console.log("init wasm-pack");
  renderApp();
  // @ts-ignore
  console.log(">>>>>>>>>>>>>> got playgtround", Playground.new().getState());
});
