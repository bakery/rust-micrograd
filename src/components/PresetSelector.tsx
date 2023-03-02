import { usePlayground, PlaygroundPresets } from "../context/Playground";
import { useReactFlow } from "reactflow";

const PresetSelector = () => {
  const reactFlow = useReactFlow();
  const { loadPreset } = usePlayground();
  return (
    <>
      <select
        onChange={(e) => {
          loadPreset(parseInt(e.currentTarget.value) as PlaygroundPresets);
          setTimeout(() => {
            reactFlow.fitView();
          }, 1000);
        }}
      >
        <option>👋 Pick preset</option>
        <option value={PlaygroundPresets.BasicExpression}>Basic Example</option>
        <option value={PlaygroundPresets.Neuron}>Neuron</option>
        <option value={PlaygroundPresets.BasicMLP}>Basic MLP</option>
      </select>
    </>
  );
};

export default PresetSelector;
