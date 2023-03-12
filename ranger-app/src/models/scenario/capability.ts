export type Capability = {
  description?: string;
  condition: string;
  vulnerabilities?: string[];
};

export type Capabilities = Record<string, Capability>;
