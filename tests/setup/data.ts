// Define enums and structs for borsh serialization
export enum AssetType {
  SplTokenLegacy = 0,
  SplToken2022 = 1,
  StandardNft = 2,
}

export class LaunchConfig {
  asset_type: number;
  name: string;
  symbol: string;
  decimals: number;
  total_supply: bigint;
  metadata_uri: string;
  creator: Uint8Array;
  is_mutable: number;

  constructor(fields: {
    asset_type: number;
    name: string;
    symbol: string;
    decimals: number;
    total_supply: bigint;
    metadata_uri: string;
    creator: Uint8Array;
    is_mutable: number;
  }) {
    Object.assign(this, fields);
  }
}

export const LaunchConfigSchema = new Map([
  [
    LaunchConfig,
    {
      kind: "struct",
      fields: [
        ["asset_type", "u8"],
        ["name", "string"],
        ["symbol", "string"],
        ["decimals", "u8"],
        ["total_supply", "u64"],
        ["metadata_uri", "string"],
        ["creator", [32]],
        ["is_mutable", "u8"],
      ],
    },
  ],
]);