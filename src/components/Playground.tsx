import "reactflow/dist/style.css";

import ReactFlow, { Background, Node, Edge, Position } from "reactflow";
import dagre from "dagre";
import { usePlayground, PlaygroundPresets } from "../context/Playground";
import { nodeTypes } from "./nodes";

const nodeWidth = 90;
const nodeHeight = 36;

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const Playground = () => {
  const { nodes, edges, loadPreset } = usePlayground();
  return (
    <>
      <select
        onChange={(e) => {
          loadPreset(parseInt(e.currentTarget.value) as PlaygroundPresets);
        }}
      >
        <option>Pick preset</option>
        <option value={PlaygroundPresets.BasicExpression}>Basic Example</option>
        <option value={PlaygroundPresets.Neuron}>Neuron</option>
        <option value={PlaygroundPresets.BasicMLP}>Basic MLP</option>
      </select>
      <ReactFlow fitView nodeTypes={nodeTypes} nodes={nodes} edges={edges}>
        <Background />
      </ReactFlow>
    </>
  );
};

export default Playground;
