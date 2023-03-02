import Playground from "./components/Playground";
import { PlaygroundProvider } from "./context/Playground";

interface AppProps {
  microgradInitFailed?: boolean;
}

function App(props: AppProps) {
  const { microgradInitFailed } = props;
  return !microgradInitFailed ? (
    <>
      <div className="isolate bg-white h-full">
        <div className="absolute inset-x-0 top-[-10rem] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[-20rem]">
          <svg
            className="relative left-[calc(50%-11rem)] -z-10 h-[21.1875rem] max-w-none -translate-x-1/2 rotate-[30deg] sm:left-[calc(50%-30rem)] sm:h-[42.375rem]"
            viewBox="0 0 1155 678"
          >
            <path
              fill="url(#45de2b6b-92d5-4d68-a6a0-9b9b2abad533)"
              fillOpacity=".3"
              d="M317.219 518.975L203.852 678 0 438.341l317.219 80.634 204.172-286.402c1.307 132.337 45.083 346.658 209.733 145.248C936.936 126.058 882.053-94.234 1031.02 41.331c119.18 108.451 130.68 295.337 121.53 375.223L855 299l21.173 362.054-558.954-142.079z"
            />
            <defs>
              <linearGradient
                id="45de2b6b-92d5-4d68-a6a0-9b9b2abad533"
                x1="1155.49"
                x2="-78.208"
                y1=".177"
                y2="474.645"
                gradientUnits="userSpaceOnUse"
              >
                <stop stopColor="#9089FC" />
                <stop offset={1} stopColor="#FF80B5" />
              </linearGradient>
            </defs>
          </svg>
        </div>
        <main className="h-full">
          <div className="h-full relative px-6 lg:px-8">
            <div className="h-full w-full mx-auto max-w-5xl py-32 sm:py-48 lg:py-56">
              <div className="sm:mb-8 sm:flex sm:justify-center">
                <div className="relative rounded-full py-1 px-3 text-sm leading-6 text-gray-600 ring-1 ring-gray-900/10 hover:ring-gray-900/20">
                  based on{" "}
                  <a
                    className="font-bold"
                    href="https://github.com/karpathy/micrograd"
                    target="_blank"
                  >
                    micrograd by @karpathy
                  </a>{" "}
                  <a
                    className="font-bold"
                    href="https://github.com/bakery/rust-micrograd"
                    target="_blank"
                  >
                    🛠️ source
                  </a>
                </div>
              </div>
              <h1 className="my-10 text-center text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">
                micrograd in Rust *experiment*
              </h1>
              <PlaygroundProvider>
                <div className="w-full h-5/6 rounded-2xl ring-1 ring-inset ring-gray-900/5 lg:flex lg:flex-col lg:justify-cente">
                  <div className="h-full w-full">
                    <Playground />
                  </div>
                </div>
              </PlaygroundProvider>
            </div>
            <div className="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]">
              <svg
                className="relative left-[calc(50%+3rem)] h-[21.1875rem] max-w-none -translate-x-1/2 sm:left-[calc(50%+36rem)] sm:h-[42.375rem]"
                viewBox="0 0 1155 678"
              >
                <path
                  fill="url(#ecb5b0c9-546c-4772-8c71-4d3f06d544bc)"
                  fillOpacity=".3"
                  d="M317.219 518.975L203.852 678 0 438.341l317.219 80.634 204.172-286.402c1.307 132.337 45.083 346.658 209.733 145.248C936.936 126.058 882.053-94.234 1031.02 41.331c119.18 108.451 130.68 295.337 121.53 375.223L855 299l21.173 362.054-558.954-142.079z"
                />
                <defs>
                  <linearGradient
                    id="ecb5b0c9-546c-4772-8c71-4d3f06d544bc"
                    x1="1155.49"
                    x2="-78.208"
                    y1=".177"
                    y2="474.645"
                    gradientUnits="userSpaceOnUse"
                  >
                    <stop stopColor="#9089FC" />
                    <stop offset={1} stopColor="#FF80B5" />
                  </linearGradient>
                </defs>
              </svg>
            </div>
          </div>
        </main>
      </div>
    </>
  ) : (
    <strong>Failed to initialize micrograd 🙆‍♂️</strong>
  );
}

export default App;
