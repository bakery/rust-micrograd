import { memo } from "react";
import { Handle, Position } from "reactflow";

interface ScalarNodeData {
  value: number;
}

interface ScalarNodeProps {
  data: ScalarNodeData;
  isConnectable?: boolean;
}

export default memo((props: ScalarNodeProps) => {
  const { data, isConnectable } = props;
  return (
    <>
      <Handle
        type="target"
        position={Position.Left}
        style={{ background: "#555" }}
        onConnect={(params) => console.log("handle onConnect", params)}
        isConnectable={isConnectable}
      />
      <div>
        VALUE: <strong>{data.value}</strong>
      </div>
      <Handle
        type="source"
        position={Position.Right}
        id="b"
        style={{ bottom: 10, top: "auto", background: "#555" }}
        isConnectable={isConnectable}
      />
    </>
  );
});
