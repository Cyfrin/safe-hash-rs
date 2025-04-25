import { hashTypedData, domainSeparator, hashStruct } from "viem";
import { TypedDataEncoder } from "ethers";

const formSafeMessage = (messageEIP712Hash: string): any => {
  return {
    types: {
      SafeMessage: [
        {
          name: "message",
          type: "bytes",
        },
      ],
      EIP712Domain: [
        {
          name: "chainId",
          type: "uint256",
        },
        {
          name: "verifyingContract",
          type: "address",
        },
      ],
    },
    domain: {
      chainId: "0x1",
      verifyingContract: "0x35ea56fd9ead2567f339eb9564b6940b9dd5653f",
    },
    primaryType: "SafeMessage",
    message: {
      message: messageEIP712Hash,
    },
  };
};

if (import.meta.main) {
  let content = "";

  const decoder = new TextDecoder();
  for await (const chunk of Deno.stdin.readable) {
    const text = decoder.decode(chunk);
    content += text;
  }

  const jsonIn = JSON.parse(content);

  const rawEip712 = hashTypedData(jsonIn);
  const safeMsg = formSafeMessage(rawEip712);

  const result = {
    // Raw
    eip712Hash: rawEip712,
    domainSeparator: domainSeparator({
      domain: jsonIn.domain,
    }),
    messageHash: hashStruct({
      data: jsonIn.message,
      primaryType: jsonIn.primaryType,
      types: jsonIn.types,
    }),
    domainHash: TypedDataEncoder.hashDomain(jsonIn.domain),

    // Safe
    safeEip712Hash: hashTypedData(safeMsg),
    safeDomainSeparator: domainSeparator({
      domain: safeMsg.domain,
    }),
    safeMessageHash: hashStruct({
      data: safeMsg.message,
      primaryType: safeMsg.primaryType,
      types: safeMsg.types,
    }),
    safeDomainHash: TypedDataEncoder.hashDomain(safeMsg.domain),
  };

  console.log(JSON.stringify(result));
}
