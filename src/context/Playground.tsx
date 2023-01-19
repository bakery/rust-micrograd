import {
  createContext,
  useContext,
  useEffect,
  useState,
  ReactNode,
} from "react";
import { Node, Edge } from "reactflow";
import { Value, OpType } from "../data/value";
import { Playground as PlaygroundBackend } from "micrograd";

interface Playground {
  values: Value[];
  nodes: Node[];
  edges: Edge[];
}

const PlaygroundContext = createContext<Playground>({
  values: [],
  nodes: [],
  edges: [],
});

interface PlaygroundProviderProps {
  children: ReactNode;
}

const position = { x: 0, y: 0 };

export const PlaygroundProvider = (props: PlaygroundProviderProps) => {
  const { children } = props;
  const [values, setValues] = useState<Value[]>([]);
  const [nodes, setNodes] = useState<Node[]>([
    {
      id: "1",
      position,
      type: "scalar",
      data: {
        value: -3.0,
      },
    },
    {
      id: "2",
      position,
      type: "scalar",
      data: {
        value: 2.0,
      },
    },
    {
      id: "3",
      position,
      type: "scalar",
      data: {
        value: -6.0,
      },
    },
    {
      id: "4",
      position,
      type: "operation",
      data: {
        operation: OpType.Multiply,
      },
    },
  ]);
  const [edges, setEdges] = useState<Edge[]>([
    { id: "1-4", source: "1", target: "4" },
    { id: "2-4", source: "2", target: "4" },
    { id: "4-3", source: "4", target: "3" },
  ]);

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
          value: n.data,
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
          });
        });

        // connect op node to original scalar node
        extendedEdges.push({
          id: `${opId}-${n.id}`,
          source: opId,
          target: `${n.id}`,
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
    <PlaygroundContext.Provider value={{ values, nodes, edges }}>
      {children}
    </PlaygroundContext.Provider>
  );
};

export const usePlayground = (): Playground => {
  return useContext(PlaygroundContext);
};
