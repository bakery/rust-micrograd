import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

export const renderApp = ({
  microgradInitFailed,
}: {
  microgradInitFailed?: boolean;
}) => {
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App microgradInitFailed={microgradInitFailed} />
    </React.StrictMode>
  );
};
