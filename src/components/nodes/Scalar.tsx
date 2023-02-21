import { memo } from "react";
import { Handle, Position } from "reactflow";

interface ScalarNodeData {
  value: number;
  grad: number;
  label: string;
  current?: boolean;
}

interface ScalarNodeProps {
  data: ScalarNodeData;
  isConnectable?: boolean;
}

const prettyNumber = (value: number): number => Math.round(value * 100) / 100;

export default memo((props: ScalarNodeProps) => {
  const { data, isConnectable } = props;
  return (
    <div
      style={{
        border: "solid 1px #ccc",
        padding: "10px",
        opacity: data.current ? 1.0 : 0.5,
      }}
    >
      <Handle
        type="target"
        position={Position.Left}
        style={{ background: "#555" }}
        onConnect={(params) => console.log("handle onConnect", params)}
        isConnectable={isConnectable}
      />
      <div>
        <strong>{data.label}</strong>: {prettyNumber(data.value)}
        <br />
        <strong>grad</strong>: {prettyNumber(data.grad)}
      </div>
      <Handle
        type="source"
        position={Position.Right}
        id="b"
        style={{ bottom: 10, top: "auto", background: "#555" }}
        isConnectable={isConnectable}
      />
    </div>
  );
});
