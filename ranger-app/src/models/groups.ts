
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
  id: string;
  username: string;
  password?: string;
  privateKey?: string;
};
