
export type AdGroup = {
  id: string;
  name: string;
  description?: string;
};

export type AdUser = {
  id: string;
  vmId: string;
  username?: string;
  accounts: Account[];
};

export type Account = {
  username: string;
  password?: string;
  privateKey?: string;
};
