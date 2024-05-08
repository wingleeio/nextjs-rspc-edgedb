CREATE MIGRATION m1w4keia4xr74sn34zksutjfkkjlbds3buy7hj7mgqawivtdxhv7ha
    ONTO m1gkqwjv3d2cmzdw2l2432ckiflcglfkw4eb2ssqaaq7qagdfztu7q
{
  ALTER TYPE default::Session {
      ALTER PROPERTY expires_at {
          CREATE REWRITE
              INSERT 
              USING ((std::datetime_of_statement() + <std::duration>'1 week'));
      };
  };
};
