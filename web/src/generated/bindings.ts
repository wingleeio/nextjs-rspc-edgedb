// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "auth.getSessions", input: never, result: SessionWithMetadata[] } | 
        { key: "auth.logout", input: never, result: null } | 
        { key: "auth.verify", input: string, result: Session | null } | 
        { key: "blogs.getMyBlogs", input: never, result: Blog[] } | 
        { key: "version", input: never, result: string },
    mutations: 
        { key: "auth.login", input: LoginArgs, result: null } | 
        { key: "auth.register", input: RegisterArgs, result: null },
    subscriptions: never
};

export type Blog = { id: string; title: string }

export type LoginArgs = { email: string; password: string }

export type RegisterArgs = { email: string; first_name: string; last_name: string; password: string }

export type Session = { id: string; expires_at: string; last_accessed_at: string; user: SessionUser }

export type SessionUser = { id: string; email: string; first_name: string; last_name: string }

export type SessionWithMetadata = { id: string; expires_at: string; last_accessed_at: string; os_name: string | null; os_version: string | null; browser_name: string | null; browser_version: string | null; is_current: boolean }
