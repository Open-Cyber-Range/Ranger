type GraphPoint = {
  x: Date | string | number;
  y: number;
};

type GraphDataset = {
  label: string;
  tension: number;
  borderColor: string | string[];
  backgroundColor: string | string[];
  data: GraphPoint[];
};

type GraphData = {
  datasets: GraphDataset[];
};

export type {
  GraphPoint,
  GraphDataset,
  GraphData,
};

