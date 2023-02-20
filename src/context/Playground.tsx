import { createContext, useContext, useState, ReactNode } from "react";
import { Node, Edge } from "reactflow";
import { Playground as PlaygroundBackend, PlaygroundPresets } from "micrograd";
import { getNodesAndEdges } from "../lib/helpers";

export { PlaygroundPresets } from "micrograd";

interface Playground {
  nodes: Node[];
  edges: Edge[];
  loadPreset: (preset: PlaygroundPresets) => void;
}

const PlaygroundContext = createContext<Playground>({
  nodes: [],
  edges: [],
  loadPreset: () => {
    // noop
  },
});

interface PlaygroundProviderProps {
  children: ReactNode;
}

export const PlaygroundProvider = (props: PlaygroundProviderProps) => {
  const { children } = props;
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const backend = PlaygroundBackend.new();

  return (
    <PlaygroundContext.Provider
      value={{
        nodes,
        edges,
        loadPreset: (preset) => {
          const { nodes, edges } = getNodesAndEdges(
            backend.load_preset(preset)
          );

          setNodes(nodes);
          setEdges(edges);
        },
      }}
    >
      {children}
    </PlaygroundContext.Provider>
  );
};

export const usePlayground = (): Playground => {
  return useContext(PlaygroundContext);
};
