import { LogoutButton } from "@/components/logout-button";
import { Button } from "@/components/ui/button";
import { auth } from "@/lib/auth";
import Link from "next/link";

export default async function Home() {
  const session = await auth();

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {session ? (
        <div>
          <LogoutButton />
        </div>
      ) : (
        <div>
          <Link href="/auth/login">
            <Button size="sm" className="h-8 w-full gap-2">
              Login
            </Button>
          </Link>
        </div>
      )}
    </main>
  );
}
