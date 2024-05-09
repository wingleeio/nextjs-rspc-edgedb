"use client";

import { api } from "@/lib/api";
import { useRouter } from "next/navigation";
import { Button } from "./ui/button";

export const LogoutButton = () => {
  const router = useRouter();
  return (
    <Button
      size="sm"
      className="h-8 w-full gap-2"
      onClick={() => {
        api.query(["auth.logout"]).then(() => {
          router.refresh();
        });
      }}
    >
      Logout
    </Button>
  );
};
