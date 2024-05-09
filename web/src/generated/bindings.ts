// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "auth.logout", input: never, result: null } | 
        { key: "auth.verify", input: string, result: Session | null } | 
        { key: "version", input: never, result: string },
    mutations: 
        { key: "auth.login", input: LoginArgs, result: null } | 
        { key: "auth.register", input: RegisterArgs, result: null },
    subscriptions: never
};

export type LoginArgs = { email: string; password: string }

export type RegisterArgs = { email: string; first_name: string; last_name: string; password: string }

export type Session = { id: string; expires_at: string; user: SessionUser }

export type SessionUser = { id: string; email: string; first_name: string; last_name: string }
