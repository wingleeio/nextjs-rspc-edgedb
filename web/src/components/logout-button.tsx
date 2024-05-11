"use client";

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "./ui/accordion";
import { useEffect, useState } from "react";

import { Badge } from "./ui/badge";
import { Button } from "./ui/button";
import { ChevronDown } from "lucide-react";
import { SessionWithMetadata } from "@/generated/bindings";
import { api } from "@/lib/api";
import { formatIsoDate } from "@/lib/format-iso-date";
import { useRouter } from "next/navigation";

export const LogoutButton = () => {
  const router = useRouter();
  const [sessions, setSessions] = useState<SessionWithMetadata[]>([]);
  useEffect(() => {
    api.query(["auth.getSessions"]).then((response) => {
      setSessions(response);
    });
  }, []);
  return (
    <div className="flex flex-col gap-4">
      {sessions.map((session) => (
        <div
          key={session.id}
          className="border border-solid rounded-md min-w-96 bg-muted flex relative items-center text-left"
        >
          <div className="p-4 flex justify-between bg-background flex-1 border-r border-solid rounded-md">
            <div>
              <div className="text-sm">{session.os_name ?? "Unknown"}</div>
              <div className="text-muted-foreground text-sm">
                {session.browser_name ?? "Unknown"}{" "}
                {session.browser_version ?? ""}
              </div>
              <div className="text-muted-foreground text-sm">
                {formatIsoDate(session.last_accessed_at)}
              </div>
            </div>
            <div>
              {session.is_current && (
                <Badge className="rounded-sm">This Device</Badge>
              )}
            </div>
          </div>
          <div className="text-muted-foreground p-2 h-full">
            <ChevronDown className="h-[16px] w-[16px]" />
          </div>
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
