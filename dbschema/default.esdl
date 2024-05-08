module default {
    abstract type Node {
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
        required first_name: string;
        required last_name: string;
        required email: string;
        required hashed_password: string;
        blogs := .<owner[is Blog];
    }

    type Blog extending Node {
        required title: string;
        required owner: User;
        multi authors: User;
    }

    type Post extending Node {
        required title: string;
        required content: string;
        required author: User;
        required blog: Blog;
        comments := .<post[is Comment];
    }

    type Comment extending Node {
        required content: string;
        required author: User;
        required post: Post;
    }
}
