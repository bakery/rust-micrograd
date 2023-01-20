export enum OpType {
  "Add",
  "Multiply",
}

export interface Value {
  id: number;
  label: string;
  data: number;
  grad: number;
  children: Value[];
  op?: OpType;
}
