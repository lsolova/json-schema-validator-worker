import * as esbuild from "esbuild";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

let ctx = await esbuild.context({
  entryPoints: [],
  outdir: "dist",
  bundle: true,
  format: "esm",
});

let { host, port } = await ctx.serve({
  servedir: __dirname,
  onRequest: (args) => {
    const pathname = new URL(`http://localhost${args.path}`).pathname;

    // Serve files from wasm/pkg directory
    if (pathname.startsWith("/")) {
      const wasmFile = path.join(__dirname, pathname.slice(1));
      console.log(`Request for ${pathname} -> ${wasmFile}`);

      if (fs.existsSync(wasmFile) && fs.statSync(wasmFile).isFile()) {
        const content = fs.readFileSync(wasmFile);
        args.responseHeaders = {
          ...args.responseHeaders,
          "Content-Type":
            pathname.endsWith(".wasm")
              ? "application/wasm"
              : pathname.endsWith(".js")
                ? "application/javascript"
                : pathname.endsWith(".d.ts")
                  ? "text/plain"
                  : pathname.endsWith(".schema.json")
                    ? "application/schema+json"
                    : "application/octet-stream",
        };
        return {
          status: 200,
          body: content,
          headers: args.responseHeaders,
        };
      }
    }
  },
});

console.log(`Server is running at http://${host}:${port}`);
