import axios, { AxiosInstance } from "axios";
import { ApiType } from "./types";

let axiosIns: AxiosInstance = null!;
let server = "";
let secret = "";

export async function getAxios(force: boolean = false) {
  if (axiosIns && !force) return axiosIns;

  axiosIns = axios.create({
    baseURL: `http://${server}`,
    headers: secret ? { Authorization: `Bearer ${secret}` } : {},
  });
  axiosIns.interceptors.response.use((r) => r.data);
  return axiosIns;
}

export async function getVersion() {
  const instance = await getAxios();
  return instance.get("/version") as Promise<{
    premium: boolean;
    version: string;
  }>;
}
