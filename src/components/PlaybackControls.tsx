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
        onClick={() => {
          backward();
        }}
      >
        BACKWARD
      </button>
      <button
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
