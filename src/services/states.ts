import { atom } from "recoil";
import { ApiType } from "./types";

export const atomLogData = atom<ApiType.Registry[]>({
  key: "atomLogData",
  default: [],
});

export const atomUpdateState = atom<boolean>({
  key: "atomUpdateState",
  default: false,
});
