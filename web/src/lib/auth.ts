import { cookies } from "next/headers";
import { api } from "./api";

export const auth = async () => {
  const cookie = cookies().get("auth_session");
  if (cookie) {
    return api.query(["auth.verify", cookie.value]);
  }
  return null;
};
