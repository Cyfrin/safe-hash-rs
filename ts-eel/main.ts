import { hashTypedData, domainSeparator, hashStruct } from "viem";
import { TypedDataEncoder } from "ethers";

if (import.meta.main) {
  let content = "";

  const decoder = new TextDecoder();
  for await (const chunk of Deno.stdin.readable) {
    const text = decoder.decode(chunk);
    content += text;
  }

  const jsonIn = JSON.parse(content);

  const result = {
    eip712Hash: hashTypedData(jsonIn),
    domainSeparator: domainSeparator({
      domain: jsonIn.domain,
    }),
    messageHash: hashStruct({
      data: jsonIn.message,
      primaryType: jsonIn.primaryType,
      types: jsonIn.types,
    }),
    domainHash: TypedDataEncoder.hashDomain(jsonIn.domain),
  };

  console.log(JSON.stringify(result));
}
