"use client";

import { SessionWithMetadata } from "@/generated/bindings";
import { api } from "@/lib/api";
import { formatIsoDate } from "@/lib/format-iso-date";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Badge } from "./ui/badge";
import { Button } from "./ui/button";

export const LogoutButton = () => {
  const router = useRouter();
  const [sessions, setSessions] = useState<SessionWithMetadata[]>([]);
  console.log(sessions);
  useEffect(() => {
    api.query(["auth.getSessions"]).then((response) => {
      setSessions(response);
    });
  }, []);
  return (
    <div className="flex flex-col gap-4">
      {sessions.map((session) => (
        <div key={session.id} className="p-4 border border-solid rounded-md min-w-96 flex justify-between">
          <div>
            <div className="text-sm">{session.os_name ?? "Unknown"}</div>
            <div className="text-muted-foreground text-sm">
              {session.browser_name ?? "Unknown"} {session.browser_version ?? ""}
            </div>
            <div className="text-muted-foreground text-sm">{formatIsoDate(session.last_accessed_at)}</div>
          </div>
          <div>{session.is_current && <Badge className="rounded-sm">This Device</Badge>}</div>
        </div>
      ))}
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
    </div>
  );
};
