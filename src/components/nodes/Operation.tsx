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
  // console.log(">>>>>>>>>>>>>>>> op iz", op);
  switch (op) {
    case "Add":
      return "➕";
    case "Multiply":
      return "✖️";
    case "Tanh":
      return "tanh";
  }

  if (typeof op.Pow !== "undefined") {
    return `^${op.Pow}`;
  }

  return "n/a";
};

export default memo((props: OperationNodeProps) => {
  const { data, isConnectable } = props;
  return (
    <div
      className={`overflow-hidden text-center rounded-full ring-2 ring-gray-300 bg-white px-3 py-2  shadow ${
        data.current
          ? "animate__animated animate__heartBeat animate__infinite	infinite animate__delay-2s"
          : ""
      }`}
      style={{ opacity: data.current ? 1.0 : 0.25 }}
    >
      <Handle
        type="target"
        position={Position.Left}
        style={{ background: "#555" }}
        isConnectable={isConnectable}
      />
      <dd className="mt-1 text-3xl font-semibold tracking-tight text-gray-900">
        {opTypeToString(data.operation)}
      </dd>
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
