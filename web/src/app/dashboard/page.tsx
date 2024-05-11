import { auth } from "@/lib/auth";
import { redirect } from "next/navigation";

export default async function Dashboard() {
  const session = auth();

  if (!session) {
    redirect("/auth/login");
  }

  return (
    <div>
      <h1></h1>
    </div>
  );
}
