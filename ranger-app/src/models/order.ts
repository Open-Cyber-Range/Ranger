
type NewOrder = {
  name: string;
  clientId: string;
};

type Order = {
  id: string;
  createdAt: string;
  updatedAt: string;
} & NewOrder;

export type {
  NewOrder,
  Order,
};
