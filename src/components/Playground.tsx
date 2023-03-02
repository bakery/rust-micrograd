import "reactflow/dist/style.css";

import ReactFlow, { Background, Panel } from "reactflow";
import dagre from "dagre";
import { usePlayground } from "../context/Playground";
import PlaybackControls from "./PlaybackControls";
import PresetSelector from "./PresetSelector";
import { nodeTypes } from "./nodes";

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const Playground = () => {
  const { nodes, edges } = usePlayground();
  return (
    <ReactFlow fitView nodeTypes={nodeTypes} nodes={nodes} edges={edges}>
      <Panel position="top-right">
        <PresetSelector />
      </Panel>
      {nodes.length !== 0 ? (
        <Panel position="top-left">
          <PlaybackControls />
        </Panel>
      ) : null}
      <Background />
    </ReactFlow>
  );
};

export default Playground;
