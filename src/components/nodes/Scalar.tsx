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
      className={
        data.isComputed && showValue
          ? "afnimate__animated animate__heartBeat"
          : ""
      }
      style={{
        border: "solid 1px #ccc",
        padding: "10px",
        opacity: data.current ? 1.0 : 0.25,
      }}
    >
      {data.isComputed ? (
        <Handle
          type="target"
          position={Position.Left}
          style={{ background: "#555" }}
          onConnect={(params) => console.log("handle onConnect", params)}
          isConnectable={isConnectable}
        />
      ) : null}
      <div>
        {/* <strong>{`${data.label}${data.isComputed ? "[comp]" : ""}`}</strong>:{" "} */}
        {showValue ? prettyNumber(data.value) : "?"}
        {data.showGradient ? (
          <>
            <br />
            <strong>grad</strong>: {prettyNumber(data.grad)}
          </>
        ) : null}
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
