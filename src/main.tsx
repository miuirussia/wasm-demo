import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import * as wasm from "wasm-pkg";
import z from "zod";

const photoScema = z.object({
  albumId: z.number(),
  id: z.bigint(),
  title: z.string(),
  url: z.string(),
  thumbnailUrl: z.string(),
});

const photosSchema = z.array(photoScema);

await fetch("https://jsonplaceholder.typicode.com/photos")
  .then((response) => response.text())
  .then((json) => {
    console.time("json");
    console.log(
      "json",
      photosSchema.parse(
        JSON.parse(json, (key, value, context) => {
          if (key === "id") {
            return BigInt(context.source);
          } else {
            return value;
          }
        })
      )
    );
    console.timeEnd("json");
    console.time("wasm");
    console.log("wasm", wasm.parse(json));
    console.timeEnd("wasm");
  });

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
