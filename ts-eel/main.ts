import { hashTypedData } from "viem";

if (import.meta.main) {
  let content = "";

  const decoder = new TextDecoder();
  for await (const chunk of Deno.stdin.readable) {
    const text = decoder.decode(chunk);
    content += text;
  }

  console.log(hashTypedData(JSON.parse(content)));
}
