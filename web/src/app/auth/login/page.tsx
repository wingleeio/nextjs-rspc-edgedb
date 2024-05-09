import { auth } from "@/lib/auth";
import { redirect } from "next/navigation";
import LoginForm from "./login-form";

export default async function Login() {
  const session = await auth();

  if (session) {
    redirect("/");
  }

  return <LoginForm />;
}
