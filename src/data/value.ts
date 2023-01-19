export enum OpType {
  "Add",
  "Multiply",
}

export interface Value {
  id: number;
  data: number;
  children: Value[];
  op?: OpType;
}
