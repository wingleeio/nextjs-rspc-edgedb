CREATE MIGRATION m1njye5c4pfbsdvevqojh4h6copaugbomt3opr3lmqhbyf7stuusha
    ONTO m1ctgrymz66dlunxzsizaavyaaffhzow6upjzyfnzqsg5kkyfep5aq
{
  ALTER TYPE default::Session {
      CREATE REQUIRED PROPERTY last_accessed_at: std::datetime {
          SET default := (std::datetime_current());
      };
  };
};
