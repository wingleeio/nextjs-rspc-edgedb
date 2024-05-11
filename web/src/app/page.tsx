import { LogoutButton } from "@/components/logout-button";
import { SessionList } from "@/components/session-list";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { auth } from "@/lib/auth";
import Link from "next/link";
import { Suspense } from "react";

export default async function Home() {
  const session = await auth();

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {session ? (
        <div className="flex flex-col gap-4">
          <Suspense
            fallback={
              <div className="border border-solid rounded-md min-w-96 p-4">
                <div className="space-y-[2px]">
                  <Skeleton className="h-[18px] w-[150px]" />
                  <Skeleton className="h-[18px] w-[200px]" />
                  <Skeleton className="h-[18px] w-[200px]" />
                </div>
              </div>
            }
          >
            <SessionList token={session.id} />
          </Suspense>
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
