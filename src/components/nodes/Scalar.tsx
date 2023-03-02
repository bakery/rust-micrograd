import { memo } from "react";
import { Handle, Position } from "reactflow";

interface ScalarNodeData {
  value: number;
  grad: number;
  label: string;
  current?: boolean;
  isComputed: boolean;
  showGradient: boolean;
}

interface ScalarNodeProps {
  data: ScalarNodeData;
  isConnectable?: boolean;
}

const prettyNumber = (value: number): number => Math.round(value * 100) / 100;

export default memo((props: ScalarNodeProps) => {
  const { data, isConnectable } = props;

  const showValue = !data.isComputed || data.current;

  return (
    <div
      className={`overflow-hidden text-center rounded-lg bg-white px-4 py-5 shadow sm:p-6 ${
        data.isComputed && showValue
          ? "afnimate__animated animate__heartBeat"
          : ""
      }`}
      style={{
        opacity: data.current ? 1.0 : 0.25,
      }}
    >
      {data.isComputed ? (
        <Handle
          type="target"
          position={Position.Left}
          style={{ background: "#555" }}
          isConnectable={isConnectable}
        />
      ) : null}
      <div className="relative">
        <div className="text-indigo-600 pl-0.5 top-0 left-0 fixed font-medium text-gray-900 text-sm">
          {data.label}
        </div>
        <dd className="mt-1 text-3xl font-semibold text-center tracking-tight text-gray-900">
          {showValue ? prettyNumber(data.value) : "?"}
        </dd>
        <dt
          style={{ opacity: data.showGradient ? 1.0 : 0.0 }}
          className="truncate text-sm font-medium text-gray-500"
        >
          {`grad: ${prettyNumber(data.grad)}`}
        </dt>
      </div>
      <Handle
        type="source"
        position={Position.Right}
        id="b"
        style={{ background: "#555" }}
        isConnectable={isConnectable}
      />
    </div>
  );
});
