import "reactflow/dist/style.css";

import ReactFlow, {
  Background,
  Node,
  Edge,
  Position,
  ConnectionLineType,
} from "reactflow";
import dagre from "dagre";
import { usePlayground } from "../context/Playground";
import { nodeTypes } from "./nodes";

const nodeWidth = 90;
const nodeHeight = 36;

const dagreGraph = new dagre.graphlib.Graph();
dagreGraph.setDefaultEdgeLabel(() => ({}));

const getLayoutedElements = (
  nodes: Node[],
  edges: Edge[],
  direction = "LR"
) => {
  const isHorizontal = direction === "LR";
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
      x: nodeWithPosition.x - nodeWidth / 2,
      y: nodeWithPosition.y - nodeHeight / 2,
    };

    return node;
  });

  return { nodes, edges };
};

const Playground = () => {
  const { nodes, edges } = usePlayground();

  const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(
    nodes,
    edges
  );

  return (
    <ReactFlow
      fitView
      connectionLineType={ConnectionLineType.SmoothStep}
      nodeTypes={nodeTypes}
      nodes={layoutedNodes}
      edges={layoutedEdges}
    >
      <Background />
    </ReactFlow>
  );
};

export default Playground;
