import "./App.css";
import Playground from "./components/Playground";
import PlaybackControls from "./components/PlaybackControls";
import { PlaygroundProvider } from "./context/Playground";

interface AppProps {
  microgradInitFailed?: boolean;
}

function App(props: AppProps) {
  const { microgradInitFailed } = props;
  return !microgradInitFailed ? (
    <div className="App">
      <PlaygroundProvider>
        <PlaybackControls />
        <Playground />
      </PlaygroundProvider>
    </div>
  ) : (
    <strong>Failed to initialize micrograd 🙆‍♂️</strong>
  );
}

export default App;
