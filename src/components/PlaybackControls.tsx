import { usePlayground } from "../context/Playground";

const PlaybackControls = () => {
  const { depths, forward, backward } = usePlayground();
  return (
    <>
      {/* <strong>
        CURRENT: {depths.current} ({depths.direction})
      </strong> */}
      {depths.direction !== "forward" ? (
        <button
          onClick={() => {
            backward();
          }}
          type="button"
          className="ml-6 rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
        >
          ◀️
        </button>
      ) : null}

      {depths.direction !== "backward" ? (
        <button
          onClick={() => {
            forward();
          }}
          type="button"
          className="ml-6 rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
        >
          ▶️
        </button>
      ) : null}
    </>
  );
};

export default PlaybackControls;
