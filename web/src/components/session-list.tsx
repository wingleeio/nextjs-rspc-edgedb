import { api } from "@/lib/api";
import { formatIsoDate } from "@/lib/format-iso-date";
import { ChevronDown } from "lucide-react";
import { Badge } from "./ui/badge";

export async function SessionList({ token }: { token: string }) {
  const sessions = await api.query(["auth.getSessions"], {
    context: {
      headers: {
        cookie: `auth_session=${token}`,
      },
    },
  });

  return sessions.map((session) => (
    <div key={session.id} className="border border-solid rounded-md min-w-96 bg-muted flex relative items-center text-left">
      <div className="p-4 flex justify-between bg-background flex-1 border-r border-solid rounded-md">
        <div>
          <div className="text-sm">{session.os_name ?? "Unknown"}</div>
          <div className="text-muted-foreground text-sm">
            {session.browser_name ?? "Unknown"} {session.browser_version ?? ""}
          </div>
          <div className="text-muted-foreground text-sm">{formatIsoDate(session.last_accessed_at)}</div>
        </div>
        <div>{session.is_current && <Badge className="rounded-sm">This Device</Badge>}</div>
      </div>
      <div className="text-muted-foreground p-2 h-full">
        <ChevronDown className="h-[16px] w-[16px]" />
      </div>
    </div>
  ));
}
