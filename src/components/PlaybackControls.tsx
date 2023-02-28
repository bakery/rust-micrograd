import { usePlayground } from "../context/Playground";

const PlaybackControls = () => {
  const { depths, forward, backward } = usePlayground();
  return (
    <>
      <br />
      <strong>
        CURRENT: {depths.current} ({depths.direction})
      </strong>
      <button
        disabled={depths.direction === "forward"}
        onClick={() => {
          backward();
        }}
      >
        BACKWARD
      </button>
      <button
        disabled={depths.direction === "backward"}
        onClick={() => {
          forward();
        }}
      >
        FORWARD
      </button>
      <br />
    </>
  );
};

export default PlaybackControls;
