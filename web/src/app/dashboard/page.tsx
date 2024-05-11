import { api } from "@/lib/api";
import { auth } from "@/lib/auth";
import { redirect } from "next/navigation";

export default async function Dashboard() {
  const session = await auth();

  if (!session) {
    redirect("/auth/login");
  }

  const blogs = await api.query(["blogs.getMyBlogs"], {
    context: {
      headers: {
        cookie: `auth_session=${session.id}`,
      },
    },
  });

  console.log(blogs);

  return (
    <div>
      <h1 className="text-xl font-medium">My Blogs</h1>
    </div>
  );
}
