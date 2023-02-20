import { Node, Edge } from "reactflow";
import { Value } from "../data/value";

interface NodesAndEdgesReturns {
  nodes: Node[];
  edges: Edge[];
}

const edgeType = "smoothstep";

export const getNodesAndEdges = (state: Value[]) => {
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
          animated: true,
        });
      });

      // connect op node to original scalar node
      extendedEdges.push({
        id: `${opId}-${n.id}`,
        source: opId,
        target: `${n.id}`,
        // @ts-ignore
        edgeType,
        animated: true,
      });
    }
  });

  return {
    nodes: extendedNodes,
    edges: extendedEdges,
  };
};
