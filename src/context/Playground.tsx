import { createContext, useContext, useState, ReactNode } from "react";
import { Node, Edge } from "reactflow";
import { Playground as PlaygroundBackend, PlaygroundPresets } from "micrograd";
import { Value } from "../data/value";
import { getNodesAndEdges } from "../lib/helpers";

export { PlaygroundPresets } from "micrograd";

interface DepthController {
  values: number[];
  current: number;
  direction: "forward" | "backward";
}

interface Playground {
  nodes: Node[];
  edges: Edge[];
  depths: DepthController;
  loadPreset: (preset: PlaygroundPresets) => void;
  forward: () => void;
  backward: () => void;
}

const PlaygroundContext = createContext<Playground>({
  nodes: [],
  edges: [],
  depths: {
    values: [],
    current: 0,
    direction: "forward",
  },
  forward: () => {
    /* noop */
  },
  backward: () => {
    /* noop */
  },
  loadPreset: () => {
    // noop
  },
});

interface PlaygroundProviderProps {
  children: ReactNode;
}

export const PlaygroundProvider = (props: PlaygroundProviderProps) => {
  const { children } = props;
  const [state, setState] = useState<Value[]>([]);
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [depths, setDepths] = useState<DepthController>({
    values: [],
    current: 0,
    direction: "forward",
  });
  const backend = PlaygroundBackend.new();

  const updateScene = (
    state: Value[],
    depths: DepthController,
    resetDepth = false
  ) => {
    const {
      nodes,
      edges,
      depths: depthsValues,
    } = getNodesAndEdges(
      state,
      resetDepth ? 0 : depths.current,
      resetDepth ? "forward" : depths.direction
    );

    setNodes(nodes);
    setEdges(edges);

    setDepths((ds) => {
      let currentDirection = ds.direction;

      // start moving backwards if we reached the end
      if (ds.direction === "forward") {
        if (depths.current >= Math.max(...depthsValues)) {
          currentDirection = "backward";
        }
      }
      return Object.assign(
        ds,
        {
          values: depthsValues,
          direction: currentDirection,
        },
        resetDepth
          ? {
              current: depthsValues[0],
              direction: "forward",
            }
          : {}
      );
    });
  };

  return (
    <PlaygroundContext.Provider
      value={{
        nodes,
        edges,
        depths,
        forward: () => {
          const newDepths = Object.assign(depths, {
            current: depths.values.find((d) => d > depths.current) || 0,
          });
          setDepths(newDepths);
          updateScene(state, newDepths);
        },
        backward: () => {
          const revs = depths.values.reverse();
          const newDepths = Object.assign(depths, {
            current: revs.find((d) => d < depths.current) || 0,
          });
          setDepths(newDepths);
          updateScene(state, newDepths);
        },
        loadPreset: (preset) => {
          const s = backend.load_preset(preset);
          setState(s);
          updateScene(s, depths, true);
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
