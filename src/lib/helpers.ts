import { Node, Edge, ConnectionLineType, Position } from "reactflow";
import dagre from "dagre";
import { Value } from "../data/value";

interface NodesAndEdgesReturns {
  nodes: Node[];
  edges: Edge[];
  depths: number[];
}

const edgeType = ConnectionLineType.Straight; // "smoothstep";

export const getNodesAndEdges = (
  state: Value[],
  currentDepth: number,
  direction: "forward" | "backward"
): NodesAndEdgesReturns => {
  const _getNodesAndEdges = (
    values: Value[]
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
        const child = values.find((cv) => cv.id === c);

        if (!child) {
          return;
        }

        edges.push([child, v]);
        processValue(child);
      });
    };

    values.forEach((v) => processValue(v));

    return {
      nodes,
      edges,
    };
  };

  const { nodes, edges } = _getNodesAndEdges(state);

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
        label: n.label.length < 5 ? n.label : "...",
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
      n.children.forEach((childId) => {
        extendedEdges.push({
          id: `${childId}-${opId}`,
          source: `${childId}`,
          target: opId,
          // @ts-ignore
          edgeType,
          animated: false,
        });
      });

      // connect op node to original scalar node
      extendedEdges.push({
        id: `${opId}-${n.id}`,
        source: opId,
        target: `${n.id}`,
        // @ts-ignore
        edgeType,
        animated: false,
      });
    }
  });

  const {
    edges: es,
    nodes: ns,
    depths,
  } = getLayoutedElements(extendedNodes, extendedEdges);

  const depthToUse =
    depths.length !== 0 && currentDepth === 0 ? depths[0] : currentDepth;

  return {
    edges: es.map((e) => e),
    nodes: ns.map((n) => {
      // if this is not current depth

      console.log(">>>>>>>>>>>>>> checking on node", n);

      if (n.position.x === depthToUse) {
        return Object.assign(n, {
          data: Object.assign(n.data, { current: true }),
        });
      }

      return n;
    }),
    depths,
  };
};

const nodeWidth = 90;
const nodeHeight = 36;

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const getLayoutedElements = (
  nodes: Node[],
  edges: Edge[],
  direction = "LR"
): NodesAndEdgesReturns => {
  const isHorizontal = direction === "LR";
  let depths = new Set();

  dagreGraph.setGraph({ rankdir: direction });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  nodes.forEach((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);

    node.targetPosition = isHorizontal ? Position.Left : Position.Top;
    node.sourcePosition = isHorizontal ? Position.Right : Position.Bottom;

    // We are shifting the dagre node position (anchor=center center) to the top left
    // so it matches the React Flow node anchor point (top left).
    node.position = {
      x: nodeWithPosition.x, // - nodeWidth / 2,
      y: nodeWithPosition.y, // - nodeHeight / 2,
    };

    depths.add(nodeWithPosition.x);

    return node;
  });

  return {
    nodes,
    edges,
    depths: ([...depths] as number[]).sort((a: number, b: number) => a - b),
  };
};
