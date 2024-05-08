module default {
    abstract type Node {
        id_str := <str>.id;
        required created_at: datetime {
            default := datetime_current();
            readonly := true;
        }
        updated_at: datetime {
            default := datetime_current();
            rewrite insert using (datetime_of_statement());
            rewrite update using (datetime_of_statement());
        }
    }

    type User extending Node {
        required first_name: str;
        required last_name: str;
        required email: str;
        required hashed_password: str;
        blogs := .<owner[is Blog];
        posts := .<author[is Post];
        sessions := .<user[is Session];
    }

    type Session extending Node {
        required user: User;
        required expires_at: datetime {
            rewrite insert using (datetime_of_statement() + <duration>'1 week');
        }
    }

    type Blog extending Node {
        required title: str;
        required owner: User;
        multi authors: User;
        posts := .<blog[is Post];
        comments := .<blog[is Comment];
    }

    scalar type PostStatus extending enum<Draft, Private, Published>;

    type Post extending Node {
        required title: str;
        required content: str;
        required author: User;
        required blog: Blog;
        required status: PostStatus;
        comments := .<post[is Comment];
    }

    type Comment extending Node {
        required content: str;
        required author: User;
        required post: Post;
        required blog: Blog;
    }
}
