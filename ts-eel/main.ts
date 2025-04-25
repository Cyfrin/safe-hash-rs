import { hashTypedData, hashStruct } from "viem";
import { TypedDataEncoder } from "ethers";

if (import.meta.main) {
  let content = "";

  const decoder = new TextDecoder();
  for await (const chunk of Deno.stdin.readable) {
    const text = decoder.decode(chunk);
    content += text;
  }

  const jsonIn = JSON.parse(content);

  const rawEip712 = hashTypedData(jsonIn);

  const result = {
    eip712Hash: rawEip712,
    messageHash: hashStruct({
      data: jsonIn.message,
      primaryType: jsonIn.primaryType,
      types: jsonIn.types,
    }),
    domainHash: TypedDataEncoder.hashDomain(jsonIn.domain),
  };

  console.log(JSON.stringify(result));
}
