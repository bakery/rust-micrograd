import { memo } from "react";
import { Handle, Position } from "reactflow";
import { OpType } from "../../data/value";

interface OperationNodeData {
  operation: OpType;
}

interface OperationNodeProps {
  data: OperationNodeData;
  isConnectable?: boolean;
}

const opTypeToString = (op: string): string => {
  switch (op) {
    case "Add":
      return "+";
    case "Multiply":
      return "*";
  }
};

export default memo((props: OperationNodeProps) => {
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
        <strong>{opTypeToString(data.operation)}</strong>
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