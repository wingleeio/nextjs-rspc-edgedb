CREATE MIGRATION m1mxvnntu7bolxeccw3dvadj5w33fm4gllltqal5mhyaiwqhgbkpoq
    ONTO initial
{
  CREATE ABSTRACT TYPE default::Node {
      CREATE REQUIRED PROPERTY created_at: std::datetime {
          SET default := (std::datetime_current());
          SET readonly := true;
      };
      CREATE PROPERTY updated_at: std::datetime {
          SET default := (std::datetime_current());
          CREATE REWRITE
              INSERT 
              USING (std::datetime_of_statement());
          CREATE REWRITE
              UPDATE 
              USING (std::datetime_of_statement());
      };
  };
  CREATE TYPE default::Blog EXTENDING default::Node {
      CREATE REQUIRED PROPERTY title: std::str;
  };
  CREATE TYPE default::User EXTENDING default::Node {
      CREATE REQUIRED PROPERTY email: std::str;
      CREATE REQUIRED PROPERTY first_name: std::str;
      CREATE REQUIRED PROPERTY hashed_password: std::str;
      CREATE REQUIRED PROPERTY last_name: std::str;
  };
  ALTER TYPE default::Blog {
      CREATE MULTI LINK authors: default::User;
  };
  CREATE TYPE default::Comment EXTENDING default::Node {
      CREATE REQUIRED LINK blog: default::Blog;
      CREATE REQUIRED LINK author: default::User;
      CREATE REQUIRED PROPERTY content: std::str;
  };
  ALTER TYPE default::Blog {
      CREATE LINK comments := (.<blog[IS default::Comment]);
      CREATE REQUIRED LINK owner: default::User;
  };
  ALTER TYPE default::User {
      CREATE LINK blogs := (.<owner[IS default::Blog]);
  };
  CREATE SCALAR TYPE default::PostStatus EXTENDING enum<Draft, Private, Published>;
  CREATE TYPE default::Post EXTENDING default::Node {
      CREATE REQUIRED LINK blog: default::Blog;
      CREATE REQUIRED LINK author: default::User;
      CREATE REQUIRED PROPERTY content: std::str;
      CREATE REQUIRED PROPERTY status: default::PostStatus;
      CREATE REQUIRED PROPERTY title: std::str;
  };
  ALTER TYPE default::Blog {
      CREATE LINK posts := (.<blog[IS default::Post]);
  };
  ALTER TYPE default::Comment {
      CREATE REQUIRED LINK post: default::Post;
  };
  ALTER TYPE default::Post {
      CREATE LINK comments := (.<post[IS default::Comment]);
  };
  ALTER TYPE default::User {
      CREATE LINK posts := (.<author[IS default::Post]);
  };
};
