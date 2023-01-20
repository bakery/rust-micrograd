import {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
} from "react";
import { Node, Edge } from "reactflow";
import { Value } from "../data/value";
import { Playground as PlaygroundBackend } from "micrograd";

const edgeType = "smoothstep";

interface Playground {
  nodes: Node[];
  edges: Edge[];
}

const PlaygroundContext = createContext<Playground>({
  nodes: [],
  edges: [],
});

interface PlaygroundProviderProps {
  children: ReactNode;
}

export const PlaygroundProvider = (props: PlaygroundProviderProps) => {
  const { children } = props;
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  useEffect(() => {
    const backend = PlaygroundBackend.new();

    const getNodesAndEdges = (
      root: Value
    ): { nodes: Value[]; edges: Value[][] } => {
      let nodes: Value[] = [];
      let edges: Value[][] = [];

      const isNodeIn = (value: Value) => nodes.find((n) => n.id === value.id);

      const processValue = (v: Value) => {
        if (isNodeIn(v)) {
          return;
        }

        nodes.push(v);

        v.children.forEach((c) => {
          edges.push([c, v]);
          processValue(c);
        });
      };

      processValue(root);

      return {
        nodes,
        edges,
      };
    };

    const state = backend.get_state() as Value[];
    const { nodes, edges } = getNodesAndEdges(state[0]);

    const mapEdge = ([from, to]: [Value, Value]) => ({
      id: `${from.id}-${to.id}`,
      source: `${from.id}`,
      target: `${to.id}`,
    });

    const extendedNodes: Node[] = [];
    let extendedEdges: Edge[] = [...edges.map((e) => mapEdge([e[0], e[1]]))];

    nodes.forEach((n) => {
      extendedNodes.push({
        id: `${n.id}`,
        position: { x: 0, y: 0 },
        type: "scalar",
        data: {
          label: n.label,
          value: n.data,
          grad: n.grad,
        },
      });

      if (n.op) {
        const opId = `op-for-${n.id}`;
        extendedNodes.push({
          id: opId,
          position: { x: 0, y: 0 },
          type: "operation",
          data: {
            operation: n.op,
          },
        });

        // remove existing edges to **n**
        extendedEdges = extendedEdges.filter((e) => e.target !== `${n.id}`);

        // recreate edges to point to kids
        n.children.forEach((c) => {
          extendedEdges.push({
            id: `${c.id}-${opId}`,
            source: `${c.id}`,
            target: opId,
            edgeType,
            animated: true,
          });
        });

        // connect op node to original scalar node
        extendedEdges.push({
          id: `${opId}-${n.id}`,
          source: opId,
          target: `${n.id}`,
          edgeType,
          animated: true,
        });
      }
    });

    setNodes(extendedNodes);
    setEdges(extendedEdges);

    console.log(
      "state is",
      state,
      "traced to",
      getNodesAndEdges(state[0]),
      extendedNodes,
      extendedEdges
    );
  }, []);

  return (
    <PlaygroundContext.Provider value={{ nodes, edges }}>
      {children}
    </PlaygroundContext.Provider>
  );
};

export const usePlayground = (): Playground => {
  return useContext(PlaygroundContext);
};
