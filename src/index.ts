// @ts-ignore
import init from "micrograd";
import { renderApp } from "./main";

init()
  .then(() => {
    console.log("init wasm-pack");
    renderApp({
      microgradInitFailed: false,
    });
  })
  .catch((err) => {
    console.error(err);
    renderApp({
      microgradInitFailed: true,
    });
  });
