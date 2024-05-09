CREATE MIGRATION m1ctgrymz66dlunxzsizaavyaaffhzow6upjzyfnzqsg5kkyfep5aq
    ONTO m1cslzg5o2jwwjvaojfjupufe5k6g5u73umxfuvjwv5ynmlno2kb3q
{
  ALTER TYPE default::Session {
      ALTER PROPERTY expires_at {
          DROP REWRITE
              INSERT ;
          };
      };
  ALTER TYPE default::Session {
      ALTER PROPERTY expires_at {
          CREATE REWRITE
              INSERT 
              USING ((std::datetime_of_statement() + <std::duration>'168 hours'));
      };
  };
};
