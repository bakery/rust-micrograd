import { memo } from "react";
import { Handle, Position } from "reactflow";
import { OpType } from "../../data/value";

interface OperationNodeData {
  operation: OpType;
  current: boolean;
}

interface OperationNodeProps {
  data: OperationNodeData;
  isConnectable?: boolean;
}

const opTypeToString = (op: string | { Pow: number }): string => {
  console.log(">>>>>>>>>>>>>>>> op iz", op);
  switch (op) {
    case "Add":
      return "+";
    case "Multiply":
      return "*";
    case "Tanh":
      return "tanh";
  }

  if (typeof op.Pow !== "undefined") {
    return "^2";
  }

  return "n/a";
};

export default memo((props: OperationNodeProps) => {
  const { data, isConnectable } = props;
  return (
    <div
      className={
        data.current
          ? "animate__animated animate__heartBeat animate__infinite	infinite animate__delay-2s"
          : ""
      }
      style={{
        backgroundColor: "yellow",
        border: "solid 1px #ccc",
        padding: "5px 10px",
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
        <strong>{opTypeToString(data.operation)}</strong>
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
